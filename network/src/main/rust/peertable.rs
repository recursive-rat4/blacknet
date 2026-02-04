/*
 * Copyright (c) 2025-2026 Pavel Vasin
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use crate::endpoint::Endpoint;
use crate::settings::Settings;
use blacknet_compat::{Mode, XDGDirectories};
use blacknet_crypto::random::{
    Distribution, FAST_RNG, FastRNG, Float01Distribution, UniformIntDistribution,
};
use blacknet_io::file::replace;
use blacknet_kernel::blake2b::Hash;
use blacknet_log::{LogManager, Logger, debug, error, info, warn};
use blacknet_serialization::format::{from_read, to_write};
use blacknet_time::{Milliseconds, SystemClock};
use core::cmp::min;
use core::error::Error;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, ErrorKind, Read, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};

const MAX_SIZE: usize = 8192;
const FILE_VERSION: u32 = 5;
const FILE_NAME: &str = "peers.dat";

pub struct PeerTable {
    logger: Logger,
    settings: Arc<Settings>,
    data_dir: PathBuf,
    peers: RwLock<HashMap<Endpoint, Entry>>,
}

impl PeerTable {
    pub fn new(
        mode: &Mode,
        dirs: &XDGDirectories,
        log_manager: &LogManager,
        settings: Arc<Settings>,
    ) -> Result<Arc<Self>, Box<dyn Error>> {
        let peer_table = Self {
            logger: log_manager.logger("PeerTable")?,
            settings,
            data_dir: dirs.data().to_owned(),
            peers: RwLock::new(HashMap::with_capacity(MAX_SIZE)),
        };
        match peer_table.load() {
            Ok(()) => {
                if !peer_table.is_empty() {
                    info!(peer_table.logger, "Loaded {} peers", peer_table.len());
                }
            }
            Err(err) => {
                warn!(peer_table.logger, "{err}");
            }
        }
        if peer_table.len() < 128 {
            let added = peer_table.add(Self::builtin_peers(mode));
            if added > 0 {
                info!(peer_table.logger, "Added {added} built-in peers");
            }
        }
        Ok(Arc::new(peer_table))
    }

    pub fn contains(&self, endpoint: Endpoint) -> bool {
        let peers = self.peers.read().unwrap();
        peers.contains_key(&endpoint)
    }

    pub fn is_empty(&self) -> bool {
        let peers = self.peers.read().unwrap();
        peers.is_empty()
    }

    pub fn len(&self) -> usize {
        let peers = self.peers.read().unwrap();
        peers.len()
    }

    pub fn endpoints<R, F: Fn(Endpoint) -> R>(&self, f: F) -> Vec<R> {
        let peers = self.peers.read().unwrap();
        peers.keys().copied().map(f).collect()
    }

    pub fn map<R, F: Fn((&Endpoint, &Entry)) -> R>(&self, f: F) -> Vec<R> {
        let peers = self.peers.read().unwrap();
        peers.iter().map(f).collect()
    }

    pub fn connected(
        &self,
        endpoint: Endpoint,
        time: Milliseconds,
        user_agent: String,
        prober: bool,
    ) {
        if endpoint.is_local() || endpoint.is_private() {
            return;
        }
        let mut peers = self.peers.write().unwrap();
        peers
            .entry(endpoint)
            .and_modify(|entry| {
                entry.connected(time, user_agent.clone(), prober);
            })
            .or_insert_with(|| Entry::with_connected(time, user_agent));
    }

    pub fn try_contact(&self, endpoint: Endpoint) -> bool {
        if endpoint.is_local() || endpoint.is_private() {
            return false;
        }
        let mut contacted = false;
        let mut inserted = false;
        {
            let mut peers = self.peers.write().unwrap();
            // ignore max size
            peers
                .entry(endpoint)
                .and_modify(|entry| {
                    if entry.contact() {
                        contacted = true;
                    }
                })
                .or_insert_with(|| {
                    inserted = true;
                    Entry::new(true)
                });
        }
        contacted || inserted
    }

    pub fn contacted(&self, endpoint: Endpoint) {
        if endpoint.is_local() || endpoint.is_private() {
            return;
        }
        let mut contacted = false;
        let mut inserted = false;
        {
            let mut peers = self.peers.write().unwrap();
            // ignore max size
            peers
                .entry(endpoint)
                .and_modify(|entry| {
                    if entry.contact() {
                        contacted = true;
                    }
                })
                .or_insert_with(|| {
                    inserted = true;
                    Entry::new(true)
                });
        }
        if contacted || inserted {
            return;
        }
        error!(
            self.logger,
            "Inconsistent contact to {}",
            endpoint.to_log(self.settings.log_endpoint)
        );
    }

    pub fn discontacted(&self, endpoint: Endpoint) {
        if endpoint.is_local() || endpoint.is_private() {
            return;
        }
        let mut discontacted = false;
        let mut visited = false;
        {
            let mut peers = self.peers.write().unwrap();
            peers.entry(endpoint).and_modify(|entry| {
                visited = true;
                if entry.discontact() {
                    discontacted = true;
                }
            });
        }
        if discontacted {
            // return
        } else if !visited {
            error!(
                self.logger,
                "Not found entry of {}",
                endpoint.to_log(self.settings.log_endpoint)
            );
        } else {
            error!(
                self.logger,
                "Inconsistent discontact from {}",
                endpoint.to_log(self.settings.log_endpoint)
            );
        }
    }

    pub fn add(&self, mut new_peers: impl Iterator<Item = Endpoint>) -> usize {
        let mut added = 0;
        {
            let mut peers = self.peers.write().unwrap();
            let free_slots = if MAX_SIZE > peers.len() {
                MAX_SIZE - peers.len()
            } else {
                0
            };
            while added < free_slots {
                if let Some(peer) = new_peers.next() {
                    if Self::add_impl(&mut peers, peer) {
                        added += 1;
                    }
                } else {
                    break;
                }
            }
        }
        added
    }

    fn add_impl(peers: &mut HashMap<Endpoint, Entry>, peer: Endpoint) -> bool {
        if peer.is_local() || peer.is_private() {
            return false;
        }
        if let Endpoint::TORv2 {
            port: _,
            address: _,
        } = peer
        {
            return false;
        } // obsolete
        if peers.contains_key(&peer) {
            return false;
        }
        peers.insert(peer, Entry::new(false));
        true
    }

    pub fn candidate(&self, predicate: impl Fn(&Endpoint, &Entry) -> bool) -> Option<Endpoint> {
        let peers = self.peers.read().unwrap();
        let mut candidates = Vec::<(&Endpoint, &Entry, f32)>::with_capacity(peers.len());
        let now = SystemClock::millis();
        for (endpoint, entry) in peers.iter() {
            if predicate(endpoint, entry) {
                candidates.push((endpoint, entry, entry.chance(now)));
            }
        }
        let mut uid = UniformIntDistribution::<FastRNG>::default();
        let mut f01 = Float01Distribution::<f32, FastRNG>::new();
        FAST_RNG.with_borrow_mut(|rng| {
            while !candidates.is_empty() {
                uid.set_range(0..candidates.len() as u32);
                let random = uid.sample(rng) as usize;
                let (endpoint, entry, chance) = candidates[random];
                if chance > f01.sample(rng) && entry.contact() {
                    return Some(*endpoint);
                } else {
                    candidates.swap_remove(random);
                }
            }
            None
        })
    }

    pub fn random(&self, n: usize) -> Vec<Endpoint> {
        let peers = self.peers.read().unwrap();
        let mut candidates = Vec::<Endpoint>::with_capacity(peers.len());
        for (&endpoint, _) in peers.iter() {
            candidates.push(endpoint);
        }
        Self::shuffle(&mut candidates);
        let x = min(candidates.len(), n);
        candidates.truncate(x);
        candidates
    }

    fn shuffle(slice: &mut [Endpoint]) {
        let mut uid = UniformIntDistribution::<FastRNG>::default();
        FAST_RNG.with_borrow_mut(|rng| {
            for i in 1..slice.len() {
                uid.set_range(0..=i as u32);
                let j = uid.sample(rng) as usize;
                slice.swap(i, j);
            }
        })
    }

    pub(crate) async fn rotate(self: Arc<Self>) {
        let mut rotated = 0;
        let now = SystemClock::millis();
        {
            let mut peers = self.peers.write().unwrap();
            peers.retain(|_, entry| {
                let mut retain = true;
                if entry.is_old(now) && entry.contact() {
                    retain = false;
                    rotated += 1;
                }
                retain
            });
        }
        if rotated != 0 {
            self.save();
            debug!(self.logger, "Rotated {rotated} endpoints");
        }
    }

    fn load(&self) -> Result<(), Box<dyn Error>> {
        let mut peers = self.peers.write().unwrap();
        let mut file = match File::open(self.data_dir.join(FILE_NAME)) {
            Ok(file) => BufReader::new(file),
            Err(err) => {
                if err.kind() == ErrorKind::NotFound {
                    // first run or unlinked file
                    return Ok(());
                } else {
                    return Err(Box::new(err));
                }
            }
        };
        let mut version = [0u8; 4];
        file.read_exact(&mut version)?;
        let version = u32::from_be_bytes(version);
        if version != FILE_VERSION {
            warn!(self.logger, "Unknown {FILE_NAME} version {version}");
            return Ok(());
        }
        let deserealized: HashMap<Endpoint, Entry> = from_read(&mut file)?;
        peers.extend(deserealized);
        Ok(())
    }

    fn save(&self) {
        let peers = self.peers.read().unwrap();
        if let Err(err) = replace(&self.data_dir, FILE_NAME, |buffered| {
            let version = FILE_VERSION.to_be_bytes();
            buffered.write_all(&version)?;
            to_write(&*peers, buffered)
        }) {
            error!(self.logger, "Can't write {FILE_NAME}: {err}");
        }
    }

    fn builtin_peers(mode: &Mode) -> impl Iterator<Item = Endpoint> {
        mode.builtin_peers()
            .lines()
            .map(|line| Endpoint::parse(line, mode.default_p2p_port()).expect("peers.txt"))
    }
}

impl Drop for PeerTable {
    fn drop(&mut self) {
        info!(self.logger, "Saving {FILE_NAME}");
        self.save();
    }
}

#[derive(Deserialize, Serialize)]
pub struct Entry {
    #[serde(skip)]
    in_contact: AtomicBool,
    attempts: u64,
    last_try: Milliseconds,
    last_connected: Milliseconds,
    user_agent: String,
    subnetworks: HashSet<Hash>,
    added: Milliseconds,
}

impl Entry {
    fn new(in_contact: bool) -> Self {
        Self {
            in_contact: AtomicBool::new(in_contact),
            attempts: 0,
            last_try: Milliseconds::ZERO,
            last_connected: Milliseconds::ZERO,
            user_agent: String::new(),
            subnetworks: HashSet::new(),
            added: SystemClock::millis(),
        }
    }

    fn with_connected(time: Milliseconds, user_agent: String) -> Self {
        Self {
            in_contact: AtomicBool::new(true),
            attempts: 0,
            last_try: Milliseconds::ZERO,
            last_connected: time,
            user_agent,
            subnetworks: HashSet::new(),
            added: SystemClock::millis(),
        }
    }

    fn connected(&mut self, time: Milliseconds, user_agent: String, prober: bool) {
        self.last_connected = time;
        self.user_agent = user_agent;
        if !prober {
            self.subnetworks.clear();
        }
        self.attempts = 0;
        self.last_try = time;
    }

    fn contact(&self) -> bool {
        self.in_contact
            .compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed)
            .is_ok()
    }

    fn discontact(&self) -> bool {
        self.in_contact
            .compare_exchange(true, false, Ordering::Relaxed, Ordering::Relaxed)
            .is_ok()
    }

    fn is_old(&self, now: Milliseconds) -> bool {
        if self.last_connected == Milliseconds::ZERO && self.attempts > 15 {
            return true;
        }
        if self.last_connected != Milliseconds::ZERO
            && now - self.last_connected > Milliseconds::from_days(15)
        {
            return true;
        }

        false
    }

    fn chance(&self, now: Milliseconds) -> f32 {
        let age = now - self.last_try;
        let attempts = min(self.attempts, i32::MAX as u64) as i32;
        let chance = 0.66_f32.powi(min(attempts, 8));
        if age > Milliseconds::from_minutes(15) {
            chance
        } else {
            chance * 0.01
        }
    }

    pub fn in_contact(&self) -> bool {
        self.in_contact.load(Ordering::Relaxed)
    }

    pub const fn attempts(&self) -> u64 {
        self.attempts
    }

    pub const fn last_try(&self) -> Milliseconds {
        self.last_try
    }

    pub const fn last_connected(&self) -> Milliseconds {
        self.last_connected
    }

    pub fn user_agent(&self) -> &str {
        &self.user_agent
    }

    pub const fn subnetworks(&self) -> &HashSet<Hash> {
        &self.subnetworks
    }

    pub const fn added(&self) -> Milliseconds {
        self.added
    }
}

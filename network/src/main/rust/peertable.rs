/*
 * Copyright (c) 2025 Pavel Vasin
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
use blacknet_io::file::replace;
use blacknet_log::{LogManager, debug, error, info, warn};
use blacknet_serialization::format::{from_read, to_write};
use blacknet_time::milliseconds::Milliseconds;
use blacknet_time::systemclock::SystemClock;
use core::error::Error;
use serde::{Deserialize, Serialize};
use spdlog::Logger;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, ErrorKind, Read, Write};
use std::path::PathBuf;
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
                    if !entry.in_contact {
                        entry.in_contact = true;
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
                    if !entry.in_contact {
                        entry.in_contact = true;
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
                if entry.in_contact {
                    entry.in_contact = false;
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

    pub(crate) async fn rotate(self: Arc<Self>) {
        let mut rotated = 0;
        let now = SystemClock::now();
        {
            let mut peers = self.peers.write().unwrap();
            peers.retain(|_, entry| {
                let mut retain = true;
                if entry.is_old(now) && !entry.in_contact {
                    retain = false;
                    rotated += 1;
                    entry.in_contact = true;
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
struct Entry {
    #[serde(skip)]
    in_contact: bool,
    attempts: u64,
    last_try: Milliseconds,
    last_connected: Milliseconds,
    user_agent: String,
    subnetworks: HashSet<[u8; 32]>,
    added: Milliseconds,
}

impl Entry {
    fn new(in_contact: bool) -> Self {
        Self {
            in_contact,
            attempts: 0,
            last_try: Milliseconds::ZERO,
            last_connected: Milliseconds::ZERO,
            user_agent: String::new(),
            subnetworks: HashSet::new(),
            added: Milliseconds::ZERO,
        }
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
}

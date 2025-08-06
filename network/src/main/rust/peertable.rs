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
use blacknet_compat::mode::Mode;
use blacknet_log::error;
use blacknet_log::logmanager::LogManager;
use core::error::Error;
use serde::{Deserialize, Serialize};
use spdlog::Logger;
use std::collections::{HashMap, HashSet};
use std::sync::RwLock;

const MAX_SIZE: usize = 8192;
const FILE_VERSION: u32 = 5;
const FILE_NAME: &str = "peers.dat";

pub struct PeerTable {
    logger: Logger,
    settings: Settings,
    peers: RwLock<HashMap<Endpoint, Entry>>,
}

impl PeerTable {
    pub fn new(
        mode: &Mode,
        log_manager: &LogManager,
        settings: Settings,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            logger: log_manager.logger("PeerTable")?,
            settings,
            peers: RwLock::new(HashMap::with_capacity(MAX_SIZE)),
        })
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
            return;
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
}

#[derive(Deserialize, Serialize)]
struct Entry {
    #[serde(skip)]
    in_contact: bool,
    attempts: u64,
    last_try: u64,
    last_connected: u64,
    user_agent: String,
    subnetworks: HashSet<[u8; 32]>,
    added: u64,
}

impl Entry {
    fn new(in_contact: bool) -> Self {
        Self {
            in_contact,
            attempts: 0,
            last_try: 0,
            last_connected: 0,
            user_agent: String::new(),
            subnetworks: HashSet::new(),
            added: 0,
        }
    }
}

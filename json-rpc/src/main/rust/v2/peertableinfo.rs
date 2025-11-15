/*
 * Copyright (c) 2018-2025 Pavel Vasin
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

use crate::v2::{EndpointInfo, HashInfo};
use blacknet_network::endpoint::Endpoint;
use blacknet_network::peertable::{Entry, PeerTable};
use blacknet_time::Milliseconds;
use serde::{Deserialize, Serialize};
use serde_json::{Value, to_value};

#[derive(Deserialize, Serialize)]
pub struct PeerTableInfo {
    size: u32,
    peers: Value,
}

impl PeerTableInfo {
    pub fn new(peer_table: &PeerTable) -> Self {
        let peers = peer_table.endpoints(|endpoint| Value::String(endpoint.to_log(true)));
        Self {
            size: peers.len() as u32,
            peers: Value::Array(peers),
        }
    }

    pub fn with_stat(peer_table: &PeerTable) -> Self {
        let peers = peer_table.map(|(&endpoint, entry)| {
            to_value(EntryInfo::new(endpoint, entry)).expect("EntryInfo")
        });
        Self {
            size: peers.len() as u32,
            peers: Value::Array(peers),
        }
    }
}

#[derive(Deserialize, Serialize)]
struct EntryInfo {
    address: EndpointInfo,
    in_contact: bool,
    attempts: u64,
    last_try: Milliseconds,
    last_connected: Milliseconds,
    user_agent: String,
    subnetworks: Vec<HashInfo>,
    added: Milliseconds,
}

impl EntryInfo {
    pub fn new(endpoint: Endpoint, entry: &Entry) -> Self {
        Self {
            address: endpoint.into(),
            in_contact: entry.in_contact(),
            attempts: entry.attempts(),
            last_try: entry.last_try(),
            last_connected: entry.last_connected(),
            user_agent: entry.user_agent().to_owned(),
            subnetworks: entry
                .subnetworks()
                .iter()
                .copied()
                .map(HashInfo::from)
                .collect(),
            added: entry.added(),
        }
    }
}

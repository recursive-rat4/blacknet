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

use crate::connection::{Connection, State};
use crate::node::MIN_PROTOCOL_VERSION;
use crate::packet::Packet;
use blacknet_kernel::amount::Amount;
use blacknet_log::{error, info};
use blacknet_serialization::format::from_bytes;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const MAGIC: u8 = 128;
const VERSION: u8 = 129;
const NONCE: u8 = 130;
const AGENT: u8 = 131;
const FEE_FILTER: u8 = 132;

#[derive(Deserialize, Serialize)]
pub struct Hello {
    data: HashMap<u8, Box<[u8]>>,
}

impl Hello {
    fn magic(&self) -> Option<u32> {
        if let Some(bytes) = self.data.get(&MAGIC) {
            from_bytes::<u32>(bytes, false).ok()
        } else {
            None
        }
    }

    fn version(&self) -> Option<u32> {
        if let Some(bytes) = self.data.get(&VERSION) {
            from_bytes::<u32>(bytes, false).ok()
        } else {
            None
        }
    }

    fn nonce(&self) -> Option<u64> {
        if let Some(bytes) = self.data.get(&NONCE) {
            from_bytes::<u64>(bytes, false).ok()
        } else {
            None
        }
    }

    fn agent(&self) -> Option<String> {
        if let Some(bytes) = self.data.get(&AGENT) {
            from_bytes::<String>(bytes, false).ok()
        } else {
            None
        }
    }

    fn fee_filter(&self) -> Option<Amount> {
        if let Some(bytes) = self.data.get(&FEE_FILTER) {
            from_bytes::<Amount>(bytes, false).ok()
        } else {
            None
        }
    }
}

impl Packet for Hello {
    fn handle(self, connection: &mut Connection) {
        let network_magic = connection.node().mode().network_magic();
        if let Some(magic) = self.magic()
            && magic != network_magic
        {
            // connection from another network
            connection.close();
            return;
        }

        // if not provided, the oldest supported version may be tried
        connection.set_version(if let Some(version) = self.version() {
            version
        } else {
            MIN_PROTOCOL_VERSION
        });
        if let Some(agent) = self.agent() {
            connection.set_agent(&agent);
        }
        if let Some(fee_filter) = self.fee_filter() {
            connection.set_fee_filter(fee_filter);
        }

        if connection.version() < MIN_PROTOCOL_VERSION {
            info!(
                connection.logger(),
                "Obsolete protocol version {} {}",
                connection.version(),
                connection.agent()
            );
            connection.close();
            return;
        }

        match connection.state() {
            State::IncomingWaiting => {
                let node = connection.node();
                if let Some(nonce) = self.nonce()
                    && nonce == node.nonce()
                {
                    // connected to self or bad luck
                    connection.close();
                    return;
                }
                send_handshake(connection);
                info!(
                    connection.logger(),
                    "Accepted connection from {}",
                    connection.agent()
                );
                connection.set_state(State::IncomingConnected);
            }
            State::OutgoingWaiting => {
                info!(connection.logger(), "Connected to {}", connection.agent());
                connection.set_state(State::OutgoingConnected);
                let peer_table = connection.node().peer_table();
                peer_table.connected(
                    connection.remote_endpoint(),
                    connection.connected_at(),
                    connection.agent(),
                    false,
                );
            }
            State::ProberWaiting => {
                // keeping track of online peers
                connection.set_state(State::ProberConnected);
                connection.close();
                let peer_table = connection.node().peer_table();
                peer_table.connected(
                    connection.remote_endpoint(),
                    connection.connected_at(),
                    connection.agent(),
                    true,
                );
            }
            _ => {
                error!(
                    connection.logger(),
                    "Unexpected connection state {:?}",
                    connection.state()
                );
                connection.close();
            }
        }
    }
}

fn send_handshake(_connection: &Connection) {
    todo!();
}

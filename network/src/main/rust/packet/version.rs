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
use crate::node::{MIN_PROTOCOL_VERSION, PROTOCOL_VERSION};
use crate::packet::{BlockAnnounce, Packet};
use blacknet_kernel::amount::Amount;
use blacknet_log::{error, info};
use blacknet_time::{Seconds, SystemClock};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Version {
    magic: u32,
    version: u32,
    time: Seconds,
    nonce: u64,
    agent: String,
    fee_filter: Amount,
    block_announce: BlockAnnounce,
}

impl Version {
    pub const fn new(
        magic: u32,
        version: u32,
        time: Seconds,
        nonce: u64,
        agent: String,
        fee_filter: Amount,
        block_announce: BlockAnnounce,
    ) -> Self {
        Self {
            magic,
            version,
            time,
            nonce,
            agent,
            fee_filter,
            block_announce,
        }
    }
}

impl Packet for Version {
    fn handle(self, connection: &mut Connection) {
        let magic = connection.node().mode().network_magic();
        if self.magic != magic {
            // connection from another network
            connection.close();
            return;
        }

        connection.set_time_offset(self.time - SystemClock::secs());
        connection.set_version(self.version);
        connection.set_agent(&self.agent);
        connection.set_fee_filter(self.fee_filter);
        connection.set_last_block(self.block_announce.clone());

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
                if self.nonce != node.nonce() {
                    // echo the nonce
                    send_version(connection, self.nonce);
                    info!(
                        connection.logger(),
                        "Accepted connection from {}",
                        connection.agent()
                    );
                    connection.set_state(State::IncomingConnected);
                } else {
                    // connected to self or bad luck
                    connection.close();
                    return;
                }
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
                return;
            }
            _ => {
                error!(
                    connection.logger(),
                    "Unexpected connection state {:?}",
                    connection.state()
                );
                connection.close();
                return;
            }
        }

        // got anything?
        let block_fetcher = connection.node().block_fetcher();
        block_fetcher.offer(connection, self.block_announce);
    }
}

#[expect(unreachable_code)]
fn send_version(connection: &Connection, nonce: u64) {
    let magic = connection.node().mode().network_magic();
    let user_agent = connection.node().agent_string();
    let min_fee_rate = connection.node().tx_pool().read().unwrap().min_fee_rate();
    connection.send_packet(Version::new(
        magic,
        PROTOCOL_VERSION,
        SystemClock::secs(),
        nonce,
        user_agent.to_owned(),
        min_fee_rate,
        BlockAnnounce::new(todo!(), todo!()),
    ));
}

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

use crate::connection::Connection;
use crate::packet::{MAX_TRANSACTIONS, PACKET_HEADER_SIZE_BYTES, Packet, PacketKind, Transactions};
use blacknet_kernel::blake2b::Hash;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct GetTransactions {
    list: Vec<Hash>,
}

impl GetTransactions {
    pub const fn new(list: Vec<Hash>) -> Self {
        Self { list }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            list: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, hash: Hash) {
        self.list.push(hash)
    }

    pub fn clear(&mut self) {
        self.list.clear()
    }

    pub const fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub const fn len(&self) -> usize {
        self.list.len()
    }
}

impl Packet for GetTransactions {
    fn kind() -> PacketKind {
        PacketKind::GetTransactions
    }

    fn handle(self, connection: &Arc<Connection>) {
        let len = self.list.len();
        if len > MAX_TRANSACTIONS {
            connection.dos("Invalid GetTransactions len");
            return;
        }

        let node = connection.node();
        let tx_pool = node.tx_pool().read().unwrap();

        let mut size = PACKET_HEADER_SIZE_BYTES + 2;
        let max_size = node.min_packet_size(); // actual value is unknown, minimum is assumed
        let mut response = Transactions::with_capacity(len);

        for hash in self.list.into_iter() {
            if let Some(bytes) = tx_pool.get_raw(hash) {
                let new_size = size + bytes.len() as u32 + 4;

                if response.is_empty() {
                    response.push(bytes.into());
                    size = new_size;
                    if size > max_size {
                        connection.send_packet(&response);
                        response.clear();
                        size = PACKET_HEADER_SIZE_BYTES + 2;
                    }
                } else {
                    if new_size > max_size {
                        connection.send_packet(&response);
                        response.clear();
                        size = PACKET_HEADER_SIZE_BYTES + 2;
                    }
                    response.push(bytes.into());
                    size += bytes.len() as u32 + 4;
                }
            }
        }

        if !response.is_empty() {
            connection.send_packet(&response);
        }
    }
}

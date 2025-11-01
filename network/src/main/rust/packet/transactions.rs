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
use crate::packet::Packet;
use blacknet_kernel::amount::Amount;
use blacknet_kernel::blake2b::Hash;
use blacknet_kernel::error::Error;
use blacknet_kernel::transaction::Transaction;
use blacknet_log::debug;
use serde::{Deserialize, Serialize};

pub type UnfilteredInvList = Vec<(Hash, u32, Amount)>;

pub const MAX_TRANSACTIONS: usize = 1000;

#[derive(Deserialize, Serialize)]
pub struct Transactions {
    list: Vec<Box<[u8]>>,
}

impl Transactions {
    pub const fn new(list: Vec<Box<[u8]>>) -> Self {
        Self { list }
    }
}

impl Packet for Transactions {
    fn handle(self, connection: &mut Connection) {
        if self.list.len() > MAX_TRANSACTIONS {
            connection.dos("Invalid Transactions len");
            return;
        }

        let mut inv = UnfilteredInvList::with_capacity(self.list.len());
        let time = connection.last_packet_time();

        let node = connection.node();
        let tx_fetcher = node.tx_fetcher();
        let mut tx_pool = node.tx_pool().write().unwrap();

        for bytes in self.list.into_iter() {
            let hash = if let Some(hash) = Transaction::compute_hash(&bytes) {
                hash
            } else {
                connection.dos("Unhashable tx");
                continue;
            };

            if !tx_fetcher.fetched(connection, hash) {
                connection.dos("Unrequested tx");
                continue;
            }

            match tx_pool.process(hash, &bytes, time, true) {
                Ok(fee) => inv.push((hash, bytes.len() as u32, fee)),
                Err(Error::Invalid(msg)) => connection.dos(&msg),
                Err(Error::InFuture(msg)) => debug!(connection.logger(), "{msg}"),
                Err(Error::NotReachableVertex(msg)) => debug!(connection.logger(), "{msg}"),
                Err(Error::AlreadyHave(msg)) => debug!(connection.logger(), "{msg}"),
            }
        }

        drop(tx_pool);

        if !inv.is_empty() {
            node.broadcast_inv(inv, Some(connection.id()));
            connection.set_last_tx_time(time);
        }
    }
}

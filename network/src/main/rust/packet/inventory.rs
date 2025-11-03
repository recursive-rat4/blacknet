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
use blacknet_kernel::blake2b::Hash;
use blacknet_time::Milliseconds;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub const MAX_INVENTORY: usize = 50000;
pub const INVENTORY_SEND_MAX: usize = 512;
pub const INVENTORY_SEND_TIMEOUT: Milliseconds = Milliseconds::from_seconds(5);

#[derive(Deserialize, Serialize)]
pub struct Inventory {
    list: Box<[Hash]>,
}

impl Inventory {
    pub const fn len(&self) -> usize {
        self.list.len()
    }
}

impl IntoIterator for Inventory {
    type Item = Hash;
    type IntoIter = <Box<[Hash]> as IntoIterator>::IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.list.into_iter()
    }
}

impl Packet for Inventory {
    fn handle(self, connection: &Arc<Connection>) {
        if self.list.is_empty() || self.list.len() > MAX_INVENTORY {
            connection.dos("Invalid Inventory len");
            return;
        }

        let node = connection.node();
        let tx_fetcher = node.tx_fetcher();

        if node.is_initial_synchronization() {
            return;
        }

        tx_fetcher.offer(Arc::downgrade(connection), self);
    }
}

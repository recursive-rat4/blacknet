/*
 * Copyright (c) 2019-2025 Pavel Vasin
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
use crate::packet::{Packet, PacketKind};
use blacknet_crypto::bigint::UInt256;
use blacknet_kernel::blake2b::Hash;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Deserialize, Serialize)]
pub struct BlockAnnounce {
    hash: Hash,
    cumulative_difficulty: Box<[u8]>,
}

impl BlockAnnounce {
    pub fn new(hash: Hash, cumulative_difficulty: UInt256) -> Self {
        Self {
            hash,
            cumulative_difficulty: unsafe { Box::new(cumulative_difficulty.to_java::<32>()) },
        }
    }

    pub const fn hash(&self) -> Hash {
        self.hash
    }

    pub fn cumulative_difficulty(&self) -> UInt256 {
        unsafe { UInt256::from_java(&self.cumulative_difficulty) }
    }

    pub fn raw_cumulative_difficulty(&self) -> &[u8] {
        &self.cumulative_difficulty
    }
}

impl Default for BlockAnnounce {
    fn default() -> Self {
        Self {
            hash: Hash::ZERO,
            cumulative_difficulty: Box::new([0; 1]),
        }
    }
}

impl Packet for BlockAnnounce {
    fn kind() -> PacketKind {
        PacketKind::BlockAnnounce
    }

    fn handle(self, connection: &Arc<Connection>) {
        let len = self.cumulative_difficulty.len();
        if len == 0 || len > 32 {
            connection.dos("Invalid cumulative difficulty len");
            return;
        }

        connection.set_last_block(self.clone());

        let block_fetcher = connection.node().block_fetcher();
        block_fetcher.offer(connection, self);
    }
}

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
use blacknet_kernel::proofofstake::ROLLBACK_LIMIT;
use serde::{Deserialize, Serialize};

pub const MAX_BLOCKS: usize = 1000;
pub const MAX_HASHES: usize = ROLLBACK_LIMIT;

#[derive(Deserialize, Serialize)]
pub struct Blocks {
    hashes: Box<[Hash]>,
    blocks: Box<[Box<[u8]>]>,
}

impl Packet for Blocks {
    fn handle(self, connection: &mut Connection) {
        if self.hashes.len() > MAX_HASHES {
            connection.dos("Invalid hashes len");
            return;
        }
        if self.blocks.len() > MAX_BLOCKS {
            connection.dos("Invalid blocks len");
            return;
        }
        if !self.hashes.is_empty() && !self.blocks.is_empty() {
            connection.dos("Both blocks and hashes");
            return;
        }

        let block_fetcher = connection.node().block_fetcher();
        block_fetcher.blocks(connection, self);
    }
}

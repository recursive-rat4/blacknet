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
use crate::packet::{BlockAnnounce, Blocks, ConsensusFault};
use blacknet_kernel::blake2b::Hash;
use blacknet_kernel::error::Result;

pub struct BlockFetcher {}

impl BlockFetcher {
    #[expect(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {}
    }

    pub fn is_synchronizing(&self) -> bool {
        todo!();
    }

    pub fn disconnected(&self, _connection: &Connection) {
        todo!();
    }

    pub fn offer(&self, _connection: &Connection, _block_announce: BlockAnnounce) {
        todo!();
    }

    pub async fn staked_block(&self, _hash: Hash, _bytes: Vec<u8>) -> Result<usize> {
        todo!();
    }

    pub fn consensus_fault(&self, _connection: &Connection, _consensus_fault: ConsensusFault) {
        todo!();
    }

    pub fn blocks(&self, _connection: &Connection, _blocks: Blocks) {
        todo!();
    }
}

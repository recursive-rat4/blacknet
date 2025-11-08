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

use crate::rollinghashset::RollingHashSet;
use blacknet_kernel::amount::Amount;
use blacknet_kernel::blake2b::Hash;
use blacknet_kernel::proofofstake::ROLLBACK_LIMIT;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct BlockIndex {
    previous: Hash,
    next: Hash,
    next_size: u32,
    height: u32,
    generated: Amount,
}

impl BlockIndex {
    pub const fn previous(self) -> Hash {
        self.previous
    }

    pub const fn next(self) -> Hash {
        self.next
    }

    pub const fn next_size(self) -> u32 {
        self.next_size
    }

    pub const fn height(self) -> u32 {
        self.height
    }

    pub const fn generated(self) -> Amount {
        self.generated
    }
}

pub struct BlockDB {
    cached_block: Arc<Option<(Hash, Box<[u8]>)>>,
    rejects: RollingHashSet<Hash>,
}

impl BlockDB {
    #[expect(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            cached_block: Arc::new(None),
            rejects: RollingHashSet::new(ROLLBACK_LIMIT),
        }
    }

    #[allow(clippy::type_complexity)]
    pub const fn cached_block(&self) -> &Arc<Option<(Hash, Box<[u8]>)>> {
        &self.cached_block
    }

    pub fn is_rejected(&self, hash: Hash) -> bool {
        self.rejects.contains(&hash)
    }

    pub fn index(&self, _hash: Hash) -> Option<BlockIndex> {
        todo!();
    }

    pub fn get_raw(&self, _hash: Hash) -> Option<Box<[u8]>> {
        todo!();
    }

    pub fn next_block_hashes(&self, _start: Hash, _max: usize) -> Option<Vec<Hash>> {
        todo!();
    }
}

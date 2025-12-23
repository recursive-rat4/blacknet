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

use crate::dbview::DBView;
use crate::rollinghashset::RollingHashSet;
use arc_swap::ArcSwapOption;
use blacknet_kernel::amount::Amount;
use blacknet_kernel::blake2b::Hash;
use blacknet_kernel::block::Block;
use blacknet_kernel::proofofstake::ROLLBACK_LIMIT;
use fjall::{Keyspace, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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
    cached_block: ArcSwapOption<(Hash, Box<[u8]>)>,
    rejects: RollingHashSet<Hash>,
    blocks: DBView<Hash, Block>,
    indexes: DBView<Hash, BlockIndex>,
}

impl BlockDB {
    pub fn new(fjall: &Keyspace) -> Result<Self> {
        Ok(Self {
            cached_block: ArcSwapOption::empty(),
            rejects: RollingHashSet::new(ROLLBACK_LIMIT),
            blocks: DBView::with_blob(fjall, "blocks")?,
            indexes: DBView::new(fjall, "indexes")?,
        })
    }

    pub const fn cached_block(&self) -> &ArcSwapOption<(Hash, Box<[u8]>)> {
        &self.cached_block
    }

    pub fn is_rejected(&self, hash: Hash) -> bool {
        self.rejects.contains(&hash)
    }

    pub fn contains(&self, hash: Hash) -> bool {
        self.indexes.contains(hash)
    }

    pub fn index(&self, hash: Hash) -> Option<BlockIndex> {
        self.indexes.get(hash)
    }

    pub fn get(&self, hash: Hash) -> Option<(Block, usize)> {
        self.blocks.get_with_size(hash)
    }

    pub fn get_bytes(&self, hash: Hash) -> Option<Box<[u8]>> {
        self.blocks.get_bytes(hash)
    }

    pub fn next_block_hashes(&self, start: Hash, max: usize) -> Option<Vec<Hash>> {
        let mut index = self.indexes.get(start)?;
        let mut result = Vec::<Hash>::with_capacity(max);
        loop {
            let hash = index.next();
            if hash == Hash::ZERO {
                break;
            }
            result.push(hash);
            if result.len() == max {
                break;
            }
            index = match self.indexes.get(index.next()) {
                Some(index) => index,
                None => break,
            };
        }
        Some(result)
    }

    pub fn hash(&self, _height: u32) -> Option<Hash> {
        todo!();
    }

    /**
     * Return a `Path` of written data or `None` if not synchronized
     */
    pub fn export(&self) -> Option<PathBuf> {
        todo!();
    }

    pub fn check(&self) -> Check {
        todo!();
    }
}

#[derive(Deserialize, Serialize)]
pub struct Check {
    result: bool,
    height: u32,
    indexes: u32,
    blocks: u32,
}

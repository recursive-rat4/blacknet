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

use crate::v2::{HashInfo, PublicKeyInfo};
use blacknet_kernel::blake2b::Hash;
use blacknet_kernel::block::Block;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct BlockNotification {
    hash: HashInfo,
    height: u32,
    size: u32,
    version: u32,
    previous: HashInfo,
    time: i64,
    generator: PublicKeyInfo,
    transactions: u32,
}

impl BlockNotification {
    pub fn new(block: &Block, hash: Hash, height: u32, size: u32) -> Self {
        Self {
            hash: hash.into(),
            height,
            size,
            version: block.version(),
            previous: block.previous().into(),
            time: block.time().into(),
            generator: block.generator().into(),
            transactions: block.raw_transactions().len() as u32,
        }
    }
}

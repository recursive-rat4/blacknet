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

use blacknet_crypto::bigint::UInt256;
use blacknet_kernel::blake2b::Hash;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
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

    pub fn hash(&self) -> Hash {
        self.hash
    }

    pub fn cumulative_difficulty(&self) -> UInt256 {
        unsafe { UInt256::from_java(&self.cumulative_difficulty) }
    }
}

impl Default for BlockAnnounce {
    fn default() -> Self {
        Self {
            hash: Default::default(),
            cumulative_difficulty: Box::new([0; 1]),
        }
    }
}

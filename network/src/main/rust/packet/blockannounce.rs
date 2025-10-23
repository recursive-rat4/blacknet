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

use blacknet_kernel::blake2b::Hash;
use serde::{Deserialize, Serialize};

//TODO BigIntegerSerializer

#[derive(Deserialize, Serialize)]
pub struct BlockAnnounce {
    #[serde(rename = "chain")]
    hash: Hash,
    #[serde(rename = "cumulativeDifficulty")]
    cumulative_difficulty: Box<[u8]>,
}

impl Default for BlockAnnounce {
    fn default() -> Self {
        Self {
            hash: Default::default(),
            cumulative_difficulty: Box::new([0; 1]),
        }
    }
}

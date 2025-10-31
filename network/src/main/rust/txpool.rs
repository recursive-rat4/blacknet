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

use blacknet_kernel::blake2b::Hash;
use std::collections::{HashMap, hash_map::Keys};

pub struct TxPool {
    map: HashMap<Hash, Box<[u8]>>,
    data_len: usize,
}

impl TxPool {
    #[expect(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            data_len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub const fn data_len(&self) -> usize {
        self.data_len
    }

    pub fn hashes(&self) -> Keys<'_, Hash, Box<[u8]>> {
        self.map.keys()
    }

    pub fn get_raw(&self, hash: Hash) -> Option<&[u8]> {
        self.map.get(&hash).map(|x| &**x)
    }
}

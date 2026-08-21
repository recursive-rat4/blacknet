/*
 * Copyright (c) 2018-2026 Pavel Vasin
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

use blacknet_kernel::{amount::Amount, blake2b::Hash};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct BlockIndex {
    previous: Hash,
    pub(super) next: Hash,
    pub(super) next_size: u32,
    height: u32,
    generated: Amount,
}

impl BlockIndex {
    pub const fn new(
        previous: Hash,
        next: Hash,
        next_size: u32,
        height: u32,
        generated: Amount,
    ) -> Self {
        Self {
            previous,
            next,
            next_size,
            height,
            generated,
        }
    }

    pub const fn previous(&self) -> Hash {
        self.previous
    }

    pub const fn next(&self) -> Hash {
        self.next
    }

    pub const fn next_size(&self) -> u32 {
        self.next_size
    }

    pub const fn height(&self) -> u32 {
        self.height
    }

    pub const fn generated(&self) -> Amount {
        self.generated
    }

    pub const fn set_next(&mut self, next: Hash) {
        self.next = next
    }

    pub const fn set_next_size(&mut self, next_size: u32) {
        self.next_size = next_size
    }
}

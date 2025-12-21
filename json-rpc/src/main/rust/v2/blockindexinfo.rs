/*
 * Copyright (c) 2025 Pavel Vasin
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

use crate::v2::{AmountInfo, HashInfo};
use blacknet_network::blockdb::BlockIndex;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct BlockIndexInfo {
    previous: HashInfo,
    next: HashInfo,
    nextSize: u32,
    height: u32,
    generated: AmountInfo,
}

impl BlockIndexInfo {
    pub fn new(index: BlockIndex) -> Self {
        Self {
            previous: index.previous().into(),
            next: index.next().into(),
            nextSize: index.next_size(),
            height: index.height(),
            generated: index.generated().into(),
        }
    }
}

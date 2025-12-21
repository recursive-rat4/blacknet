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

use crate::v2::HashInfo;
use blacknet_network::txpool::TxPool;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TxPoolInfo {
    size: u32,
    dataSize: u32,
    tx: Vec<HashInfo>,
}

impl TxPoolInfo {
    pub fn new(tx_pool: &TxPool) -> Self {
        Self {
            size: tx_pool.len() as u32,
            dataSize: tx_pool.data_len() as u32,
            tx: tx_pool.hashes().copied().map(HashInfo::from).collect(),
        }
    }
}

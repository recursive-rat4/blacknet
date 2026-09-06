/*
 * Copyright (c) 2019-2026 Pavel Vasin
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

use blacknet_network::wallet::{TransactionData, TransactionDataType};
use blacknet_time::Seconds;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TransactionDataTypeInfo {
    r#type: u8,
    dataIndex: u8,
}

impl TransactionDataTypeInfo {
    pub const fn new(tx_data_type: &TransactionDataType) -> Self {
        Self {
            r#type: tx_data_type.kind(),
            dataIndex: tx_data_type.data_index(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct TransactionDataInfo {
    pub types: Vec<TransactionDataTypeInfo>,
    pub time: Seconds,
    pub height: u32,
}

impl TransactionDataInfo {
    pub fn new(tx_data: &TransactionData) -> Self {
        Self {
            types: tx_data
                .types()
                .iter()
                .map(TransactionDataTypeInfo::new)
                .collect(),
            time: tx_data.time(),
            height: tx_data.height(),
        }
    }
}

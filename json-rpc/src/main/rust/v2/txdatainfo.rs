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

use blacknet_kernel::transaction::TxKind;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize)]
pub struct TxDataInfo {
    r#type: u8,
    dataIndex: u32,
    data: Value,
}

impl TxDataInfo {
    pub fn new(kind: TxKind, data_index: u32, data: &[u8]) -> Self {
        #[expect(unreachable_code)]
        Self {
            r#type: kind as u8,
            dataIndex: data_index,
            data: todo!(),
        }
    }
}

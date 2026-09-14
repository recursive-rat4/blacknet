/*
 * Copyright (c) 2020-2025 Pavel Vasin
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

use crate::v2::error::Result;
use blacknet_kernel::transaction::Dispel;
use blacknet_serialization::format::from_bytes;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DispelInfo;

impl DispelInfo {
    pub fn new(data: &[u8]) -> Result<Self> {
        let _dispel = from_bytes::<Dispel>(data, false)?;
        Ok(Self {})
    }
}

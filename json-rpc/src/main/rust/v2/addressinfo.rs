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

use crate::v2::Result;
use blacknet_network::wallet::AddressCodec;
use core::fmt::Write;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AddressInfo {
    publicKey: String,
}

impl AddressInfo {
    pub fn new(string: &str, address_codec: &AddressCodec) -> Result<Self> {
        let public_key = address_codec.decode(string)?;
        let mut hex = String::with_capacity(64);
        for byte in public_key.as_ref() {
            write!(hex, "{byte:02X}").expect("hex format");
        }
        Ok(Self { publicKey: hex })
    }
}

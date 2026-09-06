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

use crate::v2::Result;
use blacknet_kernel::ed25519::{to_public_key, to_secret_key};
use blacknet_network::wallet::AddressCodec;
use core::fmt::Write;
use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

#[derive(Deserialize, Serialize)]
pub struct NewMnemonicInfo {
    mnemonic: String,
    address: String,
    publicKey: String,
}

impl NewMnemonicInfo {
    pub fn new(string: String, address_codec: &AddressCodec) -> Result<Self> {
        let secret_key = to_secret_key(&string).ok_or("Invalid mnemonic")?;
        let public_key = to_public_key(&secret_key);
        let address = address_codec.encode(public_key)?;
        let mut hex = String::with_capacity(64);
        for byte in public_key.as_ref() {
            write!(hex, "{byte:02X}").expect("hex format");
        }
        Ok(Self {
            mnemonic: string,
            address,
            publicKey: hex,
        })
    }
}

impl Drop for NewMnemonicInfo {
    fn drop(&mut self) {
        self.mnemonic.zeroize();
    }
}

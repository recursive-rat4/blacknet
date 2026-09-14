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
use blacknet_crypto::zeroize::ZeroizingString;
use blacknet_kernel::ed25519::{to_public_key, to_secret_key};
use blacknet_network::wallet::{AddressCodec, Mnemonic};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct NewMnemonicInfo {
    mnemonic: ZeroizingString,
    address: String,
    publicKey: String,
}

impl NewMnemonicInfo {
    pub fn new(mnemonic: Mnemonic, address_codec: &AddressCodec) -> Result<Self> {
        let mnemonic = mnemonic.into_zeroizing_string();
        let secret_key = to_secret_key(&mnemonic).ok_or("Invalid mnemonic")?;
        let public_key = to_public_key(&secret_key);
        let address = address_codec.encode(public_key)?;
        let hex = const_hex::encode_upper(public_key);
        Ok(Self {
            mnemonic,
            address,
            publicKey: hex,
        })
    }
}

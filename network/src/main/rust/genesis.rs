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

use blacknet_compat::Mode;
use blacknet_crypto::bigint::UInt256;
use blacknet_kernel::amount::Amount;
use blacknet_kernel::blake2b::Hash;
use blacknet_kernel::ed25519::PublicKey;
use blacknet_time::Seconds;
use data_encoding::HEXLOWER;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::collections::HashMap;

pub const fn time() -> Seconds {
    Seconds::new(1545555600)
}

pub const fn hash() -> Hash {
    Hash::ZERO
}

pub const fn cumulative_difficulty() -> UInt256 {
    UInt256::ZERO
}

pub fn balances(mode: &Mode) -> HashMap<PublicKey, Amount> {
    let json = from_str::<Vec<GenesisJsonEntry>>(mode.genesis_json()).expect("genesis json");
    json.into_iter()
        .map(|entry| (entry.public_key(), entry.balance()))
        .collect()
}

#[allow(nonstandard_style)]
#[derive(Deserialize, Serialize)]
struct GenesisJsonEntry {
    publicKey: String,
    balance: u64,
}

impl GenesisJsonEntry {
    fn public_key(&self) -> PublicKey {
        HEXLOWER
            .decode(self.publicKey.as_bytes())
            .expect("genesis public key hex")
            .try_into()
            .expect("genesis public key len")
    }

    fn balance(&self) -> Amount {
        self.balance.into()
    }
}

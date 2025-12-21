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

use crate::v2::error::Result;
use crate::v2::{AmountInfo, PublicKeyInfo, SignatureInfo};
use blacknet_kernel::transaction::{CreateMultisig, Dep, Sig};
use blacknet_serialization::format::from_bytes;
use blacknet_wallet::address::AddressCodec;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DepInfo {
    from: PublicKeyInfo,
    amount: AmountInfo,
}

impl DepInfo {
    pub fn new(dep: Dep, address_codec: &AddressCodec) -> Result<Self> {
        Ok(Self {
            from: PublicKeyInfo::new(dep.from(), address_codec)?,
            amount: dep.amount().into(),
        })
    }
}

#[derive(Deserialize, Serialize)]
pub struct SigInfo {
    index: u8,
    signature: SignatureInfo,
}

impl From<Sig> for SigInfo {
    fn from(sig: Sig) -> Self {
        Self {
            index: sig.index(),
            signature: sig.signature().into(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct CreateMultisigInfo {
    n: u8,
    deposits: Vec<DepInfo>,
    signatures: Vec<SigInfo>,
}

impl CreateMultisigInfo {
    pub fn new(data: &[u8], address_codec: &AddressCodec) -> Result<Self> {
        let create_multisig = from_bytes::<CreateMultisig>(data, false)?;
        Ok(Self {
            n: create_multisig.n(),
            deposits: create_multisig
                .deposits()
                .iter()
                .copied()
                .map(|dep| DepInfo::new(dep, address_codec))
                .collect::<Result<Vec<_>>>()?,
            signatures: create_multisig
                .signatures()
                .iter()
                .copied()
                .map(SigInfo::from)
                .collect(),
        })
    }
}

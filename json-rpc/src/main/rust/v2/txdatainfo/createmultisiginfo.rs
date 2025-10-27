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
use blacknet_kernel::transaction::{CreateMultisig, Deposit, Sig};
use blacknet_serialization::format::from_bytes;
use blacknet_wallet::address::AddressCodec;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DepositInfo {
    from: PublicKeyInfo,
    amount: AmountInfo,
}

impl DepositInfo {
    pub fn new(deposit: Deposit, address_codec: &AddressCodec) -> Result<Self> {
        Ok(Self {
            from: PublicKeyInfo::new(deposit.from(), address_codec)?,
            amount: deposit.amount().into(),
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
    deposits: Vec<DepositInfo>,
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
                .map(|deposit| DepositInfo::new(deposit, address_codec))
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

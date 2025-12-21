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
use crate::v2::{AmountInfo, txdatainfo::SigInfo};
use blacknet_kernel::transaction::SpendMultisig;
use blacknet_serialization::format::from_bytes;
use blacknet_wallet::address::{AddressCodec, AddressKind};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SpendMultisigInfo {
    id: String,
    amounts: Vec<AmountInfo>,
    signatures: Vec<SigInfo>,
}

impl SpendMultisigInfo {
    pub fn new(data: &[u8], address_codec: &AddressCodec) -> Result<Self> {
        let spend_multisig = from_bytes::<SpendMultisig>(data, false)?;
        Ok(Self {
            id: address_codec.encode_with_kind(AddressKind::Multisig, &spend_multisig.id())?,
            amounts: spend_multisig
                .amounts()
                .iter()
                .copied()
                .map(AmountInfo::from)
                .collect(),
            signatures: spend_multisig
                .signatures()
                .iter()
                .copied()
                .map(SigInfo::from)
                .collect(),
        })
    }
}

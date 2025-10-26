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

use crate::v2::{AmountInfo, HashInfo, PublicKeyInfo, Result, SignatureInfo, TxDataInfo};
use blacknet_kernel::blake2b::Hash;
use blacknet_kernel::transaction::Transaction;
use blacknet_time::Seconds;
use blacknet_wallet::address::AddressCodec;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TransactionNotification {
    hash: HashInfo,
    time: i64,
    size: u32,
    signature: SignatureInfo,
    from: PublicKeyInfo,
    seq: u32,
    referenceChain: HashInfo,
    fee: AmountInfo,
    r#type: u8,
    data: Box<[TxDataInfo]>,
}

impl TransactionNotification {
    pub fn new(
        tx: &Transaction,
        hash: Hash,
        time: Seconds,
        size: u32,
        address_codec: &AddressCodec,
    ) -> Result<Self> {
        #[expect(unreachable_code)]
        Ok(Self {
            hash: hash.into(),
            time: time.into(),
            size,
            signature: tx.signature().into(),
            from: PublicKeyInfo::new(tx.from(), address_codec)?,
            seq: tx.seq(),
            referenceChain: tx.anchor().into(),
            fee: tx.fee().into(),
            r#type: tx.kind() as u8,
            data: todo!(),
        })
    }
}

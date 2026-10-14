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

use crate::v2::{AmountInfo, PublicKeyInfo, Result, TxDataInfo};
use blacknet_kernel::{blake2b::Hash256, ed25519::Signature, transaction::Transaction};
use blacknet_network::{txpool::Notification, wallet::AddressCodec};
use blacknet_time::Seconds;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TransactionNotification {
    hash: Hash256,
    time: i64,
    size: u32,
    signature: Signature,
    from: PublicKeyInfo,
    seq: u32,
    referenceChain: Hash256,
    fee: AmountInfo,
    r#type: u8,
    data: Vec<TxDataInfo>,
}

impl TransactionNotification {
    pub fn new(
        hash: Hash256,
        time: Seconds,
        size: u32,
        tx: &Transaction,
        address_codec: &AddressCodec,
    ) -> Result<Self> {
        Ok(Self {
            hash,
            time: time.into(),
            size,
            signature: tx.signature(),
            from: PublicKeyInfo::new(tx.from(), address_codec)?,
            seq: tx.seq(),
            referenceChain: tx.anchor(),
            fee: tx.fee().into(),
            r#type: tx.kind() as u8,
            data: TxDataInfo::new(tx.kind(), tx.data_bytes(), address_codec)?,
        })
    }

    pub fn with(notification: &Notification, address_codec: &AddressCodec) -> Result<Self> {
        Ok(Self {
            hash: notification.0,
            time: notification.3.into(),
            size: notification.2.len() as u32,
            signature: notification.1.signature(),
            from: PublicKeyInfo::new(notification.1.from(), address_codec)?,
            seq: notification.1.seq(),
            referenceChain: notification.1.anchor(),
            fee: notification.1.fee().into(),
            r#type: notification.1.kind() as u8,
            data: TxDataInfo::new(
                notification.1.kind(),
                notification.1.data_bytes(),
                address_codec,
            )?,
        })
    }
}

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

use crate::v2::{AmountInfo, HashInfo, PublicKeyInfo, Result, SignatureInfo, TxDataInfo};
use blacknet_network::txpool::Notification;
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
    data: Vec<TxDataInfo>,
}

impl TransactionNotification {
    pub fn new(notification: &Notification, address_codec: &AddressCodec) -> Result<Self> {
        Ok(Self {
            hash: notification.1.into(),
            time: notification.2.into(),
            size: notification.3,
            signature: notification.0.signature().into(),
            from: PublicKeyInfo::new(notification.0.from(), address_codec)?,
            seq: notification.0.seq(),
            referenceChain: notification.0.anchor().into(),
            fee: notification.0.fee().into(),
            r#type: notification.0.kind() as u8,
            data: TxDataInfo::new(
                notification.0.kind(),
                notification.0.data_bytes(),
                address_codec,
            )?,
        })
    }
}

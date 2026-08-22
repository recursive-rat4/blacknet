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

use crate::v2::{AmountInfo, PublicKeyInfo, error::Result};
use blacknet_kernel::transaction::{PayloadKind, PaymentId, Transfer};
use blacknet_network::wallet::AddressCodec;
use blacknet_serialization::format::from_bytes;
use data_encoding::HEXUPPER;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PaymentIdInfo {
    kind: PayloadKind,
    payload: String,
}

impl From<&PaymentId> for PaymentIdInfo {
    fn from(payment_id: &PaymentId) -> Self {
        if payment_id.kind() == PayloadKind::Plain
            && let Ok(string) = str::from_utf8(payment_id.payload())
        {
            Self {
                kind: PayloadKind::Plain,
                payload: string.to_owned(),
            }
        } else {
            Self {
                kind: payment_id.kind(),
                payload: HEXUPPER.encode(payment_id.payload()),
            }
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct TransferInfo {
    amount: AmountInfo,
    to: PublicKeyInfo,
    payment_id: PaymentIdInfo,
}

impl TransferInfo {
    pub fn new(data: &[u8], address_codec: &AddressCodec) -> Result<Self> {
        let transfer = from_bytes::<Transfer>(data, false)?;
        Ok(Self {
            amount: transfer.amount().into(),
            to: PublicKeyInfo::new(transfer.to(), address_codec)?,
            payment_id: transfer.payment_id().into(),
        })
    }
}

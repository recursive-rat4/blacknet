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

use crate::v2::error::Result;
use crate::v2::{AmountInfo, PublicKeyInfo};
use blacknet_kernel::transaction::WithdrawFromLease;
use blacknet_serialization::format::from_bytes;
use blacknet_wallet::address::AddressCodec;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct WithdrawFromLeaseInfo {
    withdraw: AmountInfo,
    amount: AmountInfo,
    to: PublicKeyInfo,
    height: u32,
}

impl WithdrawFromLeaseInfo {
    pub fn new(data: &[u8], address_codec: &AddressCodec) -> Result<Self> {
        let withdraw_from_lease = from_bytes::<WithdrawFromLease>(data, false)?;
        Ok(Self {
            withdraw: withdraw_from_lease.withdraw().into(),
            amount: withdraw_from_lease.amount().into(),
            to: PublicKeyInfo::new(withdraw_from_lease.to(), address_codec)?,
            height: withdraw_from_lease.height(),
        })
    }
}

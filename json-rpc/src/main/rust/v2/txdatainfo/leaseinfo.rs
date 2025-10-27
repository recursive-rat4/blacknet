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
use crate::v2::{AmountInfo, PublicKeyInfo};
use blacknet_kernel::transaction::Lease;
use blacknet_serialization::format::from_bytes;
use blacknet_wallet::address::AddressCodec;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct LeaseInfo {
    amount: AmountInfo,
    to: PublicKeyInfo,
}

impl LeaseInfo {
    pub fn new(data: &[u8], address_codec: &AddressCodec) -> Result<Self> {
        let lease = from_bytes::<Lease>(data, false)?;
        Ok(Self {
            amount: lease.amount().into(),
            to: PublicKeyInfo::new(lease.to(), address_codec)?,
        })
    }
}

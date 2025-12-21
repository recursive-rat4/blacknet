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
use crate::v2::{AmountInfo, ByteArrayInfo, PublicKeyInfo};
use blacknet_kernel::hashlock::HashLock;
use blacknet_kernel::timelock::TimeLock;
use blacknet_kernel::transaction::CreateHTLC;
use blacknet_serialization::format::from_bytes;
use blacknet_wallet::address::AddressCodec;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct TimeLockInfo {
    r#type: u8,
    data: i64,
}

impl From<&TimeLock> for TimeLockInfo {
    fn from(time_lock: &TimeLock) -> Self {
        Self {
            r#type: time_lock.algorithm(),
            data: time_lock.data(),
        }
    }
}

#[derive(Deserialize, Serialize)]
struct HashLockInfo {
    r#type: u8,
    data: ByteArrayInfo,
}

impl From<&HashLock> for HashLockInfo {
    fn from(hash_lock: &HashLock) -> Self {
        Self {
            r#type: hash_lock.algorithm(),
            data: hash_lock.image().into(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct CreateHTLCInfo {
    amount: AmountInfo,
    to: PublicKeyInfo,
    time_lock: TimeLockInfo,
    hash_lock: HashLockInfo,
}

impl CreateHTLCInfo {
    pub fn new(data: &[u8], address_codec: &AddressCodec) -> Result<Self> {
        let create_htlc = from_bytes::<CreateHTLC>(data, false)?;
        Ok(Self {
            amount: create_htlc.amount().into(),
            to: PublicKeyInfo::new(create_htlc.to(), address_codec)?,
            time_lock: create_htlc.time_lock().into(),
            hash_lock: create_htlc.hash_lock().into(),
        })
    }
}

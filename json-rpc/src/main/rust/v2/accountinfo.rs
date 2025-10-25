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

use crate::v2::{AmountInfo, PublicKeyInfo};
use blacknet_kernel::account::{Account, Lease};
use blacknet_wallet::address::AddressCodec;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AccountInfo {
    seq: u32,
    balance: AmountInfo,
    confirmedBalance: AmountInfo,
    stakingBalance: AmountInfo,
    inLeases: Vec<LeaseInfo>,
}

impl AccountInfo {
    pub fn new(
        account: &Account,
        height: u32,
        confirmations: u32,
        address_codec: &AddressCodec,
    ) -> Self {
        Self {
            seq: account.seq(),
            balance: account.balance().into(),
            confirmedBalance: account.confirmed_balance(height, confirmations).into(),
            stakingBalance: account.staking_balance(height).into(),
            inLeases: account
                .leases()
                .iter()
                .copied()
                .map(|i| LeaseInfo::new(i, address_codec))
                .collect(),
        }
    }
}

#[derive(Deserialize, Serialize)]
struct LeaseInfo {
    publicKey: PublicKeyInfo,
    height: u32,
    amount: AmountInfo,
}

impl LeaseInfo {
    fn new(lease: Lease, address_codec: &AddressCodec) -> Self {
        Self {
            publicKey: PublicKeyInfo::new(lease.public_key(), address_codec),
            height: lease.height(),
            amount: lease.balance().into(),
        }
    }
}

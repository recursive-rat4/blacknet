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

use crate::v2::AmountInfo;
use blacknet_kernel::amount::Amount;
use blacknet_time::Seconds;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct StakingInfo {
    stakingAccounts: u32,
    hashRate: f64,
    weight: AmountInfo,
    networkWeight: AmountInfo,
    expectedTime: i64,
}

impl StakingInfo {
    pub fn new(
        staking_accounts: u32,
        hash_rate: f64,
        weight: Amount,
        network_weight: Amount,
        expected_time: Seconds,
    ) -> Self {
        Self {
            stakingAccounts: staking_accounts,
            hashRate: hash_rate,
            weight: weight.into(),
            networkWeight: network_weight.into(),
            expectedTime: expected_time.into(),
        }
    }
}

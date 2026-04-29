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

use crate::v2::AmountInfo;
use blacknet_network::staker::StakerStats;
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
    pub fn new(stats: &StakerStats) -> Self {
        Self {
            stakingAccounts: stats.staking_accounts(),
            hashRate: stats.hash_rate(),
            weight: stats.weight().into(),
            networkWeight: stats.network_weight().into(),
            expectedTime: stats.expected_time().into(),
        }
    }
}

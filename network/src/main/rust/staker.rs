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

use blacknet_kernel::amount::Amount;
use blacknet_kernel::ed25519::{PublicKey, SecretKey};
use blacknet_time::Seconds;

#[derive(Default)]
pub struct Staker {}

impl Staker {
    pub const fn new() -> Self {
        Self {}
    }

    pub const fn start_staking(&self, _secret_key: &SecretKey) -> bool {
        todo!();
    }

    pub const fn stop_staking(&self, _secret_key: &SecretKey) -> bool {
        todo!();
    }

    pub const fn is_staking(&self, _secret_key: &SecretKey) -> bool {
        todo!();
    }

    pub const fn stats(&self, _public_key: &Option<PublicKey>) -> StakerStats {
        todo!();
    }
}

pub struct StakerStats {
    staking_accounts: u32,
    hash_rate: f64,
    weight: Amount,
    network_weight: Amount,
    expected_time: Seconds,
}

impl StakerStats {
    pub const fn staking_accounts(&self) -> u32 {
        self.staking_accounts
    }

    pub const fn hash_rate(&self) -> f64 {
        self.hash_rate
    }

    pub const fn weight(&self) -> Amount {
        self.weight
    }

    pub const fn network_weight(&self) -> Amount {
        self.network_weight
    }

    pub const fn expected_time(&self) -> Seconds {
        self.expected_time
    }
}

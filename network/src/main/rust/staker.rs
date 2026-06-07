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
use blacknet_log::{Error as LogError, LogManager, Logger, info};
use blacknet_time::Seconds;
use core::fmt;

pub struct Staker {
    logger: Logger,
    state: State,
}

impl Staker {
    pub fn new(log_manager: &LogManager) -> core::result::Result<Self, LogError> {
        Ok(Self {
            logger: log_manager.logger("Staker")?,
            state: State::Initializing,
        })
    }

    #[expect(unused_variables)]
    pub const fn start_staking(&self, secret_key: &SecretKey) -> bool {
        todo!();
    }

    #[expect(unused_variables)]
    pub const fn stop_staking(&self, secret_key: &SecretKey) -> bool {
        todo!();
    }

    #[expect(unused_variables)]
    pub const fn is_staking(&self, secret_key: &SecretKey) -> bool {
        todo!();
    }

    #[expect(unused_variables)]
    pub const fn stats(&self, public_key: &Option<PublicKey>) -> StakerStats {
        todo!();
    }

    fn set_state(&mut self, state: State) {
        if self.state == state {
            return;
        }
        self.state = state;
        info!(self.logger, "{state}");
    }
}

impl Drop for Staker {
    fn drop(&mut self) {
        self.set_state(State::Terminating);
    }
}

#[expect(dead_code)]
#[derive(Clone, Copy, Eq, PartialEq)]
enum State {
    Initializing,
    Terminating,
    AwaitingOnline,
    AwaitingSync,
    Staking,
    Started,
    Stopped,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::Initializing => f.write_str("Initializing staker"),
            State::Terminating => f.write_str("Terminating staker"),
            State::AwaitingOnline => f.write_str("Awaiting to get online"),
            State::AwaitingSync => f.write_str("Awaiting to get synchronized"),
            State::Staking => f.write_str("Staking"),
            State::Started => f.write_str("Started staker"),
            State::Stopped => f.write_str("Stopped staker"),
        }
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

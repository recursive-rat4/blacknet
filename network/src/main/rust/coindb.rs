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

use crate::genesis;
use blacknet_compat::Mode;
use blacknet_kernel::account::Account;
use blacknet_kernel::amount::Amount;
use blacknet_kernel::blake2b::Hash;
use blacknet_kernel::ed25519::PublicKey;
use blacknet_kernel::proofofstake::DEFAULT_MAX_BLOCK_SIZE;
use blacknet_time::Seconds;
use serde::{Deserialize, Serialize};

pub struct CoinDB {
    state: State,
}

impl CoinDB {
    pub fn new(mode: &Mode) -> Self {
        Self {
            state: State::genesis(mode), //TODO
        }
    }

    pub const fn state(&self) -> State {
        self.state
    }

    pub fn account(&self, _public_key: PublicKey) -> Option<Account> {
        todo!();
    }

    pub fn check(&self) -> Check {
        todo!();
    }
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct State {
    height: u32,
    block_hash: Hash,
    block_time: Seconds,
    difficulty: [u8; 32],            //UInt256,
    cumulative_difficulty: [u8; 32], //UInt256,
    supply: Amount,
    nxtrng: Hash,
    rolling_checkpoint: Hash,
    max_block_size: u32,
    upgraded: u16,
    fork_v2: u16,
}

impl State {
    #[expect(unused)]
    pub fn genesis(mode: &Mode) -> Self {
        let balances = genesis::balances(mode);
        let supply = balances.values().copied().sum();
        Self {
            height: 0,
            block_hash: genesis::hash(),
            block_time: genesis::time(),
            difficulty: todo!(),
            cumulative_difficulty: todo!(),
            supply,
            nxtrng: Hash::ZERO,
            rolling_checkpoint: genesis::hash(),
            max_block_size: DEFAULT_MAX_BLOCK_SIZE,
            upgraded: 0,
            fork_v2: 0,
        }
    }

    pub const fn height(self) -> u32 {
        self.height
    }

    pub const fn block_hash(self) -> Hash {
        self.block_hash
    }

    pub const fn block_time(self) -> Seconds {
        self.block_time
    }

    pub const fn supply(self) -> Amount {
        self.supply
    }

    pub const fn nxtrng(self) -> Hash {
        self.nxtrng
    }

    pub const fn rolling_checkpoint(self) -> Hash {
        self.rolling_checkpoint
    }

    pub const fn max_block_size(self) -> u32 {
        self.max_block_size
    }
}

#[derive(Deserialize, Serialize)]
pub struct Check {
    result: bool,
    accounts: u32,
    htlcs: u32,
    multisigs: u32,
    expected_supply: Amount,
    actual_supply: Amount,
}

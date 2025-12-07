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

use crate::dbview::DBView;
use crate::genesis;
use blacknet_compat::Mode;
use blacknet_crypto::bigint::UInt256;
use blacknet_kernel::account::Account;
use blacknet_kernel::amount::Amount;
use blacknet_kernel::blake2b::Hash;
use blacknet_kernel::ed25519::PublicKey;
use blacknet_kernel::htlc::HTLC;
use blacknet_kernel::multisig::Multisig;
use blacknet_kernel::proofofstake::{DEFAULT_MAX_BLOCK_SIZE, INITIAL_DIFFICULTY};
use blacknet_kernel::transaction::{HashTimeLockContractId, MultiSignatureLockContractId};
use blacknet_time::Seconds;
use fjall::{Keyspace, Result};
use serde::{Deserialize, Serialize};

pub struct CoinDB {
    state: State,
    accounts: DBView<PublicKey, Account>,
    htlcs: DBView<HashTimeLockContractId, HTLC>,
    multisigs: DBView<MultiSignatureLockContractId, Multisig>,
}

impl CoinDB {
    pub fn new(mode: &Mode, fjall: &Keyspace) -> Result<Self> {
        Ok(Self {
            state: State::genesis(mode), //TODO
            accounts: DBView::new(fjall, "accounts")?,
            htlcs: DBView::new(fjall, "htlcs")?,
            multisigs: DBView::new(fjall, "multisigs")?,
        })
    }

    pub const fn state(&self) -> State {
        self.state
    }

    pub fn account(&self, public_key: PublicKey) -> Option<Account> {
        self.accounts.get(public_key)
    }

    pub fn htlc(&self, id: HashTimeLockContractId) -> Option<HTLC> {
        self.htlcs.get(id)
    }

    pub fn multisig(&self, id: MultiSignatureLockContractId) -> Option<Multisig> {
        self.multisigs.get(id)
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
    difficulty: UInt256,
    cumulative_difficulty: UInt256,
    supply: Amount,
    nxtrng: Hash,
    rolling_checkpoint: Hash,
    max_block_size: u32,
    upgraded: u16,
    fork_v2: u16,
}

impl State {
    pub fn genesis(mode: &Mode) -> Self {
        let balances = genesis::balances(mode);
        let supply = balances.values().copied().sum();
        Self {
            height: 0,
            block_hash: genesis::hash(),
            block_time: genesis::time(),
            difficulty: INITIAL_DIFFICULTY,
            cumulative_difficulty: genesis::cumulative_difficulty(),
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

    pub const fn difficulty(self) -> UInt256 {
        self.difficulty
    }

    pub const fn cumulative_difficulty(self) -> UInt256 {
        self.cumulative_difficulty
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

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

use crate::blockdb::{BlockDB, BlockIndex};
use crate::dbview::DBView;
use crate::genesis;
use crate::undoblock::UndoBlock;
use blacknet_compat::Mode;
use blacknet_crypto::bigint::UInt256;
use blacknet_kernel::account::Account;
use blacknet_kernel::amount::Amount;
use blacknet_kernel::blake2b::Hash;
use blacknet_kernel::ed25519::PublicKey;
use blacknet_kernel::error::{Error, Result};
use blacknet_kernel::htlc::HTLC;
use blacknet_kernel::multisig::Multisig;
use blacknet_kernel::proofofstake::{DEFAULT_MAX_BLOCK_SIZE, INITIAL_DIFFICULTY, ROLLBACK_LIMIT};
use blacknet_kernel::transaction::{CoinTx, HashTimeLockContractId, MultiSignatureLockContractId};
use blacknet_serialization::format::{from_bytes, to_bytes};
use blacknet_time::Seconds;
use fjall::{Error as FjallError, Keyspace, WriteBatch};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, hash_map};
use std::sync::Arc;

pub struct CoinDB {
    state: State,
    accounts: DBView<PublicKey, Account>,
    htlcs: DBView<HashTimeLockContractId, HTLC>,
    multisigs: DBView<MultiSignatureLockContractId, Multisig>,
    block_db: Arc<BlockDB>,
}

impl CoinDB {
    pub fn new(
        mode: &Mode,
        fjall: &Keyspace,
        block_db: Arc<BlockDB>,
    ) -> core::result::Result<Arc<Self>, FjallError> {
        Ok(Arc::new(Self {
            state: State::genesis(mode), //TODO
            accounts: DBView::new(fjall, "accounts")?,
            htlcs: DBView::new(fjall, "htlcs")?,
            multisigs: DBView::new(fjall, "multisigs")?,
            block_db,
        }))
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

    pub fn check_anchor(&self, hash: Hash) -> Result<()> {
        if hash == genesis::hash() || self.block_db.indexes.contains(hash) {
            Ok(())
        } else {
            Err(Error::NotReachableVertex(hash.to_string()))
        }
    }

    fn next_rolling_checkpoint(&self) -> Hash {
        if self.state.rolling_checkpoint != genesis::hash() {
            let block_index = self
                .block_db
                .indexes
                .get(self.state.rolling_checkpoint)
                .expect("consistent block index");
            block_index.next()
        } else {
            if self.state.height < ROLLBACK_LIMIT as u32 + 1 {
                return genesis::hash();
            }
            let checkpoint = self.state.height - ROLLBACK_LIMIT as u32;
            let mut block_index = self
                .block_db
                .indexes
                .get(self.state.block_hash)
                .expect("consistent block index");
            while block_index.height() != checkpoint + 1 {
                block_index = self
                    .block_db
                    .indexes
                    .get(block_index.previous())
                    .expect("consistent block index");
            }
            block_index.previous()
        }
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

    pub const fn upgraded(self) -> u16 {
        self.upgraded
    }

    pub const fn fork_v2(self) -> u16 {
        self.fork_v2
    }
}

#[expect(unused)]
struct Update {
    coin_db: Arc<CoinDB>,
    write_batch: WriteBatch,
    block_version: u32,
    block_hash: Hash,
    block_previous: Hash,
    block_time: Seconds,
    block_size: u32,
    block_generator: PublicKey,
    state: State,
    height: u32,
    supply: Amount,
    rolling_checkpoint: Hash,
    accounts: HashMap<PublicKey, Account>,
    htlcs: HashMap<HashTimeLockContractId, Option<HTLC>>,
    multisigs: HashMap<MultiSignatureLockContractId, Option<Multisig>>,
    undo: UndoBlock,
    block_index: Option<BlockIndex>,
    prev_index: Option<BlockIndex>,
}

impl Update {
    #[expect(unused)]
    fn new(
        coin_db: Arc<CoinDB>,
        write_batch: WriteBatch,
        block_version: u32,
        block_hash: Hash,
        block_previous: Hash,
        block_time: Seconds,
        block_size: u32,
        block_generator: PublicKey,
    ) -> Self {
        let state = coin_db.state();
        let rolling_checkpoint = coin_db.next_rolling_checkpoint();
        Self {
            coin_db,
            write_batch,
            block_version,
            block_hash,
            block_previous,
            block_time,
            block_size,
            block_generator,
            state,
            height: state.height() + 1,
            supply: state.supply(),
            rolling_checkpoint,
            accounts: HashMap::new(),
            htlcs: HashMap::new(),
            multisigs: HashMap::new(),
            undo: UndoBlock::new(
                state.block_time(),
                state.difficulty(),
                state.cumulative_difficulty(),
                state.supply(),
                state.nxtrng(),
                state.rolling_checkpoint(),
                state.upgraded(),
                todo!(),
                state.fork_v2(),
            ),
            block_index: None,
            prev_index: None,
        }
    }
}

impl CoinTx for Update {
    fn add_supply(&mut self, amount: Amount) {
        self.supply += amount;
    }

    fn sub_supply(&mut self, amount: Amount) {
        self.supply -= amount;
    }

    fn check_anchor(&self, hash: Hash) -> Result<()> {
        self.coin_db.check_anchor(hash)
    }

    fn block_hash(&self) -> Hash {
        self.block_hash
    }

    fn block_time(&self) -> Seconds {
        self.block_time
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn get_account(&mut self, key: PublicKey) -> Result<Account> {
        match self.accounts.get(&key) {
            Some(account) => Ok(account.clone()),
            None => match self.coin_db.accounts.get_bytes(key) {
                Some(bytes) => {
                    let mut db_account = from_bytes::<Account>(&bytes, false)?;
                    if !db_account.prune(self.height) {
                        self.undo.add(key, Some(bytes));
                    } else {
                        self.undo.add(key, Some(to_bytes(&db_account)?.into()));
                    }
                    Ok(db_account)
                }
                None => Err(Error::Invalid("Account not found".to_owned())),
            },
        }
    }

    fn get_or_create(&mut self, key: PublicKey) -> Account {
        match self.get_account(key) {
            Ok(account) => account,
            Err(_) => {
                self.undo.add(key, None);
                Account::new()
            }
        }
    }

    fn set_account(&mut self, key: PublicKey, state: Account) {
        self.accounts.insert(key, state);
    }

    fn add_htlc(&mut self, id: HashTimeLockContractId, htlc: HTLC) {
        self.undo.add_htlc(id, None);
        self.htlcs.insert(id, Some(htlc));
    }

    fn get_htlc(&mut self, id: HashTimeLockContractId) -> Result<HTLC> {
        match self.htlcs.entry(id) {
            hash_map::Entry::Occupied(entry) => entry
                .get()
                .clone()
                .ok_or_else(|| Error::Invalid("HTLC not found".to_owned())),
            hash_map::Entry::Vacant(_) => {
                let maybe_bytes = self.coin_db.htlcs.get_bytes(id);
                self.undo.add_htlc(id, maybe_bytes.clone());
                match maybe_bytes {
                    Some(bytes) => Ok(from_bytes::<HTLC>(&bytes, false)?),
                    None => Err(Error::Invalid("HTLC not found".to_owned())),
                }
            }
        }
    }

    fn remove_htlc(&mut self, id: HashTimeLockContractId) {
        self.htlcs.insert(id, None);
    }

    fn add_multisig(&mut self, id: MultiSignatureLockContractId, multisig: Multisig) {
        self.undo.add_multisig(id, None);
        self.multisigs.insert(id, Some(multisig));
    }

    fn get_multisig(&mut self, id: MultiSignatureLockContractId) -> Result<Multisig> {
        match self.multisigs.entry(id) {
            hash_map::Entry::Occupied(entry) => entry
                .get()
                .clone()
                .ok_or_else(|| Error::Invalid("Multisig not found".to_owned())),
            hash_map::Entry::Vacant(_) => {
                let maybe_bytes = self.coin_db.multisigs.get_bytes(id);
                self.undo.add_multisig(id, maybe_bytes.clone());
                match maybe_bytes {
                    Some(bytes) => Ok(from_bytes::<Multisig>(&bytes, false)?),
                    None => Err(Error::Invalid("Multisig not found".to_owned())),
                }
            }
        }
    }

    fn remove_multisig(&mut self, id: MultiSignatureLockContractId) {
        self.multisigs.insert(id, None);
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

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

use crate::blockdb::{BlockDB, BlockIndex};
use crate::dbview::DBView;
use crate::fjall::Fjall;
use crate::genesis;
use crate::undoblock::UndoBlock;
use arc_swap::ArcSwap;
use blacknet_compat::Mode;
use blacknet_crypto::bigint::UInt256;
use blacknet_kernel::account::Account;
use blacknet_kernel::amount::Amount;
use blacknet_kernel::blake2b::Hash;
use blacknet_kernel::block::{BLOCK_VERSION, Block};
use blacknet_kernel::ed25519::PublicKey;
use blacknet_kernel::error::{Error, Result};
use blacknet_kernel::htlc::HTLC;
use blacknet_kernel::multisig::Multisig;
use blacknet_kernel::proofofstake::{
    BLOCK_SIZE_SPAN, DEFAULT_MAX_BLOCK_SIZE, INITIAL_DIFFICULTY, ROLLBACK_LIMIT, UPGRADE_THRESHOLD,
    Version as PoSVersion, cumulative_difficulty, max_block_size, mint, next_difficulty, nxtrng,
    verify as verify_pos,
};
use blacknet_kernel::transaction::{
    CoinTx, HashTimeLockContractId, MultiSignatureLockContractId, Transaction,
};
use blacknet_log::{LogManager, Logger, error, info};
use blacknet_serialization::format::{from_bytes, to_bytes};
use blacknet_time::Seconds;
use core::cmp::{max, min};
use core::error::Error as StdError;
use fjall::OwnedWriteBatch as WriteBatch;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque, hash_map};
use std::sync::Arc;

type StateKey = [u8; 1];
const STATE_KEY: StateKey = [0; 1];

pub struct CoinDB {
    logger: Logger,
    state: ArcSwap<State>,
    db_state: DBView<StateKey, State>,
    accounts: DBView<PublicKey, Account>,
    htlcs: DBView<HashTimeLockContractId, HTLC>,
    multisigs: DBView<MultiSignatureLockContractId, Multisig>,
    undos: DBView<Hash, UndoBlock>,
    block_db: Arc<BlockDB>,
}

impl CoinDB {
    pub fn new(
        mode: &Mode,
        fjall: &Fjall,
        log_manager: &LogManager,
        block_db: Arc<BlockDB>,
    ) -> core::result::Result<Arc<Self>, Box<dyn StdError>> {
        let db_state = DBView::new(fjall, "state")?;
        let accounts = DBView::new(fjall, "accounts")?;

        let state = db_state.get(STATE_KEY).map_or_else(
            || State::genesis(mode, fjall, &db_state, &accounts, &block_db.indexes),
            |mut state| {
                state.requires_network = mode.requires_network();
                state
            },
        );

        let logger = log_manager.logger("CoinDB")?;
        info!(
            logger,
            "Consensus height {} PoS {:?}",
            state.height(),
            state.pos_version()
        );

        Ok(Arc::new(Self {
            logger,
            state: ArcSwap::new(Arc::new(state)),
            db_state,
            accounts,
            htlcs: DBView::new(fjall, "htlcs")?,
            multisigs: DBView::new(fjall, "multisigs")?,
            undos: DBView::new(fjall, "undos")?,
            block_db,
        }))
    }

    pub const fn state(&self) -> &ArcSwap<State> {
        &self.state
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

    pub fn prune(&self) {
        let mut batch = self.block_db.fjall.create_write_batch();
        self.prune_batched(&mut batch);
        batch.commit().unwrap();
    }

    pub fn prune_batched(&self, batch: &mut WriteBatch) {
        let mut block_index = self
            .block_db
            .indexes
            .get(self.state.load().rolling_checkpoint)
            .expect("consistent block index");
        loop {
            let hash = block_index.previous();
            if !self.undos.contains(hash) {
                break;
            }
            self.undos.remove(batch, hash);
            if hash == Hash::ZERO {
                break;
            }
            block_index = self
                .block_db
                .indexes
                .get(hash)
                .expect("consistent block index");
        }
    }

    pub fn warnings(&self, warnings: &mut Vec<String>) {
        let state = self.state.load();
        if state.upgraded >= UPGRADE_THRESHOLD / 2 {
            warnings.push("This version is obsolete, upgrade required!".to_owned())
        }
    }

    pub fn check(&self) -> Check {
        let state = self.state.load();
        let mut check = Check {
            result: false,
            accounts: 0,
            htlcs: 0,
            multisigs: 0,
            expected_supply: state.supply,
            actual_supply: Amount::ZERO,
        };
        for (_, account) in self.accounts.iter() {
            check.actual_supply += account.total_balance();
            check.accounts += 1;
        }
        for (_, htlc) in self.htlcs.iter() {
            check.actual_supply += htlc.amount;
            check.htlcs += 1;
        }
        for (_, multisig) in self.multisigs.iter() {
            check.actual_supply += multisig.amount();
            check.multisigs += 1;
        }
        if check.actual_supply == check.expected_supply {
            check.result = true
        }
        check
    }

    pub fn check_anchor(&self, hash: Hash) -> Result<()> {
        if hash == genesis::hash() || self.block_db.indexes.contains(hash) {
            Ok(())
        } else {
            Err(Error::not_reachable_vertex(hash.to_string()))
        }
    }

    fn next_rolling_checkpoint(&self) -> Hash {
        let state = self.state.load();
        if state.rolling_checkpoint != genesis::hash() {
            let block_index = self
                .block_db
                .indexes
                .get(state.rolling_checkpoint)
                .expect("consistent block index");
            block_index.next()
        } else {
            if state.height < ROLLBACK_LIMIT as u32 + 1 {
                return genesis::hash();
            }
            let checkpoint = state.height - ROLLBACK_LIMIT as u32;
            let mut block_index = self
                .block_db
                .indexes
                .get(state.block_hash)
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

    pub fn process_block_impl(
        &self,
        coin_tx: &mut Update,
        hash: Hash,
        block: &Block,
        size: u32,
    ) -> Result<Vec<Hash>> {
        let state = self.state.load();
        if block.previous() != state.block_hash {
            error!(
                self.logger,
                "{hash} not adjacent to {} edge {}",
                state.block_hash,
                block.previous()
            );
            return Err(Error::not_reachable_vertex(block.previous().to_string()));
        }
        if size > state.max_block_size {
            return Err(Error::invalid(format!(
                "Too large block {size} bytes, maximum {}",
                state.max_block_size
            )));
        }
        if block.time() <= state.block_time {
            return Err(Error::invalid("Timestamp is too early"));
        }
        let mut generator = coin_tx.get_account(block.generator())?;
        let height = coin_tx.height();
        let mut tx_hashes = Vec::<Hash>::with_capacity(block.raw_transactions().len());
        let pos_version = state.pos_version();

        verify_pos(
            pos_version,
            block.time(),
            block.generator(),
            state.nxtrng(),
            state.difficulty(),
            state.block_time(),
            generator.staking_balance(height),
        )?;

        coin_tx.set_account(block.generator(), generator);

        let mut fees = Amount::ZERO;
        for tx_bytes in block.raw_transactions() {
            let tx = from_bytes::<Transaction>(tx_bytes, false)?;
            let tx_hash = Transaction::compute_hash(tx_bytes).expect("Hashable tx");
            coin_tx.process_transaction_impl(&tx, tx_hash)?;
            tx_hashes.push(tx_hash);
            fees += tx.fee();

            //TODO WalletDB
        }

        generator = coin_tx.get_account(block.generator())?;

        let mint = mint(pos_version, state.supply);
        let generated = mint + fees;

        let mut prev_index = self
            .block_db
            .indexes
            .get(block.previous())
            .expect("Previous block index");
        prev_index.set_next(hash);
        prev_index.set_next_size(size);
        coin_tx.prev_index = Some(prev_index);
        coin_tx.block_index = Some(BlockIndex::new(
            block.previous(),
            Hash::ZERO,
            0,
            height,
            generated,
        ));

        coin_tx.add_supply(mint);
        generator.debit(height, generated);
        coin_tx.set_account(block.generator(), generator);

        //TODO WalletDB

        Ok(tx_hashes)
    }
}

#[derive(Clone, Deserialize, Serialize)]
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
    block_sizes: VecDeque<u32>,
    #[serde(skip)]
    requires_network: bool,
}

impl State {
    pub fn genesis(
        mode: &Mode,
        fjall: &Fjall,
        db_state: &DBView<StateKey, State>,
        accounts: &DBView<PublicKey, Account>,
        indexes: &DBView<Hash, BlockIndex>,
    ) -> Self {
        let mut supply = Amount::ZERO;
        let mut batch = fjall.create_write_batch();

        for (public_key, balance) in genesis::balances(mode) {
            let account = Account::with_stake(balance);
            accounts.insert(&mut batch, public_key, &account);
            supply += balance;
        }

        let mut block_sizes = VecDeque::with_capacity(BLOCK_SIZE_SPAN);
        block_sizes.push_back(0);
        let state = Self {
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
            block_sizes,
            requires_network: mode.requires_network(),
        };

        let block_index = BlockIndex::new(Hash::ZERO, Hash::ZERO, 0, 0, Amount::ZERO);
        indexes.insert(&mut batch, genesis::hash(), &block_index);

        db_state.insert(&mut batch, STATE_KEY, &state);
        batch.commit().unwrap();
        state
    }

    pub const fn pos_version(&self) -> PoSVersion {
        if self.requires_network {
            if self.fork_v2 == UPGRADE_THRESHOLD + 1 {
                PoSVersion::V4_1
            } else {
                PoSVersion::V4
            }
        } else {
            PoSVersion::V4_1
        }
    }

    pub const fn height(&self) -> u32 {
        self.height
    }

    pub const fn block_hash(&self) -> Hash {
        self.block_hash
    }

    pub const fn block_time(&self) -> Seconds {
        self.block_time
    }

    pub const fn difficulty(&self) -> UInt256 {
        self.difficulty
    }

    pub const fn cumulative_difficulty(&self) -> UInt256 {
        self.cumulative_difficulty
    }

    pub const fn supply(&self) -> Amount {
        self.supply
    }

    pub const fn nxtrng(&self) -> Hash {
        self.nxtrng
    }

    pub const fn rolling_checkpoint(&self) -> Hash {
        self.rolling_checkpoint
    }

    pub const fn max_block_size(&self) -> u32 {
        self.max_block_size
    }

    pub const fn upgraded(&self) -> u16 {
        self.upgraded
    }

    pub const fn fork_v2(&self) -> u16 {
        self.fork_v2
    }
}

pub struct Update {
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
    pub fn new(
        coin_db: Arc<CoinDB>,
        write_batch: WriteBatch,
        block_version: u32,
        block_hash: Hash,
        block_previous: Hash,
        block_time: Seconds,
        block_size: u32,
        block_generator: PublicKey,
    ) -> Self {
        let state = coin_db.state().load_full();
        let height = state.height() + 1;
        let supply = state.supply();
        let rolling_checkpoint = coin_db.next_rolling_checkpoint();
        let undo = UndoBlock::new(
            state.block_time(),
            state.difficulty(),
            state.cumulative_difficulty(),
            state.supply(),
            state.nxtrng(),
            state.rolling_checkpoint(),
            state.upgraded(),
            *state.block_sizes.front().expect("consistent coin db state"),
            state.fork_v2(),
        );
        Self {
            coin_db,
            write_batch,
            block_version,
            block_hash,
            block_previous,
            block_time,
            block_size,
            block_generator,
            state: Arc::unwrap_or_clone(state),
            height,
            supply,
            rolling_checkpoint,
            accounts: HashMap::new(),
            htlcs: HashMap::new(),
            multisigs: HashMap::new(),
            undo,
            block_index: None,
            prev_index: None,
        }
    }

    pub fn commit_impl(mut self) {
        if self.state.block_sizes.len() == BLOCK_SIZE_SPAN {
            self.state.block_sizes.pop_front();
        }
        self.state.block_sizes.push_back(self.block_size);

        let pos_version = self.state.pos_version();
        let difficulty = next_difficulty(
            pos_version,
            self.undo.difficulty(),
            self.undo.block_time(),
            self.block_time,
        );
        let cumulative_difficulty =
            cumulative_difficulty(self.undo.cumulative_difficulty(), difficulty);
        let nxtrng = nxtrng(self.state.nxtrng, self.block_generator);
        let max_block_size = max_block_size(&self.state.block_sizes);
        let upgraded = if self.block_version > BLOCK_VERSION {
            min(self.state.upgraded + 1, UPGRADE_THRESHOLD + 1)
        } else {
            max(self.state.upgraded.saturating_sub(1), 0)
        };
        let fork_v2 = if self.block_version >= 2 {
            min(self.state.fork_v2 + 1, UPGRADE_THRESHOLD + 1)
        } else {
            max(self.state.fork_v2.saturating_sub(1), 0)
        };
        let new_state = State {
            height: self.height,
            block_hash: self.block_hash,
            block_time: self.block_time,
            difficulty,
            cumulative_difficulty,
            supply: self.supply,
            nxtrng,
            rolling_checkpoint: self.rolling_checkpoint,
            max_block_size,
            upgraded,
            fork_v2,
            block_sizes: self.state.block_sizes,
            requires_network: self.state.requires_network,
        };
        let batch = &mut self.write_batch;
        self.coin_db.db_state.insert(batch, STATE_KEY, &new_state);
        self.coin_db.state.store(Arc::new(new_state));
        self.coin_db
            .undos
            .insert(batch, self.block_hash, &self.undo);
        self.coin_db
            .block_db
            .indexes
            .insert(batch, self.block_previous, &self.prev_index.unwrap());
        self.coin_db
            .block_db
            .indexes
            .insert(batch, self.block_hash, &self.block_index.unwrap());
        for (key, account) in self.accounts {
            self.coin_db.accounts.insert(batch, key, &account)
        }
        for (id, htlc) in self.htlcs {
            match htlc {
                Some(htlc) => self.coin_db.htlcs.insert(batch, id, &htlc),
                None => self.coin_db.htlcs.remove(batch, id),
            }
        }
        for (id, multisig) in self.multisigs {
            match multisig {
                Some(multisig) => self.coin_db.multisigs.insert(batch, id, &multisig),
                None => self.coin_db.multisigs.remove(batch, id),
            }
        }
        self.write_batch.commit().unwrap();
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
                None => Err(Error::invalid("Account not found")),
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
                .ok_or_else(|| Error::invalid("HTLC not found")),
            hash_map::Entry::Vacant(_) => {
                let maybe_bytes = self.coin_db.htlcs.get_bytes(id);
                self.undo.add_htlc(id, maybe_bytes.clone());
                match maybe_bytes {
                    Some(bytes) => Ok(from_bytes::<HTLC>(&bytes, false)?),
                    None => Err(Error::invalid("HTLC not found")),
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
                .ok_or_else(|| Error::invalid("Multisig not found")),
            hash_map::Entry::Vacant(_) => {
                let maybe_bytes = self.coin_db.multisigs.get_bytes(id);
                self.undo.add_multisig(id, maybe_bytes.clone());
                match maybe_bytes {
                    Some(bytes) => Ok(from_bytes::<Multisig>(&bytes, false)?),
                    None => Err(Error::invalid("Multisig not found")),
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

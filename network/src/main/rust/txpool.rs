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

use crate::db::{BlockDB, BlockNotifier, CoinDB};
use blacknet_compat::config::Network as Config;
use blacknet_kernel::{
    account::Account,
    amount::Amount,
    blake2b::Hash,
    block::Block,
    ed25519::PublicKey,
    error::{Error, Result},
    htlc::HTLC,
    multisig::Multisig,
    transaction::{CoinTx, HashTimeLockContractId, MultiSignatureLockContractId, Transaction},
};
use blacknet_log::{Error as LogError, LogManager, Logger, debug, error, warn};
use blacknet_serialization::format::from_bytes;
use blacknet_time::{Milliseconds, Seconds};
use core::{
    cmp::{max, min},
    mem::replace,
};
use std::{
    collections::{HashMap, HashSet, hash_map::Keys},
    sync::{Arc, Mutex, RwLock},
};
use tokio::{runtime::Runtime, sync::mpsc};

pub type Notification = (Transaction, Hash, Milliseconds, u32);
pub type Notifier = mpsc::UnboundedReceiver<Arc<Notification>>;
pub type Subscriber = mpsc::UnboundedSender<Arc<Notification>>;

pub struct TxPool {
    logger: Logger,
    config: Arc<Config>,
    map: HashMap<Hash, Box<[u8]>>,
    rejects: HashSet<Hash>,
    data_size: usize,
    max_seen_len: usize,
    accounts: HashMap<PublicKey, Account>,
    htlcs: HashMap<HashTimeLockContractId, Option<HTLC>>,
    multisigs: HashMap<MultiSignatureLockContractId, Option<Multisig>>,
    transactions: Vec<Hash>,
    undo_accounts: HashMap<PublicKey, Option<Account>>,
    undo_htlcs: HashMap<HashTimeLockContractId, (bool, Option<HTLC>)>,
    undo_multisigs: HashMap<MultiSignatureLockContractId, (bool, Option<Multisig>)>,
    coin_db: Arc<CoinDB>,
    subscribers: Mutex<Vec<Subscriber>>,
}

impl TxPool {
    pub fn new(
        log_manager: &LogManager,
        runtime: &Runtime,
        config: Arc<Config>,
        block_db: &BlockDB,
        coin_db: Arc<CoinDB>,
    ) -> core::result::Result<Arc<RwLock<Self>>, LogError> {
        let tx_pool = Arc::new(RwLock::new(Self {
            logger: log_manager.logger("TxPool")?,
            config,
            map: HashMap::new(),
            rejects: HashSet::new(),
            data_size: 0,
            max_seen_len: 512,
            accounts: HashMap::new(),
            htlcs: HashMap::new(),
            multisigs: HashMap::new(),
            transactions: Vec::new(),
            undo_accounts: HashMap::new(),
            undo_htlcs: HashMap::new(),
            undo_multisigs: HashMap::new(),
            coin_db,
            subscribers: Mutex::new(Vec::new()),
        }));
        runtime.spawn(TxPool::block_observer(
            tx_pool.clone(),
            block_db.subscribe(),
        ));
        Ok(tx_pool)
    }

    pub fn subscribe(&self) -> Notifier {
        let (sender, receiver) = mpsc::unbounded_channel();
        self.subscribers.lock().unwrap().push(sender);
        receiver
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub const fn data_size(&self) -> usize {
        self.data_size
    }

    pub fn min_fee_rate(&self) -> Amount {
        Amount::new(self.config.min_relay_fee_rate)
    }

    pub fn hashes(&self) -> Keys<'_, Hash, Box<[u8]>> {
        self.map.keys()
    }

    pub fn get_raw(&self, hash: Hash) -> Option<&[u8]> {
        self.map.get(&hash).map(|x| &**x)
    }

    pub fn is_interesting(&self, hash: Hash) -> bool {
        !self.rejects.contains(&hash) && !self.map.contains_key(&hash)
    }

    pub fn fill(&self, block: &mut Block) {
        let mut free_block_size = min(
            self.coin_db.state().load().max_block_size(),
            self.config.soft_block_size_limit,
        ) - 176;
        for hash in &self.transactions {
            let Some(bytes) = self.map.get(hash) else {
                error!(self.logger, "Inconsistent TxPool");
                continue;
            };
            if bytes.len() as u32 + 4 > free_block_size {
                break;
            }
            free_block_size -= bytes.len() as u32 + 4;
            block.push(bytes.clone());
        }
    }

    pub fn process(
        &mut self,
        hash: Hash,
        bytes: &[u8],
        time: Milliseconds,
        remote: bool,
    ) -> Result<Amount> {
        if self.rejects.contains(&hash) {
            return Err(Error::invalid("Already rejected tx"));
        }
        if self.map.contains_key(&hash) {
            return Err(Error::already_have(hash.to_string()));
        }
        if self.data_size + bytes.len() > self.config.tx_pool_size {
            if remote {
                return Err(Error::in_future("TxPool is full"));
            } else {
                warn!(self.logger, "TxPool is full");
            }
        }
        let result = self.process_impl_with_fee(hash, bytes, time);
        if matches!(result, Err(Error::Invalid(_)) | Err(Error::InFuture(_))) {
            self.rejects.insert(hash);
        }
        result
    }

    fn process_impl(&mut self, hash: Hash, bytes: &[u8]) -> Result<()> {
        let tx = from_bytes::<Transaction>(bytes, false)?;
        let fee = tx.fee();
        self.check_fee(bytes.len() as u32, fee)?;
        let result = self.process_transaction_impl(&tx, hash);
        self.undo_impl(result)?;
        self.map.insert(hash, bytes.into());
        self.data_size += bytes.len();
        self.transactions.push(hash);
        Ok(())
    }

    fn process_impl_with_fee(
        &mut self,
        hash: Hash,
        bytes: &[u8],
        time: Milliseconds,
    ) -> Result<Amount> {
        let tx = from_bytes::<Transaction>(bytes, false)?;
        let fee = tx.fee();
        let bytes_len = bytes.len() as u32;
        self.check_fee(bytes_len, fee)?;
        let result = self.process_transaction_impl(&tx, hash);
        self.undo_impl(result)?;
        self.map.insert(hash, bytes.into());
        self.data_size += bytes.len();
        self.transactions.push(hash);
        debug!(self.logger, "Accepted {hash}");
        self.notify((tx, hash, time, bytes_len));
        Ok(fee)
    }

    fn check_fee(&self, size: u32, amount: Amount) -> Result<()> {
        if amount >= Amount::new(self.config.min_relay_fee_rate) * (1 + size / 1000).into() {
            Ok(())
        } else {
            Err(Error::invalid(format!("Too low fee {}", amount)))
        }
    }

    fn undo_impl(&mut self, result: Result<()>) -> Result<()> {
        if result.is_ok() {
            self.undo_accounts.clear();
            self.undo_htlcs.clear();
            self.undo_multisigs.clear();
        } else {
            self.undo_accounts.drain().for_each(|(key, account)| {
                match account {
                    Some(account) => self.accounts.insert(key, account),
                    None => self.accounts.remove(&key),
                };
            });
            self.undo_htlcs.drain().for_each(|(id, (insert, htlc))| {
                if insert {
                    self.htlcs.insert(id, htlc);
                } else {
                    self.htlcs.remove(&id);
                }
            });
            self.undo_multisigs
                .drain()
                .for_each(|(id, (insert, multisig))| {
                    if insert {
                        self.multisigs.insert(id, multisig);
                    } else {
                        self.multisigs.remove(&id);
                    }
                });
        }
        result
    }

    fn remove(&mut self, hashes: &[Hash]) {
        if hashes.is_empty() || self.transactions.is_empty() {
            return;
        }

        let (txs, map) = self.steal();
        for hash in txs {
            if !hashes.contains(&hash) {
                let _ = self.process_impl(hash, map.get(&hash).unwrap());
            }
        }
    }

    fn steal(&mut self) -> (Vec<Hash>, HashMap<Hash, Box<[u8]>>) {
        self.max_seen_len = max(self.max_seen_len, self.transactions.len());
        let txs = replace(
            &mut self.transactions,
            Vec::with_capacity(self.max_seen_len),
        );
        let map = replace(&mut self.map, HashMap::with_capacity(self.max_seen_len));
        self.data_size = 0;
        self.accounts.clear();
        self.htlcs.clear();
        self.multisigs.clear();
        (txs, map)
    }

    fn notify(&self, notification: Notification) {
        let notification = Arc::new(notification);
        let subscribers = self.subscribers.lock().unwrap();
        for subscriber in subscribers.iter() {
            let _ = subscriber.send(notification.clone());
        }
    }

    async fn block_observer(tx_pool: Arc<RwLock<Self>>, mut block_notifier: BlockNotifier) {
        while let Some(notification) = block_notifier.recv().await {
            let mut tx_pool = tx_pool.write().unwrap();
            tx_pool.rejects.clear();
            tx_pool.remove(&notification.4);
        }
    }
}

impl CoinTx for TxPool {
    fn add_supply(&mut self, _amount: Amount) {}

    fn sub_supply(&mut self, _amount: Amount) {}

    fn check_anchor(&self, hash: Hash) -> Result<()> {
        self.coin_db.check_anchor(hash)
    }

    fn block_hash(&self) -> Hash {
        self.coin_db.state().load().block_hash()
    }

    fn block_time(&self) -> Seconds {
        self.coin_db.state().load().block_time()
    }

    fn height(&self) -> u32 {
        self.coin_db.state().load().height()
    }

    fn get_account(&mut self, key: PublicKey) -> Result<Account> {
        match self.accounts.get(&key) {
            Some(account) => {
                self.undo_accounts
                    .entry(key)
                    .or_insert_with(|| Some(account.clone()));
                Ok(account.clone())
            }
            None => {
                let db_account = self.coin_db.account(key);
                self.undo_accounts.insert(key, None);
                db_account.ok_or(Error::invalid("Account not found"))
            }
        }
    }

    fn get_or_create(&mut self, key: PublicKey) -> Account {
        match self.get_account(key) {
            Ok(account) => account,
            Err(_) => {
                self.undo_accounts.insert(key, None);
                Account::new()
            }
        }
    }

    fn set_account(&mut self, key: PublicKey, state: Account) {
        self.accounts.insert(key, state);
    }

    fn add_htlc(&mut self, id: HashTimeLockContractId, htlc: HTLC) {
        self.undo_htlcs.insert(id, (false, None));
        self.htlcs.insert(id, Some(htlc));
    }

    fn get_htlc(&mut self, id: HashTimeLockContractId) -> Result<HTLC> {
        if !self.htlcs.contains_key(&id) {
            self.undo_htlcs.insert(id, (false, None));
            self.coin_db
                .htlc(id)
                .ok_or(Error::invalid("HTLC not found"))
        } else {
            let htlc = self.htlcs.get(&id).cloned().flatten();
            self.undo_htlcs
                .entry(id)
                .or_insert_with(|| (true, htlc.clone()));
            htlc.ok_or(Error::invalid("HTLC not found"))
        }
    }

    fn remove_htlc(&mut self, id: HashTimeLockContractId) {
        self.htlcs.insert(id, None);
    }

    fn add_multisig(&mut self, id: MultiSignatureLockContractId, multisig: Multisig) {
        self.undo_multisigs.insert(id, (false, None));
        self.multisigs.insert(id, Some(multisig));
    }

    fn get_multisig(&mut self, id: MultiSignatureLockContractId) -> Result<Multisig> {
        if !self.multisigs.contains_key(&id) {
            self.undo_multisigs.insert(id, (false, None));
            self.coin_db
                .multisig(id)
                .ok_or(Error::invalid("Multisig not found"))
        } else {
            let multisig = self.multisigs.get(&id).cloned().flatten();
            self.undo_multisigs
                .entry(id)
                .or_insert_with(|| (true, multisig.clone()));
            multisig.ok_or(Error::invalid("Multisig not found"))
        }
    }

    fn remove_multisig(&mut self, id: MultiSignatureLockContractId) {
        self.multisigs.insert(id, None);
    }
}

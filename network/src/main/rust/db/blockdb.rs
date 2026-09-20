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

use crate::{
    db::{
        BlockIndex, CoinDB, DBVersion, DBVersionKey, Fjall, Snapshot, State, Update, View, genesis,
    },
    rollinghashset::RollingHashSet,
};
use arc_swap::ArcSwapOption;
use blacknet_compat::{XDGDirectories, statvfs};
use blacknet_kernel::{
    blake2b::Hash256,
    block::{BLOCK_VERSION, Block},
    error::{Error, Result},
    proofofstake::{
        MAX_BLOCK_SIZE, ROLLBACK_LIMIT, UPGRADE_THRESHOLD, Version as PoSVersion,
        is_too_far_in_future,
    },
};
use blacknet_log::{LogManager, Logger, debug, error, info, warn};
use blacknet_serialization::from_bytes;
use blacknet_time::SystemClock;
use core::error::Error as StdError;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::{
    fs::{self, File},
    io::{BufReader, BufWriter, Read, Write},
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tokio::sync::mpsc;

const MIN_DISK_SPACE: u64 = MAX_BLOCK_SIZE as u64 * 2;

#[derive(Debug, Default, Deserialize_repr, Serialize_repr)]
#[repr(u32)]
enum BlockDBVersion {
    #[default]
    V1,
}

pub type Notification = (Block, Hash256, u32, u32, Vec<Hash256>);
pub type Notifier = mpsc::UnboundedReceiver<Arc<Notification>>;
pub type Subscriber = mpsc::UnboundedSender<Arc<Notification>>;

pub struct BlockDB {
    logger: Logger,
    cached_block: ArcSwapOption<(Hash256, Box<[u8]>)>,
    cached_index: ArcSwapOption<(Hash256, BlockIndex)>,
    rejects: Mutex<RollingHashSet<Hash256>>,
    subscribers: Mutex<Vec<Subscriber>>,
    pub(super) blocks: View<Hash256, Block>,
    pub(crate) indexes: View<Hash256, BlockIndex>,
    fjall: Arc<Fjall>,
    data_dir: PathBuf,
}

impl BlockDB {
    pub fn new(
        dirs: &XDGDirectories,
        fjall: Arc<Fjall>,
        db_version: &DBVersion,
        log_manager: &LogManager,
    ) -> Result<Arc<Self>, Box<dyn StdError>> {
        let logger = log_manager.logger("BlockDB")?;

        let snapshot = fjall.snapshot();
        match db_version.get_or_err::<BlockDBVersion>(&snapshot, DBVersionKey::BlockDB) {
            Some(Ok(version)) => debug!(logger, "Open {version:?}"),
            Some(Err(err)) => {
                debug!(logger, "{err:?}");
                return Err("Unknown BlockDB version".into());
            }
            None => {
                let version = BlockDBVersion::default();
                debug!(logger, "Initializing {version:?}");
                let mut batch = fjall.create_write_batch();
                batch.verset(db_version, DBVersionKey::BlockDB, &version);
                batch.commit();
            }
        }

        Ok(Arc::new(Self {
            logger,
            cached_block: ArcSwapOption::empty(),
            cached_index: ArcSwapOption::empty(),
            rejects: Mutex::new(RollingHashSet::new(ROLLBACK_LIMIT)),
            subscribers: Mutex::new(Vec::new()),
            blocks: View::with_blob(&fjall, "blocks")?,
            indexes: View::new(&fjall, "indexes")?,
            fjall,
            data_dir: dirs.data().to_owned(),
        }))
    }

    pub fn subscribe(&self) -> Notifier {
        let (sender, receiver) = mpsc::unbounded_channel();
        self.subscribers.lock().unwrap().push(sender);
        receiver
    }

    pub const fn cached_block(&self) -> &ArcSwapOption<(Hash256, Box<[u8]>)> {
        &self.cached_block
    }

    pub fn is_rejected(&self, hash: Hash256) -> bool {
        let rejects = self.rejects.lock().unwrap();
        rejects.contains(&hash)
    }

    pub fn remove(&self, hashes: Vec<Hash256>) {
        let mut batch = self.fjall.create_write_batch();
        for hash in hashes {
            batch.remove(&self.blocks, hash)
        }
        batch.commit();
    }

    pub fn contains(&self, snapshot: &Snapshot, hash: Hash256) -> bool {
        snapshot.contains(&self.indexes, hash)
    }

    pub fn index(&self, snapshot: &Snapshot, hash: Hash256) -> Option<BlockIndex> {
        snapshot.get(&self.indexes, hash)
    }

    pub fn get(&self, snapshot: &Snapshot, hash: Hash256) -> Option<(Block, usize)> {
        snapshot.get_with_size(&self.blocks, hash)
    }

    pub fn get_bytes(&self, snapshot: &Snapshot, hash: Hash256) -> Option<Box<[u8]>> {
        snapshot.get_bytes(&self.blocks, hash)
    }

    pub fn next_block_hashes(
        &self,
        snapshot: &Snapshot,
        start: Hash256,
        max: usize,
    ) -> Option<Vec<Hash256>> {
        let mut index = snapshot.get(&self.indexes, start)?;
        let mut result = Vec::<Hash256>::with_capacity(max);
        loop {
            let hash = index.next();
            if hash == Hash256::ZERO {
                break;
            }
            result.push(hash);
            if result.len() == max {
                break;
            }
            index = match snapshot.get(&self.indexes, index.next()) {
                Some(index) => index,
                None => break,
            };
        }
        Some(result)
    }

    pub fn hash(&self, state: &State, snapshot: &Snapshot, height: u32) -> Option<Hash256> {
        if height > state.height() {
            return None;
        } else if height == 0 {
            return Some(genesis::hash());
        } else if height == state.height() {
            return Some(state.block_hash());
        }

        if let Some(cached_index) = self.cached_index.load_full() {
            let (cached_hash, cached_index) = *cached_index;
            if cached_index.height() == height {
                return Some(cached_hash);
            }
        }

        let mut hash: Hash256;
        let mut index: BlockIndex;
        if height < state.height() / 2 {
            hash = genesis::hash();
            index = snapshot
                .get(&self.indexes, hash)
                .expect("consistent block index");
        } else {
            hash = state.block_hash();
            index = snapshot
                .get(&self.indexes, hash)
                .expect("consistent block index");
        }
        if let Some(cached_index) = self.cached_index.load_full() {
            let (cached_hash, cached_index) = *cached_index;
            if height.abs_diff(index.height()) > height.abs_diff(cached_index.height()) {
                hash = cached_hash;
                index = cached_index;
            }
        }

        while index.height() > height {
            hash = index.previous();
            index = snapshot
                .get(&self.indexes, hash)
                .expect("consistent block index");
        }
        while index.height() < height {
            hash = index.next();
            index = snapshot
                .get(&self.indexes, hash)
                .expect("consistent block index");
        }
        if index.height() + (ROLLBACK_LIMIT as u32) < state.height() + 1 {
            self.cached_index.store(Some(Arc::new((hash, index))));
        }

        Some(hash)
    }

    /**
     * Import a bootstrap if the file exists.
     */
    pub fn import(&self, coin_db: &Arc<CoinDB>) {
        let path = self.data_dir.join("bootstrap.dat");
        if let Ok(file) = File::open(&path) {
            let mut file = BufReader::new(file);
            info!(self.logger, "Found bootstrap");
            let mut n = 0;

            loop {
                let mut size = [0u8; 4];
                if file.read_exact(&mut size).is_err() {
                    break;
                }
                let size = u32::from_be_bytes(size);

                let mut bytes =
                    unsafe { Box::<[u8]>::new_zeroed_slice(size as usize).assume_init() };
                if file.read_exact(&mut bytes).is_err() {
                    break;
                }

                if let Some(hash) = Block::compute_hash(&bytes) {
                    match self.process(coin_db, hash, bytes) {
                        Ok(()) => {
                            n += 1;
                            if n & 0xFFFF == 0 {
                                info!(self.logger, "Processed {n} blocks");
                            }
                            coin_db.prune();
                        }
                        Err(Error::AlreadyHave(_)) => {
                            continue;
                        }
                        Err(err) => {
                            warn!(self.logger, "{err} block {hash}");
                            break;
                        }
                    }
                } else {
                    warn!(self.logger, "Can't hash a block in bootstrap");
                    break;
                }
            }

            drop(file);

            if let Err(err) = fs::rename(path, self.data_dir.join("bootstrap.dat.old")) {
                error!(self.logger, "Can't rename bootstrap.dat ({err})");
            }

            info!(self.logger, "Imported {n} blocks");
        }
    }

    /**
     * Return `Some` path of written data or `None` if not synchronized
     */
    pub fn export(&self, state: &State, snapshot: &Snapshot) -> Option<PathBuf> {
        let checkpoint = state.rolling_checkpoint();
        if checkpoint == genesis::hash() {
            return None;
        }

        let path = self.data_dir.join("bootstrap.dat.new");
        let file = File::create(&path).ok()?;
        let mut buffered = BufWriter::new(file);

        let mut hash = genesis::hash();
        let mut index = snapshot.get(&self.indexes, hash)?;
        while hash != checkpoint {
            hash = index.next;
            index = snapshot.get(&self.indexes, hash)?;
            let bytes = snapshot.get_bytes(&self.blocks, hash)?;
            buffered
                .write_all(&(bytes.len() as u32).to_be_bytes())
                .ok()?;
            buffered.write_all(&bytes).ok()?;
        }

        buffered.flush().ok()?;

        Some(path)
    }

    pub fn warnings(&self, warnings: &mut Vec<String>) {
        match statvfs(&self.data_dir) {
            Ok(available) => {
                if available <= MIN_DISK_SPACE {
                    warnings.push("Disk space is low!".to_owned())
                }
            }
            Err(error) => warnings.push(format!("statvfs: {error}")),
        }
    }

    pub fn check(&self, state: &State, snapshot: &Snapshot) -> BlockDBCheck {
        let mut check = BlockDBCheck {
            result: false,
            height: state.height(),
            indexes: 0,
            blocks: 0,
        };
        check.indexes = snapshot.count(&self.indexes) as u32;
        check.blocks = snapshot.count(&self.blocks) as u32;
        // genesis is not in blocks, but is in indexes
        if check.height + 1 == check.indexes && check.height == check.blocks {
            check.result = true;
        }
        check
    }

    pub fn process(&self, coin_db: &Arc<CoinDB>, hash: Hash256, bytes: Box<[u8]>) -> Result<()> {
        let mut rejects = self.rejects.lock().unwrap();
        if rejects.contains(&hash) {
            return Err(Error::invalid("Already rejected block"));
        }
        let (state, snapshot) = Arc::unwrap_or_clone(coin_db.state().load_full());
        if self.contains(&snapshot, hash) {
            return Err(Error::already_have(hash.to_string()));
        }
        let result = self.process_block(coin_db, state, snapshot, hash, bytes);
        if matches!(result, Err(Error::Invalid(_))) {
            rejects.insert(hash);
        }
        result
    }

    fn process_block(
        &self,
        coin_db: &Arc<CoinDB>,
        state: State,
        snapshot: Snapshot,
        hash: Hash256,
        bytes: Box<[u8]>,
    ) -> Result<()> {
        let block = from_bytes::<Block>(&bytes, false)?;
        if block.version() > BLOCK_VERSION {
            let percent = 100 * state.upgraded() / UPGRADE_THRESHOLD;
            if percent > 9 {
                info!(self.logger, "{percent}% upgraded to unknown version");
            } else {
                info!(self.logger, "Unknown version {}", block.version());
            }
        }
        let pos_version = state.pos_version();
        match pos_version {
            PoSVersion::V4_1 => {
                if block.version() < 2 {
                    return Err(Error::invalid(format!(
                        "Block version {} is no longer accepted",
                        block.version()
                    )));
                }
            }
            PoSVersion::V4 => {}
        };
        if is_too_far_in_future(pos_version, SystemClock::secs(), block.time()) {
            return Err(Error::in_future(block.time().to_string()));
        }
        block.verify_content_hash(&bytes)?;
        block.verify_signature(hash)?;
        if block.previous() != state.block_hash() {
            return Err(Error::not_reachable_vertex(block.previous().to_string()));
        }
        let bytes_len = bytes.len() as u32;
        let new_height = state.height() + 1;
        let mut batch = self.fjall.create_write_batch();
        batch.insert_bytes(&self.blocks, hash, &bytes);
        let mut coin_tx = Update::new(
            coin_db.clone(),
            state,
            snapshot,
            batch,
            block.version(),
            hash,
            block.previous(),
            block.time(),
            bytes_len,
            block.generator(),
        );
        let tx_hashes = coin_db.process_block_impl(&mut coin_tx, hash, &block, bytes_len)?;
        coin_tx.commit_impl();
        self.cached_block
            .store(Some(Arc::new((block.previous(), bytes))));
        self.notify((block, hash, new_height, bytes_len, tx_hashes));
        Ok(())
    }

    fn notify(&self, notification: Notification) {
        let notification = Arc::new(notification);
        let subscribers = self.subscribers.lock().unwrap();
        for subscriber in subscribers.iter() {
            let _ = subscriber.send(notification.clone());
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct BlockDBCheck {
    result: bool,
    height: u32,
    indexes: u32,
    blocks: u32,
}

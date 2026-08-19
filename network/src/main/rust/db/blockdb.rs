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

use crate::db::{CoinDB, DBVersion, DBVersionKey, DBView, Fjall, State, Update, genesis};
use crate::rollinghashset::RollingHashSet;
use arc_swap::ArcSwapOption;
use blacknet_compat::{XDGDirectories, statvfs};
use blacknet_kernel::amount::Amount;
use blacknet_kernel::blake2b::Hash;
use blacknet_kernel::block::{BLOCK_VERSION, Block};
use blacknet_kernel::error::{Error, Result};
use blacknet_kernel::proofofstake::{
    MAX_BLOCK_SIZE, ROLLBACK_LIMIT, UPGRADE_THRESHOLD, Version as PoSVersion, is_too_far_in_future,
};
use blacknet_log::{LogManager, Logger, debug, error, info, warn};
use blacknet_serialization::format::from_bytes;
use blacknet_time::SystemClock;
use core::error::Error as StdError;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

const MIN_DISK_SPACE: u64 = MAX_BLOCK_SIZE as u64 * 2;

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct BlockIndex {
    previous: Hash,
    pub(super) next: Hash,
    pub(super) next_size: u32,
    height: u32,
    generated: Amount,
}

impl BlockIndex {
    pub const fn new(
        previous: Hash,
        next: Hash,
        next_size: u32,
        height: u32,
        generated: Amount,
    ) -> Self {
        Self {
            previous,
            next,
            next_size,
            height,
            generated,
        }
    }

    pub const fn previous(&self) -> Hash {
        self.previous
    }

    pub const fn next(&self) -> Hash {
        self.next
    }

    pub const fn next_size(&self) -> u32 {
        self.next_size
    }

    pub const fn height(&self) -> u32 {
        self.height
    }

    pub const fn generated(&self) -> Amount {
        self.generated
    }

    pub const fn set_next(&mut self, next: Hash) {
        self.next = next
    }

    pub const fn set_next_size(&mut self, next_size: u32) {
        self.next_size = next_size
    }
}

#[derive(Debug, Default, Deserialize_repr, Serialize_repr)]
#[repr(u32)]
enum BlockDBVersion {
    #[default]
    V1,
}

pub struct BlockDB {
    logger: Logger,
    cached_block: ArcSwapOption<(Hash, Box<[u8]>)>,
    cached_index: ArcSwapOption<(Hash, BlockIndex)>,
    rejects: Mutex<RollingHashSet<Hash>>,
    pub(super) blocks: DBView<Hash, Block>,
    pub(crate) indexes: DBView<Hash, BlockIndex>,
    pub(super) fjall: Arc<Fjall>,
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

        match db_version.get_or_err::<BlockDBVersion>(DBVersionKey::BlockDB) {
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
            blocks: DBView::with_blob(&fjall, "blocks")?,
            indexes: DBView::new(&fjall, "indexes")?,
            fjall,
            data_dir: dirs.data().to_owned(),
        }))
    }

    pub const fn cached_block(&self) -> &ArcSwapOption<(Hash, Box<[u8]>)> {
        &self.cached_block
    }

    pub fn is_rejected(&self, hash: Hash) -> bool {
        let rejects = self.rejects.lock().unwrap();
        rejects.contains(&hash)
    }

    pub fn remove(&self, hashes: Vec<Hash>) {
        let mut batch = self.fjall.create_write_batch();
        for hash in hashes {
            batch.remove(&self.blocks, hash)
        }
        batch.commit();
    }

    pub fn contains(&self, hash: Hash) -> bool {
        self.indexes.contains(hash)
    }

    pub fn index(&self, hash: Hash) -> Option<BlockIndex> {
        self.indexes.get(hash)
    }

    pub fn get(&self, hash: Hash) -> Option<(Block, usize)> {
        self.blocks.get_with_size(hash)
    }

    pub fn get_bytes(&self, hash: Hash) -> Option<Box<[u8]>> {
        self.blocks.get_bytes(hash)
    }

    pub fn next_block_hashes(&self, start: Hash, max: usize) -> Option<Vec<Hash>> {
        let mut index = self.indexes.get(start)?;
        let mut result = Vec::<Hash>::with_capacity(max);
        loop {
            let hash = index.next();
            if hash == Hash::ZERO {
                break;
            }
            result.push(hash);
            if result.len() == max {
                break;
            }
            index = match self.indexes.get(index.next()) {
                Some(index) => index,
                None => break,
            };
        }
        Some(result)
    }

    pub fn hash(&self, height: u32, state: &State) -> Option<Hash> {
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

        let mut hash: Hash;
        let mut index: BlockIndex;
        if height < state.height() / 2 {
            hash = genesis::hash();
            index = self.indexes.get(hash).expect("consistent block index");
        } else {
            hash = state.block_hash();
            index = self.indexes.get(hash).expect("consistent block index");
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
            index = self.indexes.get(hash).expect("consistent block index");
        }
        while index.height() < height {
            hash = index.next();
            index = self.indexes.get(hash).expect("consistent block index");
        }
        if index.height() < state.height() - ROLLBACK_LIMIT as u32 + 1 {
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
    pub fn export(&self, state: &State) -> Option<PathBuf> {
        let checkpoint = state.rolling_checkpoint();
        if checkpoint == genesis::hash() {
            return None;
        }

        let path = self.data_dir.join("bootstrap.dat.new");
        let file = File::create(&path).ok()?;
        let mut buffered = BufWriter::new(file);

        let mut hash = genesis::hash();
        let mut index = self.indexes.get(hash)?;
        while hash != checkpoint {
            hash = index.next;
            index = self.indexes.get(hash)?;
            let bytes = self.blocks.get_bytes(hash)?;
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

    pub fn check(&self, state: &State) -> BlockDBCheck {
        let mut check = BlockDBCheck {
            result: false,
            height: state.height(),
            indexes: 0,
            blocks: 0,
        };
        check.indexes = self.indexes.count() as u32;
        check.blocks = self.blocks.count() as u32;
        // genesis is not in blocks, but is in indexes
        if check.height + 1 == check.indexes && check.height == check.blocks {
            check.result = true;
        }
        check
    }

    pub fn process(&self, coin_db: &Arc<CoinDB>, hash: Hash, bytes: Box<[u8]>) -> Result<()> {
        let mut rejects = self.rejects.lock().unwrap();
        if rejects.contains(&hash) {
            return Err(Error::invalid("Already rejected block"));
        }
        if self.contains(hash) {
            return Err(Error::already_have(hash.to_string()));
        }
        let result = self.process_block(coin_db, hash, bytes);
        if matches!(result, Err(Error::Invalid(_))) {
            rejects.insert(hash);
        }
        result
    }

    fn process_block(&self, coin_db: &Arc<CoinDB>, hash: Hash, bytes: Box<[u8]>) -> Result<()> {
        let block = from_bytes::<Block>(&bytes, false)?;
        let state = coin_db.state().load();
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
        let mut batch = self.fjall.create_write_batch();
        batch.insert_bytes(&self.blocks, hash, &bytes);
        let mut coin_tx = Update::new(
            coin_db.clone(),
            batch,
            block.version(),
            hash,
            block.previous(),
            block.time(),
            bytes.len() as u32,
            block.generator(),
        );
        #[expect(unused_variables)]
        let tx_hashes =
            coin_db.process_block_impl(&mut coin_tx, hash, &block, bytes.len() as u32)?;
        coin_tx.commit_impl();
        //TODO RPC
        self.cached_block
            .store(Some(Arc::new((block.previous(), bytes))));
        Ok(())
    }
}

#[derive(Deserialize, Serialize)]
pub struct BlockDBCheck {
    result: bool,
    height: u32,
    indexes: u32,
    blocks: u32,
}

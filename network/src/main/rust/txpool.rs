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

use crate::settings::Settings;
use blacknet_kernel::amount::Amount;
use blacknet_kernel::blake2b::Hash;
use blacknet_kernel::error::{Error, Result};
use blacknet_log::{Error as LogError, LogManager, Logger, warn};
use blacknet_time::Milliseconds;
use std::collections::{HashMap, HashSet, hash_map::Keys};
use std::sync::Arc;

pub struct TxPool {
    logger: Logger,
    settings: Arc<Settings>,
    map: HashMap<Hash, Box<[u8]>>,
    rejects: HashSet<Hash>,
    data_len: usize,
}

impl TxPool {
    pub fn new(
        log_manager: &LogManager,
        settings: Arc<Settings>,
    ) -> core::result::Result<Self, LogError> {
        Ok(Self {
            logger: log_manager.logger("TxPool")?,
            settings,
            map: HashMap::new(),
            rejects: HashSet::new(),
            data_len: 0,
        })
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub const fn data_len(&self) -> usize {
        self.data_len
    }

    pub fn min_fee_rate(&self) -> Amount {
        self.settings.min_relay_fee_rate
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

    pub fn process(
        &mut self,
        hash: Hash,
        bytes: &[u8],
        time: Milliseconds,
        remote: bool,
    ) -> Result<Amount> {
        if self.rejects.contains(&hash) {
            return Err(Error::Invalid("Already rejected tx".to_owned()));
        }
        if self.map.contains_key(&hash) {
            return Err(Error::AlreadyHave(hash.to_string()));
        }
        if self.data_len + bytes.len() > self.settings.tx_pool_size {
            if remote {
                return Err(Error::InFuture("TxPool is full".to_owned()));
            } else {
                warn!(self.logger, "TxPool is full");
            }
        }
        let result = self.process_impl(hash, bytes, time);
        if matches!(result, Err(Error::Invalid(_)) | Err(Error::InFuture(_))) {
            self.rejects.insert(hash);
        }
        result
    }

    fn process_impl(&self, _hash: Hash, _bytes: &[u8], _time: Milliseconds) -> Result<Amount> {
        todo!();
    }
}

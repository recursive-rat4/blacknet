/*
 * Copyright (c) 2025 Pavel Vasin
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

use crate::address::AddressCodec;
use crate::wallet::Wallet;
use blacknet_compat::{Mode, XDGDirectories};
use blacknet_kernel::blake2b::Hash;
use blacknet_kernel::ed25519::PublicKey;
use blacknet_log::{LogManager, Logger, error, info};
use core::error::Error;
use std::collections::HashMap;
use std::fs::{DirBuilder, read_dir};
use std::io::Error as IoError;
#[cfg(target_family = "unix")]
use std::os::unix::fs::DirBuilderExt;
use std::path::PathBuf;

pub struct WalletDB {
    logger: Logger,
    address_codec: AddressCodec,
    wallets: HashMap<PublicKey, Wallet>,
}

impl WalletDB {
    #[expect(unused)]
    pub fn new(
        mode: &Mode,
        dirs: &XDGDirectories,
        log_manager: &LogManager,
    ) -> Result<Self, Box<dyn Error>> {
        let logger = log_manager.logger("WalletDB")?;
        info!(logger, "Driving SQLite {}", rusqlite::version());

        let mut wallets = HashMap::new();
        let dir_path = Self::mkdir(dirs)?;
        for dir_entry in read_dir(dir_path)? {
            let dir_entry = dir_entry?;
            match Wallet::open(&dir_entry.path(), mode) {
                Ok(wallet) => {
                    info!(
                        logger,
                        "Loaded wallet {}",
                        dir_entry.file_name().to_string_lossy()
                    );
                    wallets.insert(todo!(), wallet);
                }
                Err(err) => {
                    error!(
                        logger,
                        "Wallet {} error: {}",
                        dir_entry.file_name().to_string_lossy(),
                        err
                    );
                }
            }
        }

        Ok(Self {
            logger,
            address_codec: AddressCodec::new(mode)?,
            wallets,
        })
    }

    fn mkdir(dirs: &XDGDirectories) -> Result<PathBuf, IoError> {
        let path = dirs.data().join("wallets");
        let mut builder = DirBuilder::new();
        builder.recursive(true);
        #[cfg(target_family = "unix")]
        builder.mode(0o700);
        builder.create(&path)?;
        Ok(path)
    }

    pub const fn address_codec(&self) -> &AddressCodec {
        &self.address_codec
    }

    pub fn sequence(&self, public_key: PublicKey) -> Option<u32> {
        self.wallets.get(&public_key).map(Wallet::sequence)
    }

    pub fn anchor(&self) -> Hash {
        todo!();
    }
}

impl Drop for WalletDB {
    fn drop(&mut self) {
        info!(self.logger, "Braking SQLite");
    }
}

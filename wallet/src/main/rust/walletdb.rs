/*
 * Copyright (c) 2025-2026 Pavel Vasin
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
use core::error::Error as StdError;
use core::fmt;
use rusqlite::Error as SqliteError;
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
    pub fn new(
        mode: &Mode,
        dirs: &XDGDirectories,
        log_manager: &LogManager,
    ) -> Result<Self, Box<dyn StdError>> {
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
                    wallets.insert(wallet.public_key()?, wallet);
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

    pub fn sequence(&self, public_key: PublicKey) -> Result<u32, Error> {
        self.wallets
            .get(&public_key)
            .ok_or(Error::UnknownWallet)?
            .sequence()
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

#[derive(Debug)]
pub enum Error {
    UnknownWallet,
    WrongMagic(String),
    Sqlite(SqliteError),
}

impl From<SqliteError> for Error {
    fn from(error: SqliteError) -> Self {
        Self::Sqlite(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownWallet => write!(f, "Requested wallet not found"),
            Self::WrongMagic(name) => {
                write!(f, "This SQLite database doesn't look like {name} wallet")
            }
            Self::Sqlite(err) => write!(f, "{err}"),
        }
    }
}

impl StdError for Error {}

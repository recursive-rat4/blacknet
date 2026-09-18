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

use crate::{
    db::{CoinNotification, CoinNotifier, State, genesis},
    txpool::{Notifier as TxPoolNotifier, TxPool},
    wallet::{AddressCodec, Error as WalletError, OpenError, Wallet},
};
use blacknet_compat::{Mode, XDGDirectories};
use blacknet_kernel::{
    blake2b::Hash256, ed25519::PublicKey, proofofstake::guess_initial_synchronization,
    transaction::Transaction,
};
use blacknet_log::{LogManager, Logger, error, info};
use blacknet_time::{Milliseconds, SystemClock};
use core::{error::Error, fmt};
use hashbrown::{HashMap, hash_map::Entry};
use std::{
    fs::{DirBuilder, read_dir},
    io::Error as IoError,
    path::PathBuf,
    sync::{Arc, OnceLock, RwLock, RwLockReadGuard},
};
use tokio::{runtime::Runtime, sync::mpsc};

#[cfg(target_family = "unix")]
use std::os::unix::fs::DirBuilderExt;

pub type Notification = (Transaction, Hash256, Milliseconds, u32, PublicKey);
pub type Notifier = mpsc::UnboundedReceiver<Notification>;
pub type Subscriber = mpsc::UnboundedSender<Notification>;

pub struct WalletDB {
    logger: Logger,
    address_codec: AddressCodec,
    wallets: RwLock<HashMap<PublicKey, Wallet>>,
    subscriber: OnceLock<Subscriber>,
    mode: Arc<Mode>,
    dir: PathBuf,
}

impl WalletDB {
    pub fn new(
        mode: Arc<Mode>,
        dirs: &XDGDirectories,
        log_manager: &LogManager,
        runtime: &Runtime,
        coin_notifier: CoinNotifier,
        tx_pool: &Arc<RwLock<TxPool>>,
    ) -> Result<Arc<Self>, Box<dyn Error>> {
        let logger = log_manager.logger("WalletDB")?;
        info!(logger, "Driving SQLite {}", rusqlite::version());

        let mut wallets = HashMap::new();
        let dir = Self::mkdir(dirs)?;
        for dir_entry in read_dir(&dir)? {
            let dir_entry = dir_entry?;
            match Wallet::open(&dir_entry.path(), &mode) {
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

        let wallet_db = Arc::new(Self {
            logger,
            address_codec: AddressCodec::new(&mode)?,
            wallets: RwLock::new(wallets),
            subscriber: OnceLock::new(),
            mode,
            dir,
        });

        runtime.spawn(WalletDB::coindb_observer(wallet_db.clone(), coin_notifier));
        runtime.spawn(WalletDB::txpool_observer(
            wallet_db.clone(),
            tx_pool.read().unwrap().subscribe(),
        ));

        Ok(wallet_db)
    }

    pub fn subscribe(&self) -> Notifier {
        let (sender, receiver) = mpsc::unbounded_channel();
        self.subscriber.set(sender).expect("one subscriber");
        receiver
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

    pub fn wallets(&self) -> RwLockReadGuard<'_, HashMap<PublicKey, Wallet>> {
        self.wallets.read().unwrap()
    }

    pub fn ingest(&self, name: &str, wallet: &Wallet) -> Result<(), IngestError> {
        let public_key = wallet.public_key()?;
        let mut wallets = self.wallets.write().unwrap();
        match wallets.entry(public_key) {
            Entry::Vacant(vacant) => {
                let file_name = format!("{name}.sqlite");
                let path = self.dir.join(file_name);
                wallet.vacuum_into(&path)?;
                let wallet = Wallet::open(&path, &self.mode)?;
                vacant.insert(wallet);
                //TODO rescan
                Ok(())
            }
            Entry::Occupied(_) => Err(IngestError::Occupied),
        }
    }

    pub fn anchor(&self, state: &State) -> Hash256 {
        if !guess_initial_synchronization(
            state.pos_version(),
            SystemClock::secs(),
            state.block_time(),
        ) {
            state.rolling_checkpoint()
        } else {
            genesis::hash()
        }
    }

    #[expect(unused_variables)]
    async fn coindb_observer(self: Arc<Self>, mut coin_notifier: CoinNotifier) {
        while let Some(notification) = coin_notifier.recv().await {
            match notification {
                CoinNotification::Transaction {
                    tx_hash,
                    tx,
                    tx_bytes,
                    time,
                    height,
                } => todo!(),
                CoinNotification::Mint {
                    hash,
                    time,
                    generator,
                    height,
                    generated,
                } => todo!(),
                CoinNotification::Rollback { hash } => todo!(),
            }
        }
    }

    #[expect(unused_variables)]
    async fn txpool_observer(self: Arc<Self>, mut txpool_notifier: TxPoolNotifier) {
        while let Some(notification) = txpool_notifier.recv().await {
            todo!();
        }
    }

    #[expect(dead_code)]
    fn notify(&self, notification: Notification) {
        let Some(subscriber) = self.subscriber.get() else {
            return;
        };
        let _ = subscriber.send(notification);
    }
}

impl Drop for WalletDB {
    fn drop(&mut self) {
        info!(self.logger, "Braking SQLite");
    }
}

#[derive(Debug)]
pub enum IngestError {
    Occupied,
    Wallet(WalletError),
    Reopen(OpenError),
}

impl From<WalletError> for IngestError {
    fn from(err: WalletError) -> Self {
        Self::Wallet(err)
    }
}

impl From<OpenError> for IngestError {
    fn from(err: OpenError) -> Self {
        Self::Reopen(err)
    }
}

impl fmt::Display for IngestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Occupied => write!(f, "This public key is already occupied by a wallet"),
            Self::Wallet(err) => write!(f, "{err}"),
            Self::Reopen(err) => write!(f, "Reopen: {err}"),
        }
    }
}

impl Error for IngestError {}

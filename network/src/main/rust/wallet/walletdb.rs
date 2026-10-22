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
    wallet::{
        AddressCodec, Error as WalletError, OpenError, TransactionData, TransactionOutputData,
        Upsert, Wallet,
    },
};
use blacknet_compat::{Mode, XDGDirectories};
use blacknet_kernel::{
    account::Lease as OutLease,
    blake2b::Hash256,
    ed25519::PublicKey,
    proofofstake::guess_initial_synchronization,
    transaction::{
        Batch, CancelLease, ClaimHTLC, CreateHTLC, CreateMultisig, Lease, RefundHTLC,
        SpendMultisig, Transaction, Transfer, TxKind, WithdrawFromLease, htlc_id, multisig_id,
    },
};
use blacknet_log::{LogManager, Logger, error, info, warn};
use blacknet_serialization::{Error as SerializationError, from_bytes, to_bytes};
use blacknet_time::{Seconds, SystemClock};
use core::{error::Error, fmt};
use hashbrown::{HashMap, hash_map::Entry};
use std::{
    fs::DirBuilder,
    io::Error as IoError,
    path::PathBuf,
    sync::{Arc, OnceLock, RwLock, RwLockReadGuard},
};
use tokio::{runtime::Runtime, sync::mpsc};

#[cfg(target_family = "unix")]
use std::os::unix::fs::DirBuilderExt;

const FILE_EXTENSION: &str = "sqlite";

pub struct Notification {
    pub hash: Hash256,
    pub tx: Transaction,
    pub time: Seconds,
    pub height: u32,
    pub wallet: PublicKey,
    pub size: u32,
}
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
        for dir_entry in dir.read_dir()? {
            let dir_entry = dir_entry?;
            let path = dir_entry.path();
            if !(path.extension() == Some(FILE_EXTENSION.as_ref())) {
                warn!(
                    logger,
                    "Ignoring file {}",
                    dir_entry.file_name().to_string_lossy()
                );
                continue;
            }
            match Wallet::open(&path, &mode) {
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

    pub fn ingest(&self, name: &str, wallet: Wallet) -> Result<(), IngestError> {
        let public_key = wallet.public_key()?;
        let mut wallets = self.wallets.write().unwrap();
        match wallets.entry(public_key) {
            Entry::Vacant(vacant) => {
                let mut path = self.dir.join(name);
                path.add_extension(FILE_EXTENSION);

                wallet.vacuum_into(&path)?;
                let wallet = Wallet::open(&path, &self.mode)?;
                vacant.insert(wallet);
                //TODO rescan
                Ok(())
            }
            Entry::Occupied(_) => {
                drop(wallet);
                Err(IngestError::Occupied)
            }
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

    async fn coindb_observer(self: Arc<Self>, mut coin_notifier: CoinNotifier) {
        while let Some(notification) = coin_notifier.recv().await {
            let wallets = self.wallets.read().unwrap();
            match notification {
                CoinNotification::Transaction {
                    tx_hash,
                    tx,
                    tx_bytes,
                    time,
                    height,
                } => {
                    for (&public_key, wallet) in wallets.iter() {
                        if let Err(err) = self.scan_tx(
                            public_key,
                            wallet,
                            tx_hash,
                            &tx,
                            &tx_bytes,
                            time,
                            Some(height),
                        ) {
                            error!(self.logger, "Scan: {err}");
                        }
                    }
                }
                CoinNotification::Mint {
                    hash,
                    time,
                    generator,
                    height,
                    generated,
                } => {
                    let Some(wallet) = wallets.get(&generator) else {
                        continue;
                    };
                    let tx = Transaction::generated(generator, height, hash, generated);
                    let data = TransactionData::new(
                        vec![TransactionOutputData::mint()],
                        time,
                        Some(height),
                    );
                    let bytes = to_bytes(&tx).expect("Serialize generated");
                    match wallet.upsert_transaction(hash, &data, &bytes) {
                        Ok(Upsert::Insert) => self.notify(Notification {
                            hash,
                            tx,
                            time,
                            height,
                            wallet: generator,
                            size: bytes.len() as u32,
                        }),
                        Ok(Upsert::Update) => {}
                        Err(err) => error!(self.logger, "Mint: {err}"),
                    }
                }
                CoinNotification::Rollback { hash } => {
                    for (_, wallet) in wallets.iter() {
                        if let Err(err) = wallet.rollback(hash) {
                            error!(self.logger, "Rollback: {err}");
                        }
                    }
                }
            }
        }
    }

    async fn txpool_observer(self: Arc<Self>, mut txpool_notifier: TxPoolNotifier) {
        while let Some(notification) = txpool_notifier.recv().await {
            let wallets = self.wallets.read().unwrap();
            for (&public_key, wallet) in wallets.iter() {
                if let Err(err) = self.scan_tx(
                    public_key,
                    wallet,
                    notification.0,
                    &notification.1,
                    &notification.2,
                    notification.3.to_secs_lossy(),
                    None,
                ) {
                    error!(self.logger, "Rollback: {err}");
                }
            }
        }
    }

    fn notify(&self, notification: Notification) {
        let Some(subscriber) = self.subscriber.get() else {
            return;
        };
        let _ = subscriber.send(notification);
    }

    fn scan_tx(
        &self,
        public_key: PublicKey,
        wallet: &Wallet,
        hash: Hash256,
        tx: &Transaction,
        bytes: &[u8],
        time: Seconds,
        height: Option<u32>,
    ) -> Result<(), ScanError> {
        let outcoming = public_key == tx.from();
        if outcoming {
            if tx.seq() == wallet.sequence()? {
                wallet.increment_sequence()?;
            } else {
                warn!(self.logger, "Out of order sequence");
            }
        }
        let outputs = if tx.kind() != TxKind::Batch {
            if self.scan_tx_data(
                public_key,
                wallet,
                hash,
                0,
                tx.kind(),
                tx.data_bytes(),
                height,
                outcoming,
            )? {
                vec![TransactionOutputData::new(0, tx.kind() as u8)]
            } else {
                Vec::new()
            }
        } else {
            let mut outputs = Vec::new();
            let batch = from_bytes::<Batch>(bytes, false)?;
            for (idx, data) in batch.multi_data().iter().enumerate() {
                let index = idx as u32 + 1;
                if self.scan_tx_data(
                    public_key,
                    wallet,
                    hash,
                    index,
                    data.kind(),
                    data.data_bytes(),
                    height,
                    outcoming,
                )? {
                    outputs.push(TransactionOutputData::new(index as u8, data.kind() as u8))
                }
            }
            outputs
        };
        if !outputs.is_empty() {
            let data = TransactionData::new(outputs, time, height);
            match wallet.upsert_transaction(hash, &data, bytes)? {
                Upsert::Insert => self.notify(Notification {
                    hash,
                    tx: tx.clone(),
                    time,
                    height: height.unwrap_or(0),
                    wallet: public_key,
                    size: bytes.len() as u32,
                }),
                Upsert::Update => {}
            }
        }
        Ok(())
    }

    fn scan_tx_data(
        &self,
        public_key: PublicKey,
        wallet: &Wallet,
        tx_hash: Hash256,
        index: u32,
        kind: TxKind,
        bytes: &[u8],
        height: Option<u32>,
        outcoming: bool,
    ) -> Result<bool, ScanError> {
        match kind {
            TxKind::Transfer => {
                let transfer = from_bytes::<Transfer>(bytes, false)?;
                Ok(transfer.to() == public_key || outcoming)
            }
            TxKind::Burn => Ok(outcoming),
            TxKind::Lease => {
                let lease = from_bytes::<Lease>(bytes, false)?;
                if outcoming {
                    let out_lease = OutLease::new(lease.to(), height.unwrap_or(0), lease.amount());
                    wallet.upsert_out_lease(tx_hash, out_lease)?;
                }
                Ok(lease.to() == public_key || outcoming)
            }
            TxKind::CancelLease => {
                let cancel_lease = from_bytes::<CancelLease>(bytes, false)?;
                if outcoming {
                    let out_lease = OutLease::new(
                        cancel_lease.to(),
                        cancel_lease.height(),
                        cancel_lease.amount(),
                    );
                    if !wallet.remove_out_lease(out_lease)? {
                        warn!(self.logger, "Lease not found");
                    }
                }
                Ok(cancel_lease.to() == public_key || outcoming)
            }
            TxKind::Blob => Ok(outcoming),
            TxKind::CreateHTLC => {
                let create_htlc = from_bytes::<CreateHTLC>(bytes, false)?;
                let incoming = create_htlc.to() == public_key;
                if incoming || outcoming {
                    let id = htlc_id(tx_hash, index);
                    wallet.upsert_htlc(id)?;
                }
                Ok(incoming || outcoming)
            }
            TxKind::RefundHTLC => {
                let refund_htlc = from_bytes::<RefundHTLC>(bytes, false)?;
                Ok(wallet.remove_htlc(refund_htlc.id())? || outcoming)
            }
            TxKind::CreateMultisig => {
                let create_multisig = from_bytes::<CreateMultisig>(bytes, false)?;
                let incoming = create_multisig
                    .deposits()
                    .iter()
                    .any(|dep| dep.from() == public_key);
                if incoming || outcoming {
                    let id = multisig_id(tx_hash, index);
                    wallet.upsert_multisig(id)?;
                }
                Ok(incoming || outcoming)
            }
            TxKind::SpendMultisig => {
                let spend_multisig = from_bytes::<SpendMultisig>(bytes, false)?;
                Ok(wallet.remove_multisig(spend_multisig.id())? || outcoming)
            }
            TxKind::WithdrawFromLease => {
                let withdraw_from_lease = from_bytes::<WithdrawFromLease>(bytes, false)?;
                if outcoming {
                    let out_lease = OutLease::new(
                        withdraw_from_lease.to(),
                        withdraw_from_lease.height(),
                        withdraw_from_lease.amount(),
                    );
                    if !wallet.withdraw_from_out_lease(out_lease, withdraw_from_lease.withdraw())? {
                        warn!(self.logger, "Lease not found");
                    }
                }
                Ok(withdraw_from_lease.to() == public_key || outcoming)
            }
            TxKind::ClaimHTLC => {
                let claim_htlc = from_bytes::<ClaimHTLC>(bytes, false)?;
                Ok(wallet.remove_htlc(claim_htlc.id())? || outcoming)
            }
            TxKind::Batch => Err(ScanError::Batch),
            TxKind::Generated => Err(ScanError::Generated),
        }
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

#[derive(Debug)]
enum ScanError {
    Serialization(SerializationError),
    Wallet(WalletError),
    Batch,
    Generated,
}

impl From<SerializationError> for ScanError {
    fn from(err: SerializationError) -> Self {
        Self::Serialization(err)
    }
}

impl From<WalletError> for ScanError {
    fn from(err: WalletError) -> Self {
        Self::Wallet(err)
    }
}

impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Serialization(err) => write!(f, "{err}"),
            Self::Wallet(err) => write!(f, "{err}"),
            Self::Batch => f.write_str("Recursive batch tx"),
            Self::Generated => f.write_str("Generated as individual tx"),
        }
    }
}

impl Error for ScanError {}

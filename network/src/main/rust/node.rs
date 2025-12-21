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

use crate::blockdb::BlockDB;
use crate::blockfetcher::BlockFetcher;
use crate::coindb::CoinDB;
use crate::connection::{Connection, State};
use crate::endpoint::Endpoint;
use crate::fjall::Fjall;
use crate::packet::UnfilteredInvList;
use crate::peertable::PeerTable;
use crate::router::Router;
use crate::settings::Settings;
use crate::txfetcher::TxFetcher;
use crate::txpool::TxPool;
use blacknet_compat::{Mode, XDGDirectories, getuid, uname};
use blacknet_io::Write;
use blacknet_io::file::replace;
use blacknet_kernel::blake2b::Hash;
use blacknet_kernel::error::Error;
use blacknet_kernel::proofofstake::{
    BLOCK_RESERVED_SIZE, DEFAULT_MAX_BLOCK_SIZE, guess_initial_synchronization,
};
use blacknet_log::{LogManager, Logger, error, info, warn};
use blacknet_serialization::format::to_write;
use blacknet_time::{Milliseconds, SystemClock};
use blacknet_wallet::walletdb::WalletDB;
use core::error::Error as StdError;
use core::num::NonZero;
use fjall::Keyspace;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::atomic::AtomicU64;
use std::sync::{Arc, RwLock};
use tokio::runtime::Runtime;
use tokio::time::{Duration, sleep};

pub const NETWORK_TIMEOUT: Milliseconds = Milliseconds::from_seconds(90);
pub const PROTOCOL_VERSION: u32 = 15;
pub const MIN_PROTOCOL_VERSION: u32 = 12;

#[expect(dead_code)]
pub struct Node {
    logger: Logger,
    settings: Arc<Settings>,
    state_dir: PathBuf,
    next_peer_id: AtomicU64,
    connections: RwLock<Vec<Connection>>,
    peer_table: Arc<PeerTable>,
    router: Arc<Router>,
    block_db: BlockDB,
    block_fetcher: BlockFetcher,
    coin_db: Arc<CoinDB>,
    fjall: Keyspace,
    tx_pool: Arc<RwLock<TxPool>>,
    tx_fetcher: Arc<TxFetcher>,
    wallet_db: WalletDB,
    agent_string: String,
    prober_agent_string: String,
    agent_name: String,
    agent_version: String,
    mode: Mode,
}

impl Node {
    pub fn new(
        mode: Mode,
        dirs: &XDGDirectories,
        log_manager: &LogManager,
        runtime: &Runtime,
    ) -> Result<Arc<Self>, Box<dyn StdError>> {
        let (os_name, os_version, os_machine) = uname();
        let (agent_name, agent_version) = (mode.agent_name(), env!("CARGO_PKG_VERSION"));
        let cpu_cores = std::thread::available_parallelism()
            .map(NonZero::get)
            .unwrap_or(0);

        let logger = log_manager.logger("Node")?;
        info!(logger, "Starting up {agent_name} node {agent_version}");
        info!(logger, "CPU: {cpu_cores} cores {os_machine}");
        info!(logger, "OS: {os_name} version {os_version}");
        info!(logger, "Using config directory {}", dirs.config().display());
        info!(logger, "Using data directory {}", dirs.data().display());
        info!(logger, "Using state directory {}", dirs.state().display());

        if getuid() == 0 {
            warn!(logger, "Running as root");
        }

        let settings = Arc::new(Settings::default(&mode));
        let peer_table = PeerTable::new(&mode, dirs, log_manager, settings.clone())?;
        let fjall = Fjall::new(dirs, &settings)?;
        let coin_db = CoinDB::new(&mode, &fjall)?;
        let tx_pool = Arc::new(RwLock::new(TxPool::new(
            log_manager,
            settings.clone(),
            coin_db.clone(),
        )?));
        let node = Arc::new(Self {
            logger,
            settings: settings.clone(),
            state_dir: dirs.state().to_owned(),
            next_peer_id: AtomicU64::new(1),
            connections: RwLock::new(Vec::new()),
            peer_table: peer_table.clone(),
            router: Router::new(&mode, dirs, log_manager, runtime, &settings, peer_table)?,
            block_db: BlockDB::new(&fjall)?,
            block_fetcher: BlockFetcher::new(),
            coin_db,
            fjall,
            tx_pool: tx_pool.clone(),
            tx_fetcher: TxFetcher::new(runtime, Arc::downgrade(&tx_pool)),
            wallet_db: WalletDB::new(&mode, log_manager)?,
            agent_string: format!("/{agent_name}:{agent_version}/"),
            prober_agent_string: format!("/{agent_name}-prober:{agent_version}/"),
            agent_name: agent_name.to_owned(),
            agent_version: agent_version.to_owned(),
            mode,
        });

        runtime.spawn(node.clone().rotator());

        Ok(node)
    }

    pub fn agent_string(&self) -> &str {
        &self.agent_string
    }

    pub fn prober_agent_string(&self) -> &str {
        &self.prober_agent_string
    }

    pub fn agent_name(&self) -> &str {
        &self.agent_name
    }

    pub fn agent_version(&self) -> &str {
        &self.agent_version
    }

    pub fn nonce(&self) -> u64 {
        todo!();
    }

    pub fn is_online(&self) -> bool {
        let connections = self.connections.read().unwrap();
        connections.iter().any(Connection::is_established)
    }

    pub fn outgoing(&self) -> usize {
        let connections = self.connections.read().unwrap();
        connections
            .iter()
            .filter(|connection| connection.state() == State::OutgoingConnected)
            .count()
    }

    pub fn incoming(&self) -> usize {
        let connections = self.connections.read().unwrap();
        connections
            .iter()
            .filter(|connection| connection.state() == State::IncomingConnected)
            .count()
    }

    pub const fn connections(&self) -> &RwLock<Vec<Connection>> {
        &self.connections
    }

    pub fn listening(&self) -> &RwLock<HashSet<Endpoint>> {
        self.router.listening()
    }

    pub const fn warnings(&self) -> Vec<String> {
        //TODO
        vec![]
    }

    pub const fn max_packet_size(&self) -> u32 {
        todo!();
    }

    pub const fn min_packet_size(&self) -> u32 {
        DEFAULT_MAX_BLOCK_SIZE + BLOCK_RESERVED_SIZE
    }

    #[expect(unreachable_code)]
    pub fn is_initial_synchronization(&self) -> bool {
        self.block_fetcher.is_synchronizing()
            && guess_initial_synchronization(todo!(), SystemClock::secs(), todo!())
    }

    pub const fn block_db(&self) -> &BlockDB {
        &self.block_db
    }

    pub const fn block_fetcher(&self) -> &BlockFetcher {
        &self.block_fetcher
    }

    pub const fn coin_db(&self) -> &Arc<CoinDB> {
        &self.coin_db
    }

    pub fn peer_table(&self) -> &PeerTable {
        &self.peer_table
    }

    pub const fn tx_pool(&self) -> &Arc<RwLock<TxPool>> {
        &self.tx_pool
    }

    pub const fn tx_fetcher(&self) -> &Arc<TxFetcher> {
        &self.tx_fetcher
    }

    pub const fn wallet_db(&self) -> &WalletDB {
        &self.wallet_db
    }

    pub const fn mode(&self) -> &Mode {
        &self.mode
    }

    pub async fn broadcast_block(&self, hash: Hash, bytes: Vec<u8>) -> bool {
        match self.block_fetcher.staked_block(hash, bytes).await {
            Ok(n) => {
                if self.mode().requires_network() {
                    info!(self.logger, "Announced to {n} peers");
                }
                true
            }
            Err(error) => {
                info!(self.logger, "{error}");
                false
            }
        }
    }

    pub fn broadcast_tx(&self, hash: Hash, bytes: &[u8]) -> Result<(), Error> {
        let now = SystemClock::millis();
        let result = {
            let mut tx_pool = self.tx_pool.write().unwrap();
            tx_pool.process(hash, bytes, now, false)
        };
        if let Ok(fee) = result {
            let connections = self.connections.read().unwrap();
            for connection in connections.iter() {
                if connection.is_established()
                    && connection.check_fee_filter(bytes.len() as u32, fee)
                {
                    connection.inventory(hash)
                }
            }
        };
        result.map(|_| ())
    }

    pub fn broadcast_inv(&self, unfiltered: &UnfilteredInvList, source: Option<u64>) -> usize {
        let mut n = 0;
        let mut to_send = Vec::<Hash>::with_capacity(unfiltered.len());
        let connections = self.connections.read().unwrap();
        for connection in connections.iter() {
            if Some(connection.id()) != source && connection.is_established() {
                for i in unfiltered.iter() {
                    let &(hash, size, fee) = i;
                    if connection.check_fee_filter(size, fee) {
                        to_send.push(hash);
                    }
                }
                if !to_send.is_empty() {
                    connection.inventory_slice(&to_send);
                    to_send.clear();
                    n += 1;
                }
            }
        }
        n
    }

    async fn rotator(self: Arc<Self>) {
        loop {
            sleep(Duration::from_secs(60 * 60)).await;

            // Await while node gets online
            if !self.is_online() {
                continue;
            }

            self.peer_table.clone().rotate().await;
        }
    }

    pub fn dispose(self: Arc<Self>) {
        let mut connections = self.connections.write().unwrap();
        info!(self.logger, "Closing {} p2p connections", connections.len());
        let mut peers = Vec::with_capacity(connections.len());
        for connection in connections.iter() {
            // probers ain't interesting
            if connection.state() == State::OutgoingConnected {
                peers.push(connection.remote_endpoint());
            }
            connection.close();
        }
        connections.clear();
        info!(self.logger, "Saving node state");
        let persistent = Persistent { peers };
        if let Err(err) = replace(&self.state_dir, DATA_FILENAME, |buffered| {
            let version = DATA_VERSION.to_be_bytes();
            buffered.write_all(&version)?;
            to_write(&persistent, buffered)
        }) {
            error!(self.logger, "Can't write {DATA_FILENAME}: {err}");
        }
    }
}

const DATA_VERSION: u32 = 1;
const DATA_FILENAME: &str = "node.dat";

#[derive(Deserialize, Serialize)]
struct Persistent {
    peers: Vec<Endpoint>,
}

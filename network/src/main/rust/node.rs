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

use crate::blockdb::BlockDB;
use crate::blockfetcher::BlockFetcher;
use crate::coindb::CoinDB;
use crate::connection::{Connection, ConnectionId, State};
use crate::endpoint::Endpoint;
use crate::fjall::Fjall;
use crate::packet::{PacketKind, UnfilteredInvList};
use crate::peertable::PeerTable;
use crate::router::Router;
use crate::staker::Staker;
use crate::txfetcher::TxFetcher;
use crate::txpool::TxPool;
use blacknet_compat::config::Network as Config;
use blacknet_compat::{Mode, XDGDirectories, getuid, uname};
use blacknet_crypto::random::{Distribution, FAST_RNG, FastRNG, UniformIntDistribution};
use blacknet_io::Write;
use blacknet_io::file::replace;
use blacknet_kernel::blake2b::Hash;
use blacknet_kernel::error::Error;
use blacknet_kernel::proofofstake::{
    BLOCK_RESERVED_SIZE, DEFAULT_MAX_BLOCK_SIZE, guess_initial_synchronization, time_slot,
};
use blacknet_log::{LogManager, Logger, error, info, warn};
use blacknet_serialization::format::to_write;
use blacknet_time::{Milliseconds, Seconds, SystemClock};
use blacknet_wallet::walletdb::WalletDB;
use core::error::Error as StdError;
use core::ops::Deref;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
use tokio::io::{BufReader, BufWriter};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::runtime::{Handle, Runtime};
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::time::{Duration, sleep};

pub const NETWORK_TIMEOUT: Milliseconds = Milliseconds::with_seconds(90);
pub const PROTOCOL_VERSION: u32 = 15;
pub const MIN_PROTOCOL_VERSION: u32 = 12;

pub struct Node {
    logger: Logger,
    runtime: Handle,
    config: Arc<Config>,
    state_dir: PathBuf,
    next_connection_id: AtomicU64,
    connections: RwLock<Vec<Arc<Connection>>>,
    peer_table: Arc<PeerTable>,
    router: Arc<Router>,
    fjall: Arc<Fjall>,
    block_db: Arc<BlockDB>,
    coin_db: Arc<CoinDB>,
    block_fetcher: Arc<BlockFetcher>,
    tx_pool: Arc<RwLock<TxPool>>,
    tx_fetcher: Arc<TxFetcher>,
    wallet_db: WalletDB,
    staker: Staker,
    agent_string: String,
    prober_agent_string: String,
    agent_name: String,
    agent_version: String,
    nonce: u64,
    mode: Mode,
}

impl Node {
    pub fn new(
        mode: Mode,
        dirs: &XDGDirectories,
        log_manager: &LogManager,
        runtime: &Runtime,
        config: &Arc<Config>,
    ) -> Result<Arc<Self>, Box<dyn StdError>> {
        let (os_name, os_version, os_machine) = uname();
        let (agent_name, agent_version) = (mode.agent_name(), env!("CARGO_PKG_VERSION"));

        let logger = log_manager.logger("Node")?;
        info!(logger, "Starting up {agent_name} node {agent_version}");
        match std::thread::available_parallelism() {
            Ok(cpu_cores) => info!(logger, "CPU: {cpu_cores} cores {os_machine}"),
            Err(err) => warn!(logger, "CPU: {os_machine} ({err})"),
        }
        info!(logger, "OS: {os_name} version {os_version}");
        info!(logger, "Using config directory {}", dirs.config().display());
        info!(logger, "Using data directory {}", dirs.data().display());
        info!(logger, "Using state directory {}", dirs.state().display());

        if getuid() == 0 {
            warn!(logger, "Running as root");
        }

        let fjall = Fjall::open(dirs, config)?;
        let block_db = BlockDB::new(dirs, fjall.clone(), log_manager)?;
        let coin_db = CoinDB::new(&mode, &fjall, log_manager, block_db.clone())?;
        block_db.import(&coin_db);

        let peer_table = PeerTable::new(&mode, dirs, log_manager, config.clone())?;
        let router = Router::new(
            &mode,
            dirs,
            log_manager,
            runtime,
            config,
            peer_table.clone(),
        )?;
        let tx_pool = Arc::new(RwLock::new(TxPool::new(
            log_manager,
            config.clone(),
            coin_db.clone(),
        )?));
        let node = Arc::new(Self {
            logger,
            runtime: runtime.handle().clone(),
            config: config.clone(),
            state_dir: dirs.state().to_owned(),
            next_connection_id: AtomicU64::new(1),
            connections: RwLock::new(Vec::new()),
            peer_table,
            router,
            fjall,
            block_db,
            coin_db: coin_db.clone(),
            block_fetcher: BlockFetcher::new(runtime, config, coin_db),
            tx_pool: tx_pool.clone(),
            tx_fetcher: TxFetcher::new(runtime, Arc::downgrade(&tx_pool)),
            wallet_db: WalletDB::new(&mode, dirs, log_manager)?,
            staker: Staker::new(log_manager)?,
            agent_string: format!("/{agent_name}:{agent_version}/"),
            prober_agent_string: format!("/{agent_name}-prober:{agent_version}/"),
            agent_name: agent_name.to_owned(),
            agent_version: agent_version.to_owned(),
            nonce: Self::generate_nonce(),
            mode,
        });

        node.router.set_node(Arc::downgrade(&node));

        runtime.spawn(node.clone().rotator());

        Ok(node)
    }

    fn next_connection_id(&self) -> ConnectionId {
        let n = self.next_connection_id.fetch_add(1, Ordering::Relaxed);
        ConnectionId::new(n).expect("64-bit id is enough")
    }

    fn generate_nonce() -> u64 {
        let mut uid = UniformIntDistribution::<u64, FastRNG>::default();
        FAST_RNG.with_borrow_mut(|rng| uid.sample(rng))
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

    pub const fn nonce(&self) -> u64 {
        self.nonce
    }

    pub fn is_online(&self) -> bool {
        let connections = self.connections.read().unwrap();
        connections
            .iter()
            .map(Deref::deref)
            .any(Connection::is_established)
    }

    pub fn outgoing(&self) -> usize {
        let connections = self.connections.read().unwrap();
        connections
            .iter()
            .map(Deref::deref)
            .map(Connection::state)
            .filter(|&state| state == State::OutgoingConnected)
            .count()
    }

    pub fn incoming(&self) -> usize {
        let connections = self.connections.read().unwrap();
        connections
            .iter()
            .map(Deref::deref)
            .map(Connection::state)
            .filter(|&state| state == State::IncomingConnected)
            .count()
    }

    fn all_incoming(connections: &[Arc<Connection>]) -> usize {
        connections
            .iter()
            .map(Deref::deref)
            .map(Connection::state)
            .filter(State::is_incoming)
            .count()
    }

    pub const fn connections(&self) -> &RwLock<Vec<Arc<Connection>>> {
        &self.connections
    }

    pub fn listening(&self) -> &RwLock<HashSet<Endpoint>> {
        self.router.listening()
    }

    pub fn all_warnings(&self) -> Vec<String> {
        let mut warnings = Vec::<String>::new();
        self.block_db.warnings(&mut warnings);
        self.coin_db.warnings(&mut warnings);
        self.warnings(&mut warnings);
        warnings
    }

    pub fn warnings(&self, warnings: &mut Vec<String>) {
        let time_offset = self.time_offset();
        let state = self.coin_db.state().load();
        let pos_version = state.pos_version();
        let time_slot = time_slot(pos_version);

        if time_offset <= -time_slot || time_offset >= time_slot {
            warnings.push(
                "Please check your system clock. Many peers report different time.".to_owned(),
            )
        }
    }

    pub fn max_packet_size(&self) -> u32 {
        self.coin_db.state().load().max_block_size() + BLOCK_RESERVED_SIZE
    }

    pub const fn min_packet_size(&self) -> u32 {
        DEFAULT_MAX_BLOCK_SIZE + BLOCK_RESERVED_SIZE
    }

    pub fn is_initial_synchronization(&self) -> bool {
        let state = self.coin_db.state().load();
        let pos_version = state.pos_version();
        self.block_fetcher.is_synchronizing()
            && guess_initial_synchronization(pos_version, SystemClock::secs(), state.block_time())
    }

    pub const fn fjall(&self) -> &Arc<Fjall> {
        &self.fjall
    }

    pub const fn block_db(&self) -> &Arc<BlockDB> {
        &self.block_db
    }

    pub const fn block_fetcher(&self) -> &Arc<BlockFetcher> {
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

    pub const fn staker(&self) -> &Staker {
        &self.staker
    }

    pub const fn mode(&self) -> &Mode {
        &self.mode
    }

    fn time_offset(&self) -> Seconds {
        let min = self.config.outgoing_connections;
        let mut offsets: Vec<Seconds> = self
            .connections
            .read()
            .unwrap()
            .iter()
            .filter_map(|connection| {
                if connection.state() == State::OutgoingConnected {
                    Some(connection.time_offset())
                } else {
                    None
                }
            })
            .collect();
        if offsets.len() >= min.into() {
            offsets.sort_unstable();
            offsets[offsets.len() >> 1] // median
        } else {
            Seconds::ZERO
        }
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

    pub fn broadcast_inv(
        &self,
        unfiltered: &UnfilteredInvList,
        source: Option<ConnectionId>,
    ) -> usize {
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

    pub fn accept_connection(
        self: Arc<Self>,
        buf_reader: BufReader<OwnedReadHalf>,
        buf_writer: BufWriter<OwnedWriteHalf>,
        remote_endpoint: Endpoint,
        local_endpoint: Endpoint,
    ) {
        let id = self.next_connection_id();
        let (connection, recv_channel) = Connection::new(
            self.logger.clone(),
            self.clone(),
            remote_endpoint,
            local_endpoint,
            State::IncomingWaiting,
            id,
        );
        self.add_incoming_connection(connection, buf_reader, buf_writer, recv_channel)
    }

    pub fn add_incoming_connection(
        &self,
        connection: Arc<Connection>,
        buf_reader: BufReader<OwnedReadHalf>,
        buf_writer: BufWriter<OwnedWriteHalf>,
        recv_channel: UnboundedReceiver<(PacketKind, Vec<u8>)>,
    ) {
        let mut connections = self.connections.write().unwrap();
        if !self.have_slot(&connections) {
            info!(
                self.logger,
                "Too many connections, dropping {}",
                connection
                    .remote_endpoint()
                    .to_log(self.config.log_endpoint)
            );
            connection.close();
            return;
        }
        connections.push(connection.clone());
        connection.launch(buf_reader, buf_writer, recv_channel, &self.runtime)
    }

    fn have_slot(&self, connections: &[Arc<Connection>]) -> bool {
        if Self::all_incoming(connections) < self.config.incoming_connections as usize {
            true
        } else {
            self.evict_connection(connections)
        }
    }

    fn evict_connection(&self, connections: &[Arc<Connection>]) -> bool {
        let mut candidates = connections
            .iter()
            .filter(|c| c.state().is_incoming())
            .collect::<Vec<&Arc<Connection>>>();

        candidates.sort_by(|l, r| {
            if l.ping() != Milliseconds::ZERO {
                l.ping().cmp(&r.ping())
            } else {
                core::cmp::Ordering::Greater
            }
        });
        if candidates.len() >= 4 {
            candidates.truncate(candidates.len() - 4);
        }

        candidates.sort_by(|l, r| l.last_tx_time().cmp(&r.last_tx_time()).reverse());
        if candidates.len() >= 4 {
            candidates.truncate(candidates.len() - 4);
        }

        candidates.sort_by(|l, r| l.last_block_time().cmp(&r.last_block_time()).reverse());
        if candidates.len() >= 4 {
            candidates.truncate(candidates.len() - 4);
        }

        candidates.sort_by_key(|c| c.connected_at());
        if candidates.len() >= 4 {
            candidates.truncate(candidates.len() - 4);
        }

        //TODO network groups

        if candidates.is_empty() {
            return false;
        }

        let mut uid = UniformIntDistribution::<usize, FastRNG>::new(..candidates.len());
        let idx = FAST_RNG.with_borrow_mut(|rng| uid.sample(rng));
        let connection = candidates[idx];
        info!(
            self.logger,
            "Evicting {}",
            connection
                .remote_endpoint()
                .to_log(self.config.log_endpoint)
        );
        connection.close();
        true
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

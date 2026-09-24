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
    blockfetcher::BlockFetcher,
    connection::{Connection, ConnectionId, State},
    db::{BlockDB, CoinDB, CoinNotifier, DBVersion, Fjall},
    endpoint::Endpoint,
    packet::{BlockAnnounce, Hello, Packet, PacketKind, UnfilteredInvList, Version},
    peertable::{ContactGuard, PeerTable},
    router::{Error as RouterError, Notifier, Router},
    txfetcher::TxFetcher,
    txpool::TxPool,
};
use blacknet_compat::{
    config::Network as Config,
    feerate::FeeRate,
    {Mode, XDGDirectories},
};
use blacknet_crypto::{
    bigint::UInt256,
    random::{Distribution, FAST_RNG, UniformIntDistribution},
};
use blacknet_io::{Write, file::replace};
use blacknet_kernel::{
    amount::Amount,
    blake2b::Hash256,
    error::Error as KernelError,
    proofofstake::{
        BLOCK_RESERVED_SIZE, DEFAULT_MAX_BLOCK_SIZE, guess_initial_synchronization, time_slot,
    },
};
use blacknet_log::{LogManager, Logger, debug, error, info, warn};
use blacknet_serialization::{from_read, to_write};
use blacknet_time::{Milliseconds, Seconds, SystemClock};
use core::{error::Error, ops::Deref};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{ErrorKind, Read},
    path::{Path, PathBuf},
    sync::{
        Arc, Mutex, RwLock,
        atomic::{AtomicU64, Ordering},
    },
};
use tokio::{
    io::{BufReader, BufWriter},
    net::tcp::{OwnedReadHalf, OwnedWriteHalf},
    runtime::{Handle, Runtime},
    sync::mpsc::UnboundedReceiver,
    time::{Duration, sleep},
};

pub const NETWORK_TIMEOUT: Milliseconds = Milliseconds::with_seconds(90);
pub const PROTOCOL_VERSION: u32 = 16;
pub const MIN_PROTOCOL_VERSION: u32 = 12;

pub struct Node {
    logger: Arc<Logger>,
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
    agent_string: String,
    prober_agent_string: String,
    agent_name: String,
    agent_version: String,
    nonce: u64,
    mode: Arc<Mode>,
    queued_peers: Mutex<Vec<Endpoint>>,
}

impl Node {
    pub(super) fn new(
        mode: Arc<Mode>,
        dirs: &XDGDirectories,
        log_manager: &LogManager,
        runtime: &Runtime,
        config: &Arc<Config>,
    ) -> Result<(Arc<Self>, CoinNotifier), Box<dyn Error>> {
        let (agent_name, agent_version) = (mode.agent_name(), env!("CARGO_PKG_VERSION"));
        let logger = Arc::new(log_manager.logger("Node")?);
        let queued_peers = match Self::load(dirs.state()) {
            Ok(queued_peers) => {
                debug!(logger, "Queuing {} p2p connections", queued_peers.len());
                queued_peers
            }
            Err(err) => {
                warn!(logger, "{err}");
                Vec::new()
            }
        };

        let fjall = Fjall::open(dirs, config)?;
        let db_version = DBVersion::new(&fjall)?;
        let block_db = BlockDB::new(dirs, fjall.clone(), &db_version, log_manager)?;
        let (coin_db, coin_notifier) = CoinDB::new(
            &mode,
            fjall.clone(),
            db_version,
            log_manager,
            block_db.clone(),
        )?;
        block_db.import(&coin_db);
        let peer_table = PeerTable::new(&mode, dirs, log_manager, config.clone())?;
        let (router, router_notifier) = Router::new(
            &mode,
            dirs,
            log_manager,
            runtime,
            config,
            peer_table.clone(),
        )?;
        let tx_pool = TxPool::new(
            log_manager,
            runtime,
            config.clone(),
            &block_db,
            coin_db.clone(),
        )?;
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
            block_db: block_db.clone(),
            coin_db: coin_db.clone(),
            block_fetcher: BlockFetcher::new(log_manager, runtime, config, block_db, coin_db)?,
            tx_pool: tx_pool.clone(),
            tx_fetcher: TxFetcher::new(runtime, tx_pool.clone()),
            agent_string: format!("/{agent_name}:{agent_version}/"),
            prober_agent_string: format!("/{agent_name}-prober:{agent_version}/"),
            agent_name: agent_name.to_owned(),
            agent_version: agent_version.to_owned(),
            nonce: Self::generate_nonce(),
            mode,
            queued_peers: Mutex::new(queued_peers),
        });

        for _ in 0..config.outgoing_connections {
            runtime.spawn(node.clone().connector());
        }
        runtime.spawn(node.clone().acceptor(router_notifier));
        runtime.spawn(node.clone().prober());
        runtime.spawn(node.clone().rotator());

        Ok((node, coin_notifier))
    }

    fn next_connection_id(&self) -> ConnectionId {
        let n = self.next_connection_id.fetch_add(1, Ordering::Relaxed);
        ConnectionId::new(n).expect("64-bit id is enough")
    }

    fn generate_nonce() -> u64 {
        let mut uid = UniformIntDistribution::<u64>::default();
        FAST_RNG.with_borrow_mut(|rng| uid.sample(rng))
    }

    pub fn agent_string(&self) -> &str {
        &self.agent_string
    }

    pub(super) fn prober_agent_string(&self) -> &str {
        &self.prober_agent_string
    }

    pub fn agent_name(&self) -> &str {
        &self.agent_name
    }

    pub fn agent_version(&self) -> &str {
        &self.agent_version
    }

    pub(super) const fn nonce(&self) -> u64 {
        self.nonce
    }

    pub(super) fn is_online(&self) -> bool {
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

    pub(super) fn warnings(&self, warnings: &mut Vec<String>) {
        self.block_db.warnings(warnings);
        self.coin_db.warnings(warnings);

        let time_offset = self.time_offset();
        let (ref state, _) = **self.coin_db.state().load();
        let pos_version = state.pos_version();
        let time_slot = time_slot(pos_version);

        if time_offset <= -time_slot || time_offset >= time_slot {
            warnings.push(
                "Please check your system clock. Many peers report different time.".to_owned(),
            )
        }
    }

    pub(super) fn max_packet_size(&self) -> u32 {
        self.coin_db.state().load().0.max_block_size() + BLOCK_RESERVED_SIZE
    }

    pub(super) const fn min_packet_size(&self) -> u32 {
        DEFAULT_MAX_BLOCK_SIZE + BLOCK_RESERVED_SIZE
    }

    pub(super) fn is_initial_synchronization(&self) -> bool {
        let (ref state, _) = **self.coin_db.state().load();
        let pos_version = state.pos_version();
        self.block_fetcher.is_synchronizing()
            && guess_initial_synchronization(pos_version, SystemClock::secs(), state.block_time())
    }

    pub(super) const fn config(&self) -> &Arc<Config> {
        &self.config
    }

    pub const fn runtime(&self) -> &Handle {
        &self.runtime
    }

    pub const fn fjall(&self) -> &Arc<Fjall> {
        &self.fjall
    }

    pub const fn block_db(&self) -> &Arc<BlockDB> {
        &self.block_db
    }

    pub(super) const fn block_fetcher(&self) -> &Arc<BlockFetcher> {
        &self.block_fetcher
    }

    pub const fn coin_db(&self) -> &Arc<CoinDB> {
        &self.coin_db
    }

    pub const fn peer_table(&self) -> &Arc<PeerTable> {
        &self.peer_table
    }

    pub const fn router(&self) -> &Arc<Router> {
        &self.router
    }

    pub const fn tx_pool(&self) -> &Arc<RwLock<TxPool>> {
        &self.tx_pool
    }

    pub(super) const fn tx_fetcher(&self) -> &Arc<TxFetcher> {
        &self.tx_fetcher
    }

    pub(super) const fn mode(&self) -> &Arc<Mode> {
        &self.mode
    }

    fn time_offset(&self) -> Seconds {
        let min = self.config.outgoing_connections as usize;
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
        let n = offsets.len();
        if n != 0 && n >= min {
            offsets.sort_unstable();
            offsets[offsets.len() >> 1] // median
        } else {
            Seconds::ZERO
        }
    }

    pub(super) fn announce_block(
        &self,
        hash: Hash256,
        cumulative_difficulty: UInt256,
        source: Option<ConnectionId>,
    ) -> usize {
        self.broadcast_packet(
            &BlockAnnounce::new(hash, cumulative_difficulty),
            |connection| {
                Some(connection.id()) != source
                    && connection.state().is_established()
                    && connection.last_block().load().cumulative_difficulty()
                        < cumulative_difficulty
            },
        )
    }

    pub(super) async fn broadcast_block(&self, hash: Hash256, bytes: Box<[u8]>) -> bool {
        match self.block_fetcher.staked_block(hash, bytes).await {
            Ok(()) => {
                let (ref state, _) = **self.coin_db.state().load();
                let n = self.announce_block(hash, state.cumulative_difficulty(), None);
                if self.mode().requires_network() {
                    info!(self.logger, "Announced to {n} peers");
                }
                true
            }
            Err(err) => {
                info!(self.logger, "{err}");
                false
            }
        }
    }

    pub fn broadcast_tx(&self, hash: Hash256, bytes: &[u8]) -> Result<(), KernelError> {
        let now = SystemClock::millis();
        let result = {
            let mut tx_pool = self.tx_pool.write().unwrap();
            tx_pool.process(hash, bytes, now, false)
        };
        if let Ok(fee) = result {
            let connections = self.connections.read().unwrap();
            for connection in connections.iter() {
                if connection.is_established()
                    && connection.check_fee_filter(fee, bytes.len() as u32)
                {
                    connection.inventory(hash)
                }
            }
        };
        result.map(|_| ())
    }

    pub(super) fn broadcast_inv(
        &self,
        unfiltered: &UnfilteredInvList,
        source: Option<ConnectionId>,
    ) -> usize {
        let mut n = 0;
        let mut to_send = Vec::<Hash256>::with_capacity(unfiltered.len());
        let connections = self.connections.read().unwrap();
        for connection in connections.iter() {
            if Some(connection.id()) != source && connection.is_established() {
                for i in unfiltered.iter() {
                    let &(hash, size, fee) = i;
                    if connection.check_fee_filter(fee, size) {
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

    fn broadcast_packet<T: Packet, F: FnMut(&&Arc<Connection>) -> bool>(
        &self,
        packet: &T,
        filter: F,
    ) -> usize {
        let mut n = 0;
        self.connections
            .read()
            .unwrap()
            .iter()
            .filter(filter)
            .for_each(|connection| {
                connection.send_packet(packet);
                n += 1;
            });
        n
    }

    fn accept_connection(
        self: &Arc<Self>,
        buf_reader: BufReader<OwnedReadHalf>,
        buf_writer: BufWriter<OwnedWriteHalf>,
        remote_endpoint: Endpoint,
        local_endpoint: Endpoint,
    ) {
        let id = self.next_connection_id();
        let logger = self
            .logger
            .fork_with_name(Some(format!("Peer-{id}")))
            .unwrap_or_else(|_| self.logger.clone());
        let (connection, recv_channel) = Connection::new(
            logger,
            self.clone(),
            remote_endpoint,
            local_endpoint,
            State::IncomingWaiting,
            id,
        );
        let mut connections = self.connections.write().unwrap();
        if !self.have_slot(&connections) {
            info!(
                self.logger,
                "Too many connections, dropping {}",
                connection.log_name()
            );
            connection.close();
            return;
        }
        connections.push(connection.clone());
        self.runtime
            .spawn(connection.run(buf_reader, buf_writer, recv_channel));
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

        let mut uid = UniformIntDistribution::<usize>::new(..candidates.len());
        let idx = FAST_RNG.with_borrow_mut(|rng| uid.sample(rng));
        let connection = candidates[idx];
        info!(self.logger, "Evicting {}", connection.log_name());
        connection.close();
        true
    }

    async fn acceptor(self: Arc<Self>, mut notifier: Notifier) {
        while let Some(notification) = notifier.recv().await {
            self.accept_connection(
                notification.0,
                notification.1,
                notification.2,
                notification.3,
            )
        }
    }

    async fn connect_to(
        self: &Arc<Self>,
        remote_endpoint: Endpoint,
        v2: bool,
        prober: bool,
    ) -> Result<
        (
            Arc<Connection>,
            BufReader<OwnedReadHalf>,
            BufWriter<OwnedWriteHalf>,
            UnboundedReceiver<(PacketKind, Vec<u8>)>,
        ),
        RouterError,
    > {
        let (buf_reader, buf_writer, local_endpoint) = self.router.connect(remote_endpoint).await?;
        let state = if prober {
            State::ProberWaiting
        } else {
            State::OutgoingWaiting
        };
        let id = self.next_connection_id();
        let logger = self
            .logger
            .fork_with_name(Some(format!("Peer-{id}")))
            .unwrap_or_else(|_| self.logger.clone());
        let (connection, recv_channel) = Connection::new(
            logger,
            self.clone(),
            remote_endpoint,
            local_endpoint,
            state,
            id,
        );
        let mut connections = self.connections.write().unwrap();
        connections.push(connection.clone());
        if v2 {
            self.send_hello(&connection);
        } else {
            self.send_version(
                &connection,
                if connection.state() == State::OutgoingWaiting
                    && !connection.remote_endpoint().is_permissionless()
                {
                    self.nonce
                } else {
                    0
                },
                prober,
            );
        }
        Ok((connection, buf_reader, buf_writer, recv_channel))
    }

    fn send_version(&self, connection: &Connection, nonce: u64, prober: bool) {
        connection.send_packet(&if prober {
            Version::new(
                self.mode.network_magic(),
                PROTOCOL_VERSION,
                SystemClock::secs(),
                nonce,
                self.prober_agent_string.clone(),
                Amount::MAX,
                BlockAnnounce::default(),
            )
        } else {
            let (ref state, _) = **self.coin_db.state().load();
            Version::new(
                self.mode.network_magic(),
                PROTOCOL_VERSION,
                SystemClock::secs(),
                nonce,
                self.agent_string.clone(),
                Amount::new(self.tx_pool.read().unwrap().min_fee_rate().into()),
                BlockAnnounce::new(state.block_hash(), state.cumulative_difficulty()),
            )
        })
    }

    fn send_hello(&self, connection: &Connection) {
        let state = connection.state();
        let mut hello = Hello::new();
        hello.set_magic(self.mode.network_magic());
        hello.set_version(PROTOCOL_VERSION);
        if state == State::OutgoingWaiting && !connection.remote_endpoint().is_permissionless() {
            hello.set_nonce(self.nonce);
        }
        hello.set_agent(if state == State::ProberWaiting {
            &self.prober_agent_string
        } else {
            &self.agent_string
        });
        hello.set_fee_filter(if state == State::ProberWaiting {
            FeeRate::MAX
        } else {
            self.tx_pool.read().unwrap().min_fee_rate()
        });

        connection.send_packet(&hello);
        if state != State::ProberWaiting {
            let (ref state, _) = **self.coin_db.state().load();
            connection.send_packet(&BlockAnnounce::new(
                state.block_hash(),
                state.cumulative_difficulty(),
            ));
        }
    }

    pub async fn add_peer(
        self: Arc<Self>,
        endpoint: ContactGuard,
        time: Milliseconds,
        prober: bool,
    ) {
        let mut connection = self.connect_to(*endpoint, true, prober).await;
        let mut t = None;
        if let Ok((conn, buf_reader, buf_writer, recv_channel)) = connection {
            t = Some(conn.clone());
            conn.clone().run(buf_reader, buf_writer, recv_channel).await;
            // try v1 if accepted without reply
            if conn.total_bytes_read() == 0 {
                connection = self.connect_to(*endpoint, false, prober).await;
                if let Ok((conn, buf_reader, buf_writer, recv_channel)) = connection {
                    t = Some(conn.clone());
                    conn.run(buf_reader, buf_writer, recv_channel).await;
                }
            }
        }

        if self.is_online() {
            let waiting = t.map(|conn| conn.state() == State::OutgoingWaiting);
            match waiting {
                Some(true) | None => {
                    self.peer_table.failed(*endpoint, time);
                }
                Some(false) => {}
            }
        }
    }

    async fn connector(self: Arc<Self>) {
        loop {
            let Some(endpoint) = self.queued_peers.lock().unwrap().pop() else {
                break;
            };
            let Some(endpoint) = self.peer_table.try_contact(endpoint) else {
                continue;
            };
            let time = SystemClock::millis();
            self.clone().add_peer(endpoint, time, false).await;
        }

        loop {
            let Some(endpoint) = self.peer_table.candidate(|_, _| true) else {
                let outgoing = self.outgoing();
                info!(
                    self.logger,
                    "PeerTable has no candidates, {outgoing}/{} connections",
                    self.config.outgoing_connections
                );
                sleep(Milliseconds::with_minutes(15).try_into().unwrap()).await;
                continue;
            };

            let time = SystemClock::millis();
            self.clone().add_peer(endpoint, time, false).await;

            let x = Milliseconds::with_seconds(4) - (SystemClock::millis() - time);
            if x > Milliseconds::ZERO {
                // 請在繼續之前等待或延遲
                sleep(x.try_into().unwrap()).await;
            }
        }
    }

    async fn prober(self: Arc<Self>) {
        loop {
            sleep(Milliseconds::with_minutes(4).try_into().unwrap()).await;

            // Await peer endpoint announce
            if self.peer_table.len() < self.peer_table.max_len() / 2 {
                continue;
            }

            // Await while connectors are working
            if self.outgoing() < self.config.outgoing_connections as usize {
                continue;
            }

            let time = SystemClock::millis();
            let Some(endpoint) = self
                .peer_table
                .candidate(|_, entry| time > entry.last_try() + Milliseconds::with_hours(4))
            else {
                continue;
            };

            self.clone().add_peer(endpoint, time, true).await;
        }
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

    pub(super) fn dispose(&self) {
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

    fn load(state_dir: &Path) -> Result<Vec<Endpoint>, Box<dyn Error>> {
        let mut file = match File::open(state_dir.join(DATA_FILENAME)) {
            Ok(file) => std::io::BufReader::new(file),
            Err(err) => {
                if err.kind() == ErrorKind::NotFound {
                    // first run or unlinked file
                    return Ok(Vec::new());
                } else {
                    return Err(Box::new(err));
                }
            }
        };
        let mut version = [0u8; 4];
        file.read_exact(&mut version)?;
        let version = u32::from_be_bytes(version);
        if version != DATA_VERSION {
            return Err(format!("Unknown {DATA_FILENAME} version {version}").into());
        }
        let deserialized: Vec<Endpoint> = from_read(&mut file)?;
        Ok(deserialized)
    }
}

const DATA_VERSION: u32 = 1;
const DATA_FILENAME: &str = "node.dat";

#[derive(Deserialize, Serialize)]
struct Persistent {
    peers: Vec<Endpoint>,
}

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
    connection::{Connection, ConnectionId},
    db::{BlockDB, CoinDB, State},
    packet::{BlockAnnounce, Blocks, ConsensusFault, GetBlocks},
};
use blacknet_compat::config::Network as Config;
use blacknet_crypto::bigint::UInt256;
use blacknet_kernel::{
    blake2b::Hash,
    block::Block,
    error::{Error, Result},
    proofofstake::{ROLLBACK_LIMIT, guess_initial_synchronization},
};
use blacknet_log::{Error as LogError, LogManager, Logger, debug, error, info};
use blacknet_time::{Milliseconds, SystemClock};
use core::{cmp::max, fmt};
use std::sync::{Arc, OnceLock, RwLock};
use tokio::{
    runtime::Runtime,
    select,
    sync::{mpsc, oneshot, oneshot::error::RecvError},
    time::timeout,
};
use tokio_util::sync::CancellationToken;

pub struct BlockFetcher {
    logger: Logger,
    staker_sender: mpsc::UnboundedSender<(Hash, Box<[u8]>, oneshot::Sender<Result<()>>)>,
    announces_sender: mpsc::Sender<(Arc<Connection>, BlockAnnounce)>,
    deferred_sender: mpsc::Sender<(Arc<Connection>, Blocks)>,
    request: RwLock<Option<RequestSender>>,
    block_db: Arc<BlockDB>,
    coin_db: Arc<CoinDB>,
}

impl BlockFetcher {
    pub fn new(
        log_manager: &LogManager,
        runtime: &Runtime,
        config: &Arc<Config>,
        block_db: Arc<BlockDB>,
        coin_db: Arc<CoinDB>,
    ) -> Result<Arc<Self>, LogError> {
        let size = config.incoming_connections as usize + config.outgoing_connections as usize;
        let size = max(size, 1); // mpsc bounded channel requires buffer > 0
        let (staker_sender, staker_receiver) = mpsc::unbounded_channel();
        let (announces_sender, announces_receiver) = mpsc::channel(size);
        let (deferred_sender, deferred_receiver) = mpsc::channel(size);
        let block_fetcher = Arc::new(Self {
            logger: log_manager.logger("BlockFetcher")?,
            staker_sender,
            announces_sender,
            deferred_sender,
            block_db,
            coin_db,
            request: RwLock::new(None),
        });

        runtime.spawn(block_fetcher.clone().run(
            staker_receiver,
            announces_receiver,
            deferred_receiver,
        ));

        Ok(block_fetcher)
    }

    pub fn is_synchronizing(&self) -> bool {
        self.request.read().unwrap().is_some()
    }

    pub fn disconnected(&self, connection: &Connection) {
        let Some(ref request) = *self.request.read().unwrap() else {
            return;
        };

        if request.connection_id != connection.id() {
            return;
        }

        request.cancel(RequestError::ConnectionClosed);
    }

    pub fn offer(&self, connection: &Arc<Connection>, block_announce: BlockAnnounce) {
        if block_announce.cumulative_difficulty()
            <= self.coin_db.state().load().cumulative_difficulty()
        {
            return;
        }

        let _ = self
            .announces_sender
            .try_send((connection.clone(), block_announce));
    }

    pub async fn staked_block(&self, hash: Hash, bytes: Box<[u8]>) -> Result<()> {
        let (sender, receiver) = oneshot::channel();
        let _ = self.staker_sender.send((hash, bytes, sender));
        if let Some(ref request) = *self.request.read().unwrap() {
            request.cancel(RequestError::Staked);
        }
        receiver.await.unwrap()
    }

    pub fn consensus_fault(&self, connection: &Connection, _consensus_fault: ConsensusFault) {
        if !connection.requested_blocks() {
            connection.dos("Unexpected packet ConsensusFault");
            return;
        }

        connection.close();

        let Some(ref request) = *self.request.read().unwrap() else {
            return;
        };

        if request.connection_id != connection.id() {
            return;
        }

        request.cancel(RequestError::LongerThanCheckpoint);
    }

    pub fn blocks(&self, connection: &Arc<Connection>, blocks: Blocks) {
        let requested_difficulty = connection.swap_requested_difficulty(UInt256::ZERO);

        if requested_difficulty == UInt256::ZERO {
            connection.dos("Unexpected packet Blocks");
            return;
        }

        let mut request = self.request.write().unwrap();
        match *request {
            Some(ref request_ref) => {
                if request_ref.connection_id != connection.id() {
                    let _ = self.deferred_sender.try_send((connection.clone(), blocks));
                } else {
                    request.take().unwrap().complete(blocks);
                }
            }
            None => {
                let _ = self.deferred_sender.try_send((connection.clone(), blocks));
            }
        }
    }

    async fn run(
        self: Arc<Self>,
        mut staker_receiver: mpsc::UnboundedReceiver<(
            Hash,
            Box<[u8]>,
            oneshot::Sender<Result<()>>,
        )>,
        mut announces_receiver: mpsc::Receiver<(Arc<Connection>, BlockAnnounce)>,
        mut deferred_receiver: mpsc::Receiver<(Arc<Connection>, Blocks)>,
    ) {
        loop {
            select! {
                biased;
                Some(
                    (hash, bytes, sender)
                ) = staker_receiver.recv() => self.process_staked(
                    hash, bytes, sender
                ).await,
                Some(
                    (connection, blocks)
                ) = deferred_receiver.recv() => self.process_deferred(
                    connection, blocks
                ).await,
                Some(
                    (connection, announce)
                ) = announces_receiver.recv() => self.process_announce(
                    connection, announce
                ).await,
                else => break,
            }
        }
        info!(self.logger, "Interrupting BlockFetcher");
    }

    async fn process_staked(
        &self,
        hash: Hash,
        bytes: Box<[u8]>,
        sender: oneshot::Sender<Result<()>>,
    ) {
        let result = self.block_db.process(&self.coin_db, hash, bytes);
        let _ = sender.send(result);
    }

    /// Blocks were received after timeout. During lags, processing these helps to stay in sync.
    async fn process_deferred(&self, connection: Arc<Connection>, answer: Blocks) {
        let (hashes, blocks) = answer.into();
        if !blocks.is_empty() {
            info!(
                self.logger,
                "Mongering {} deferred blocks from {}",
                blocks.len(),
                connection.log_name(),
            );
            for i in blocks {
                let Some(hash) = Block::compute_hash(&i) else {
                    connection.dos("Unhashable block");
                    return;
                };
                let result = self.block_db.process(&self.coin_db, hash, i);
                match result {
                    Ok(()) => {
                        // Continue catching up
                        info!(self.logger, "Accepted {hash}");
                        continue;
                    }
                    Err(Error::AlreadyHave(_)) => {
                        // Perhaps sequent blocks will be useful
                        debug!(self.logger, "Already have {hash}");
                        continue;
                    }
                    Err(Error::InFuture(msg)) | Err(Error::Invalid(msg)) => {
                        // No way
                        connection.dos(&msg);
                        break;
                    }
                    Err(Error::NotReachableVertex(_)) => {
                        // 何罵
                        debug!(self.logger, "Not reachable vertex {hash}");
                        break;
                    }
                }
            }
        } else if hashes.is_empty() {
            //XXX Can be used somehow?
            debug!(self.logger, "Skipped {} deferred hashes", hashes.len());
        } else {
            // Should not happen
            error!(self.logger, "Invalid packet Blocks");
        }
    }

    async fn process_announce(&self, connection: Arc<Connection>, announce: BlockAnnounce) {
        if connection.requested_blocks() {
            return;
        }

        let mut state = self.coin_db.state().load();
        if announce.cumulative_difficulty() <= state.cumulative_difficulty() {
            return;
        }

        if self.block_db.is_rejected(announce.hash()) {
            connection.dos("Rejected block");
            return;
        }

        info!(self.logger, "Fetching {}", announce.hash());
        let mut session = Session::new(state.block_hash());

        'request_loop: loop {
            let receiver = self.request_blocks(
                &session,
                &state,
                &connection,
                announce.cumulative_difficulty(),
            );
            match receiver.run().await {
                Ok(answer) => {
                    if !answer.blocks().is_empty() {
                        if !self.process_blocks(answer, &state, &mut session, &connection) {
                            break;
                        }
                        state = self.coin_db.state().load();
                        if announce.cumulative_difficulty() > state.cumulative_difficulty() {
                            continue;
                        } else {
                            break;
                        }
                    } else if !answer.hashes().is_empty() {
                        if session.rollback_to != Hash::ZERO || session.connected_blocks != 0 {
                            connection.dos("Unexpected rollback");
                            break;
                        }
                        let mut prev = state.rolling_checkpoint();
                        for &hash in answer.hashes() {
                            if self.block_db.is_rejected(hash) {
                                connection.dos("Rejected block");
                                break 'request_loop;
                            }
                            let Some(block_index) = self.block_db.indexes.get(hash) else {
                                break;
                            };
                            if block_index.height() < state.height() - ROLLBACK_LIMIT as u32 {
                                connection.dos(&format!("Rollback to {}", block_index.height()));
                                break 'request_loop;
                            }
                            prev = hash;
                        }
                        session.rollback_to = prev;
                        continue;
                    } else {
                        break;
                    }
                }
                Err(RequestError::TimedOut) => {
                    connection.dos("Fetching cancelled: Request timed out");
                    break;
                }
                Err(err) => {
                    info!(self.logger, "Fetching cancelled: {err}");
                    break;
                }
            }
        }

        state = self.coin_db.state().load();
        if !session.undo_rollback.is_empty() {
            if session.undo_difficulty >= state.cumulative_difficulty() {
                info!(
                    self.logger,
                    "Reconnecting {} blocks",
                    session.undo_rollback.len()
                );
                let to_remove = self
                    .coin_db
                    .undo_rollback(session.rollback_to, session.undo_rollback);
                self.block_db.remove(to_remove);
            } else {
                debug!(
                    self.logger,
                    "Removing {} blocks from db",
                    session.undo_rollback.len()
                );
                self.block_db.remove(session.undo_rollback);
            }
        }

        state = self.coin_db.state().load();
        if state.block_hash() != session.original_chain {
            connection.node().announce_block(
                state.block_hash(),
                state.cumulative_difficulty(),
                Some(connection.id()),
            );
            connection.set_last_block_time(connection.last_packet_time());
        }

        if connection.is_closed() {
            info!(
                self.logger,
                "Fetched {} blocks from disconnected {}",
                session.connected_blocks,
                connection.log_name()
            );
        } else {
            info!(
                self.logger,
                "Fetched {} blocks from {}",
                session.connected_blocks,
                connection.log_name()
            );
        }

        *self.request.write().unwrap() = None;
    }

    fn process_blocks(
        &self,
        answer: Blocks,
        state: &State,
        session: &mut Session,
        connection: &Connection,
    ) -> bool {
        let (_, blocks) = answer.into();
        let n = blocks.len();
        if session.rollback_to != Hash::ZERO && session.undo_rollback.is_empty() {
            let undo_difficulty = state.cumulative_difficulty();
            let undo_rollback = self.coin_db.rollback_to(session.rollback_to);
            info!(self.logger, "Disconnected {} blocks", undo_rollback.len());
            session.undo_difficulty = undo_difficulty;
            session.undo_rollback = undo_rollback;
        }
        for block in blocks {
            let Some(hash) = Block::compute_hash(&block) else {
                connection.dos("Unhashable block");
                return false;
            };
            if session.undo_rollback.contains(&hash) {
                connection.dos("Rerolling block");
                return false;
            }
            let result = self.block_db.process(&self.coin_db, hash, block);
            if let Err(err) = result {
                connection.dos(&err.to_string());
                return false;
            }
        }
        if session.undo_rollback.is_empty() {
            self.coin_db.prune()
        }
        session.connected_blocks += n;
        if n >= 10 {
            info!(self.logger, "Connected {n} blocks");
        }
        true
    }

    fn request_blocks(
        &self,
        session: &Session,
        state: &State,
        connection: &Connection,
        difficulty: UInt256,
    ) -> RequestReceiver {
        let block_hash = if session.rollback_to != Hash::ZERO && session.undo_rollback.is_empty() {
            session.rollback_to
        } else {
            state.block_hash()
        };
        let (sender, receiver) = new_request(connection.id(), self.timeout(state));
        *self.request.write().unwrap() = Some(sender);
        connection.set_requested_difficulty(difficulty);
        connection.send_packet(&GetBlocks::new(block_hash, state.rolling_checkpoint()));
        receiver
    }

    fn timeout(&self, state: &State) -> Milliseconds {
        let pos_version = state.pos_version();
        if !guess_initial_synchronization(pos_version, SystemClock::secs(), state.block_time()) {
            Milliseconds::new(4000)
        } else {
            Milliseconds::new(10000)
        }
    }
}

struct RequestSender {
    connection_id: ConnectionId,
    sender: oneshot::Sender<Blocks>,
    cancel: CancellationToken,
    cancel_reason: Arc<OnceLock<RequestError>>,
}

impl RequestSender {
    fn complete(self, blocks: Blocks) {
        let _ = self.sender.send(blocks);
    }

    fn cancel(&self, reason: RequestError) {
        self.cancel.cancel();
        let _ = self.cancel_reason.set(reason);
    }
}

struct RequestReceiver {
    timeout: Milliseconds,
    receiver: oneshot::Receiver<Blocks>,
    cancel: CancellationToken,
    cancel_reason: Arc<OnceLock<RequestError>>,
}

impl RequestReceiver {
    async fn run(self) -> Result<Blocks, RequestError> {
        select! {
            _ = self.cancel.cancelled() => {
                Err(*self.cancel_reason.wait())
            }
            res = timeout(self.timeout.try_into().unwrap(), self.receiver) => {
                match res {
                    Ok(output) => Ok(output?),
                    Err(_) => Err(RequestError::TimedOut),
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum RequestError {
    ConnectionClosed,
    LongerThanCheckpoint,
    Staked,
    TimedOut,
    Dropped,
}

impl From<RecvError> for RequestError {
    fn from(_: RecvError) -> Self {
        Self::Dropped
    }
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConnectionClosed => f.write_str("Connection closed"),
            Self::LongerThanCheckpoint => f.write_str("Dipath longer than the rolling checkpoint"),
            Self::Staked => f.write_str("Staked new block"),
            Self::TimedOut => f.write_str("Request timed out"),
            Self::Dropped => f.write_str("Request already dropped"),
        }
    }
}

impl core::error::Error for RequestError {}

fn new_request(
    connection_id: ConnectionId,
    timeout: Milliseconds,
) -> (RequestSender, RequestReceiver) {
    let (sender, receiver) = oneshot::channel();
    let cancel = CancellationToken::new();
    let cancel_reason = Arc::new(OnceLock::new());
    (
        RequestSender {
            connection_id,
            sender,
            cancel: cancel.clone(),
            cancel_reason: cancel_reason.clone(),
        },
        RequestReceiver {
            timeout,
            receiver,
            cancel,
            cancel_reason,
        },
    )
}

struct Session {
    original_chain: Hash,
    connected_blocks: usize,
    rollback_to: Hash,
    undo_difficulty: UInt256,
    undo_rollback: Vec<Hash>,
}

impl Session {
    const fn new(original_chain: Hash) -> Self {
        Self {
            original_chain,
            connected_blocks: 0,
            rollback_to: Hash::ZERO,
            undo_difficulty: UInt256::ZERO,
            undo_rollback: Vec::new(),
        }
    }
}

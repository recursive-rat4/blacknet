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

use crate::coindb::CoinDB;
use crate::connection::{Connection, ConnectionId};
use crate::packet::{BlockAnnounce, Blocks, ConsensusFault};
use blacknet_compat::config::Network as Config;
use blacknet_crypto::bigint::UInt256;
use blacknet_kernel::blake2b::Hash;
use blacknet_kernel::error::Result;
use blacknet_kernel::proofofstake::guess_initial_synchronization;
use blacknet_time::{Milliseconds, SystemClock};
use std::sync::{Arc, OnceLock, RwLock};
use tokio::runtime::Runtime;
use tokio::select;
use tokio::sync::{mpsc, oneshot};
use tokio::time::timeout;
use tokio_util::sync::CancellationToken;

pub struct BlockFetcher {
    announces_sender: mpsc::Sender<(ConnectionId, BlockAnnounce)>,
    request: RwLock<Option<Request>>,
    coin_db: Arc<CoinDB>,
}

impl BlockFetcher {
    pub fn new(runtime: &Runtime, config: &Arc<Config>, coin_db: Arc<CoinDB>) -> Arc<Self> {
        let size = config.incoming_connections as usize + config.outgoing_connections as usize;
        let (announces_sender, announces_receiver) = mpsc::channel(size);
        let block_fetcher = Arc::new(Self {
            announces_sender,
            coin_db,
            request: RwLock::new(None),
        });

        runtime.spawn(block_fetcher.clone().implementation(announces_receiver));

        block_fetcher
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

        request.cancel("Connection closed");
    }

    pub fn offer(&self, connection: &Connection, block_announce: BlockAnnounce) {
        if block_announce.cumulative_difficulty()
            <= self.coin_db.state().load().cumulative_difficulty()
        {
            return;
        }

        let _ = self
            .announces_sender
            .try_send((connection.id(), block_announce));
    }

    pub async fn staked_block(&self, _hash: Hash, _bytes: Vec<u8>) -> Result<usize> {
        todo!();
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

        request.cancel("Dipath longer than the rolling checkpoint");
    }

    pub fn blocks(&self, connection: &Connection, blocks: Blocks) {
        let requested_difficulty = connection.swap_requested_difficulty(UInt256::ZERO);

        if requested_difficulty == UInt256::ZERO {
            connection.dos("Unexpected packet Blocks");
            return;
        }

        let mut request = self.request.write().unwrap();
        match *request {
            Some(ref request_ref) => {
                if request_ref.connection_id != connection.id() {
                    //TODO defer
                } else {
                    request.take().unwrap().complete(blocks);
                }
            }
            None => {
                //TODO defer
            }
        }
    }

    async fn implementation(
        self: Arc<Self>,
        mut announces_receiver: mpsc::Receiver<(ConnectionId, BlockAnnounce)>,
    ) {
        loop {
            #[expect(unused_variables)]
            let announce = announces_receiver.recv();
            todo!();
        }
    }

    #[expect(dead_code)]
    fn timeout(&self) -> Milliseconds {
        let state = self.coin_db.state().load();
        let pos_version = state.pos_version();
        if !guess_initial_synchronization(pos_version, SystemClock::secs(), state.block_time()) {
            Milliseconds::new(4000)
        } else {
            Milliseconds::new(10000)
        }
    }
}

#[expect(dead_code)]
struct Request {
    connection_id: ConnectionId,
    timeout: Milliseconds,
    sender: oneshot::Sender<Blocks>,
    cancel: CancellationToken,
    cancel_reason: OnceLock<&'static str>,
}

impl Request {
    #[expect(dead_code)]
    fn new(
        connection_id: ConnectionId,
        timeout: Milliseconds,
    ) -> (Self, oneshot::Receiver<Blocks>) {
        let (sender, reveiver) = oneshot::channel();
        (
            Self {
                connection_id,
                timeout,
                sender,
                cancel: CancellationToken::new(),
                cancel_reason: OnceLock::new(),
            },
            reveiver,
        )
    }

    fn complete(self, blocks: Blocks) {
        let _ = self.sender.send(blocks);
    }

    fn cancel(&self, reason: &'static str) {
        self.cancel.cancel();
        let _ = self.cancel_reason.set(reason);
    }

    #[expect(dead_code)]
    async fn run(&self, future: impl Future<Output = Blocks>) -> Result<Blocks, &'static str> {
        select! {
            _ = self.cancel.cancelled() => {
                Err(self.cancel_reason.wait())
            }
            res = timeout(self.timeout.try_into().unwrap(), future) => {
                match res {
                    Ok(output) => Ok(output),
                    Err(_) => Err("Request timed out"),
                }
            }
        }
    }
}

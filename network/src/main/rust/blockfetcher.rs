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
use crate::connection::Connection;
use crate::packet::{BlockAnnounce, Blocks, ConsensusFault};
use crate::settings::Settings;
use blacknet_crypto::bigint::UInt256;
use blacknet_kernel::blake2b::Hash;
use blacknet_kernel::error::Result;
use std::sync::Arc;
use tokio::sync::mpsc;

#[expect(dead_code)]
pub struct BlockFetcher {
    connection_id: u64,
    announces_receiver: mpsc::Receiver<(u64, BlockAnnounce)>,
    announces_sender: mpsc::Sender<(u64, BlockAnnounce)>,
    coin_db: Arc<CoinDB>,
}

impl BlockFetcher {
    pub fn new(coin_db: Arc<CoinDB>, settings: &Arc<Settings>) -> Self {
        let size = settings.incoming_connections as usize + settings.outgoing_connections as usize;
        let (announces_sender, announces_receiver) = mpsc::channel(size);
        Self {
            connection_id: 0,
            announces_receiver,
            announces_sender,
            coin_db,
        }
    }

    pub const fn is_synchronizing(&self) -> bool {
        self.connection_id != 0
    }

    pub fn disconnected(&self, connection: &Connection) {
        if self.connection_id != connection.id() {
            return;
        }

        todo!();
    }

    pub fn offer(&self, connection: &Connection, block_announce: BlockAnnounce) {
        if block_announce.cumulative_difficulty() <= self.coin_db.state().cumulative_difficulty() {
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

        if self.connection_id != connection.id() {
            return;
        }

        todo!();
    }

    pub fn blocks(&self, connection: &Connection, _blocks: Blocks) {
        let requested_difficulty = connection.swap_requested_difficulty(UInt256::ZERO);

        if requested_difficulty == UInt256::ZERO {
            connection.dos("Unexpected packet Blocks");
            return;
        }

        todo!();
    }
}

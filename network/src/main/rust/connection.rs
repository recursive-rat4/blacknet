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

use crate::endpoint::Endpoint;
use crate::node::Node;
use crate::packet::{BlockAnnounce, Packet};
use blacknet_crypto::bigint::UInt256;
use blacknet_kernel::amount::Amount;
use blacknet_log::{Logger, info};
use blacknet_time::{Milliseconds, Seconds};
use std::sync::{
    Arc,
    atomic::{AtomicU8, Ordering},
};

#[expect(dead_code)]
pub struct Connection {
    logger: Logger,
    node: Arc<Node>,

    remote_endpoint: Endpoint,
    local_endpoint: Endpoint,
    state: State,

    dos_score: AtomicU8,
    connected_at: Seconds,

    last_packet_time: Milliseconds,
    last_block: Arc<BlockAnnounce>,
    last_block_time: Milliseconds,
    last_tx_time: Milliseconds,
    last_ping_time: Milliseconds,
    last_inv_sent_time: Milliseconds,
    time_offset: Seconds,
    ping: Milliseconds,
    requested_difficulty: UInt256,

    id: u64,
    version: u32,
    agent: String,
    fee_filter: Amount,
}

impl Connection {
    pub fn send_packet<T: Packet>(&self, _packet: T) {
        todo!();
    }

    pub fn close(&self) {
        todo!();
    }

    pub fn dos(&self, reason: &str) {
        let score = self.dos_score.fetch_add(1, Ordering::AcqRel) + 1;
        if score == 100 {
            self.close();
        }
        info!(self.logger, "{reason} DoS {score}");
    }

    pub fn dos_score(&self) -> u8 {
        self.dos_score.load(Ordering::Acquire)
    }

    pub const fn last_block(&self) -> &Arc<BlockAnnounce> {
        &self.last_block
    }

    pub fn set_last_block(&mut self, block_announce: BlockAnnounce) {
        *Arc::make_mut(&mut self.last_block) = block_announce;
    }

    pub fn requested_blocks(&self) -> bool {
        self.requested_difficulty != UInt256::ZERO
    }

    pub const fn connected_at(&self) -> Seconds {
        self.connected_at
    }

    pub const fn id(&self) -> u64 {
        self.id
    }

    pub const fn version(&self) -> u32 {
        self.version
    }

    pub fn agent(&self) -> &str {
        &self.agent
    }

    pub const fn fee_filter(&self) -> Amount {
        self.fee_filter
    }

    pub const fn is_established(&self) -> bool {
        self.state.is_established()
    }

    pub const fn local_endpoint(&self) -> Endpoint {
        self.local_endpoint
    }

    pub const fn remote_endpoint(&self) -> Endpoint {
        self.remote_endpoint
    }

    pub const fn ping(&self) -> Milliseconds {
        self.ping
    }

    pub const fn time_offset(&self) -> Seconds {
        self.time_offset
    }

    pub const fn state(&self) -> State {
        self.state
    }

    pub fn total_bytes_read(&self) -> u64 {
        todo!();
    }

    pub fn total_bytes_written(&self) -> u64 {
        todo!();
    }

    pub const fn logger(&self) -> &Logger {
        &self.logger
    }

    pub fn node(&self) -> &Node {
        &self.node
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum State {
    IncomingConnected,
    IncomingWaiting,
    OutgoingConnected,
    OutgoingWaiting,
    ProberConnected,
    ProberWaiting,
}

impl State {
    // ProberConnected skips main logic
    pub const fn is_established(self) -> bool {
        matches!(self, State::IncomingConnected | State::OutgoingConnected)
    }

    pub const fn is_incoming(self) -> bool {
        matches!(self, State::IncomingConnected | State::IncomingWaiting)
    }

    pub const fn is_outgoing(self) -> bool {
        matches!(
            self,
            State::OutgoingConnected
                | State::OutgoingWaiting
                | State::ProberConnected
                | State::ProberWaiting
        )
    }
}

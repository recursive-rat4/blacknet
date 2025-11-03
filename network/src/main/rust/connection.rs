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
use core::mem::transmute;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicI64, AtomicU8, AtomicU32, AtomicU64, Ordering},
};

#[expect(dead_code)]
pub struct Connection {
    logger: Logger,
    node: Arc<Node>,

    remote_endpoint: Endpoint,
    local_endpoint: Endpoint,
    state: AtomicU8,

    dos_score: AtomicU8,
    connected_at: Milliseconds,

    last_packet_time: Milliseconds,
    last_block: Mutex<BlockAnnounce>,
    last_block_time: Milliseconds,
    last_tx_time: AtomicI64,
    last_ping_time: AtomicI64,
    last_inv_sent_time: Milliseconds,
    time_offset: AtomicI64,
    ping: AtomicI64,
    ping_request: Mutex<Option<(u32, Milliseconds)>>,
    requested_difficulty: UInt256,

    id: u64,
    version: AtomicU32,
    agent: String,
    fee_filter: AtomicU64,
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

    pub fn last_block(&self) -> BlockAnnounce {
        let x = self.last_block.lock().unwrap();
        x.clone()
    }

    pub fn set_last_block(&self, last_block: BlockAnnounce) {
        let mut x = self.last_block.lock().unwrap();
        *x = last_block;
    }

    pub fn requested_blocks(&self) -> bool {
        self.requested_difficulty != UInt256::ZERO
    }

    pub const fn connected_at(&self) -> Milliseconds {
        self.connected_at
    }

    pub const fn last_packet_time(&self) -> Milliseconds {
        self.last_packet_time
    }

    pub fn last_tx_time(&self) -> Milliseconds {
        self.last_tx_time.load(Ordering::Acquire).into()
    }

    pub fn set_last_tx_time(&self, last_tx_time: Milliseconds) {
        self.last_tx_time
            .store(last_tx_time.into(), Ordering::Release);
    }

    pub fn last_ping_time(&self) -> Milliseconds {
        self.last_ping_time.load(Ordering::Acquire).into()
    }

    pub fn set_last_ping_time(&self, last_ping_time: Milliseconds) {
        self.last_ping_time
            .store(last_ping_time.into(), Ordering::Release);
    }

    pub const fn id(&self) -> u64 {
        self.id
    }

    pub fn version(&self) -> u32 {
        self.version.load(Ordering::Acquire)
    }

    pub fn set_version(&self, version: u32) {
        self.version.store(version, Ordering::Release);
    }

    pub fn agent(&self) -> &str {
        &self.agent
    }

    pub fn set_agent(&self, _string: &str) {
        todo!();
    }

    pub fn fee_filter(&self) -> Amount {
        self.fee_filter.load(Ordering::Acquire).into()
    }

    pub fn set_fee_filter(&self, fee_filter: Amount) {
        self.fee_filter.store(fee_filter.into(), Ordering::Release);
    }

    pub fn is_established(&self) -> bool {
        self.state().is_established()
    }

    pub const fn local_endpoint(&self) -> Endpoint {
        self.local_endpoint
    }

    pub const fn remote_endpoint(&self) -> Endpoint {
        self.remote_endpoint
    }

    pub fn ping(&self) -> Milliseconds {
        self.ping.load(Ordering::Acquire).into()
    }

    pub fn set_ping(&self, ping: Milliseconds) {
        self.ping.store(ping.into(), Ordering::Release);
    }

    pub fn ping_request(&self) -> Option<(u32, Milliseconds)> {
        let x = self.ping_request.lock().unwrap();
        *x
    }

    pub fn set_ping_request(&self, ping_request: Option<(u32, Milliseconds)>) {
        let mut x = self.ping_request.lock().unwrap();
        *x = ping_request;
    }

    pub fn time_offset(&self) -> Seconds {
        self.time_offset.load(Ordering::Acquire).into()
    }

    pub fn set_time_offset(&self, time_offset: Seconds) {
        self.time_offset
            .store(time_offset.into(), Ordering::Release);
    }

    pub fn state(&self) -> State {
        unsafe { transmute(self.state.load(Ordering::Acquire)) }
    }

    pub fn set_state(&self, state: State) {
        self.state.store(state as u8, Ordering::Release);
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
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

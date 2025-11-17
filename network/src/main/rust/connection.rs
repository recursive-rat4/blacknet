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
use crate::packet::{
    BlockAnnounce, INVENTORY_SEND_MAX, INVENTORY_SEND_TIMEOUT, Inventory, Packet, PacketKind,
};
use blacknet_crypto::bigint::UInt256;
use blacknet_kernel::amount::Amount;
use blacknet_kernel::blake2b::Hash;
use blacknet_log::{Logger, error, info};
use blacknet_serialization::format::to_bytes;
use blacknet_time::{Milliseconds, Seconds, SystemClock};
use core::cmp::min;
use core::mem::transmute;
use std::sync::{Arc, Mutex, MutexGuard, RwLock, atomic::*};
use tokio::runtime::Runtime;
use tokio::sync::mpsc::UnboundedSender;
use tokio::task::JoinHandle;
use tokio::time::sleep;

#[expect(dead_code)]
pub struct Connection {
    logger: Logger,
    handles: RwLock<Vec<JoinHandle<()>>>,
    node: Arc<Node>,

    remote_endpoint: Endpoint,
    local_endpoint: Endpoint,
    state: AtomicU8,

    closed: AtomicBool,
    dos_score: AtomicU8,
    send_channel_size: AtomicUsize,
    send_channel: UnboundedSender<(PacketKind, Vec<u8>)>,
    inventory_to_send: Mutex<Vec<Hash>>,
    connected_at: Milliseconds,

    last_packet_time: Milliseconds,
    last_block: Mutex<BlockAnnounce>,
    last_block_time: Milliseconds,
    last_tx_time: AtomicI64,
    last_ping_time: AtomicI64,
    last_inv_sent_time: AtomicI64,
    time_offset: AtomicI64,
    ping: AtomicI64,
    ping_request: Mutex<Option<(u32, Milliseconds)>>,
    requested_difficulty: UInt256,

    id: u64,
    version: AtomicU32,
    agent: Mutex<String>,
    fee_filter: AtomicU64,
}

impl Connection {
    pub fn launch(self: Arc<Self>, runtime: &Runtime) {
        let mut handles = self.handles.write().unwrap();
        handles.push(runtime.spawn(self.clone().pusher()));
    }

    pub async fn join(&self) {
        loop {
            let handle = {
                let mut handles = self.handles.write().unwrap();
                if let Some(handle) = handles.pop() {
                    handle
                } else {
                    break;
                }
            };
            let _ = handle.await;
        }
    }

    pub fn inventory(&self, inv: Hash) {
        let mut inventory_to_send = self.inventory_to_send.lock().unwrap();
        inventory_to_send.push(inv);
        if inventory_to_send.len() == INVENTORY_SEND_MAX {
            self.send_inventory_impl(&mut inventory_to_send, SystemClock::millis());
        }
    }

    pub fn inventory_slice(&self, inv: &[Hash]) {
        let mut inventory_to_send = self.inventory_to_send.lock().unwrap();
        let new_len = inventory_to_send.len() + inv.len();
        if new_len < INVENTORY_SEND_MAX {
            inventory_to_send.extend(inv);
        } else if new_len > INVENTORY_SEND_MAX {
            let n = INVENTORY_SEND_MAX - inventory_to_send.len();
            for &item in inv.iter().take(n) {
                inventory_to_send.push(item);
            }
            self.send_inventory_impl(&mut inventory_to_send, SystemClock::millis());
            for &item in inv.iter().skip(n) {
                inventory_to_send.push(item);
            }
        } else {
            inventory_to_send.extend(inv);
            self.send_inventory_impl(&mut inventory_to_send, SystemClock::millis());
        }
    }

    fn send_inventory(&self, time: Milliseconds) {
        let mut inventory_to_send = self.inventory_to_send.lock().unwrap();
        if !inventory_to_send.is_empty() {
            self.send_inventory_impl(&mut inventory_to_send, time);
        }
    }

    fn send_inventory_impl(
        &self,
        inventory_to_send: &mut MutexGuard<Vec<Hash>>,
        time: Milliseconds,
    ) {
        self.send_packet(&Inventory::new(inventory_to_send.clone()));
        inventory_to_send.clear();
        self.set_last_inv_sent_time(time);
    }

    pub fn send_packet<T: Packet>(&self, packet: &T) {
        let bytes = match to_bytes(&packet) {
            Ok(bytes) => bytes,
            Err(err) => {
                error!(self.logger, "Serialization error: {err}");
                return;
            }
        };
        //TODO review threshold
        if self
            .send_channel_size
            .fetch_add(bytes.len(), Ordering::AcqRel)
            + bytes.len()
            <= self.node().max_packet_size() as usize * 10
        {
            self.send_channel.send((T::kind(), bytes)).unwrap();
        } else {
            info!(self.logger, "Disconnecting on send queue overflow");
            self.close();
        }
    }

    pub fn check_fee_filter(&self, _size: u32, fee: Amount) -> bool {
        //FIXME use size
        self.fee_filter() <= fee
    }

    #[expect(unreachable_code)]
    pub fn close(&self) {
        if !self.closed.fetch_or(true, Ordering::AcqRel) {
            todo!("close socket");

            if let Ok(mut connections) = self.node().connections().write() {
                if let Some(index) = connections
                    .iter()
                    .position(|connection| connection.id() == self.id())
                {
                    connections.swap_remove(index);
                } else {
                    error!(self.logger(), "Close can't find connection");
                }
            }

            if self.is_established() {
                self.node().block_fetcher().disconnected(self);
            }

            let handles = self.handles.read().unwrap();
            handles.iter().for_each(JoinHandle::abort);
        }
    }

    pub fn is_closed(&self) -> bool {
        self.closed.load(Ordering::Acquire)
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

    pub fn last_inv_sent_time(&self) -> Milliseconds {
        self.last_inv_sent_time.load(Ordering::Acquire).into()
    }

    pub fn set_last_inv_sent_time(&self, last_inv_sent_time: Milliseconds) {
        self.last_inv_sent_time
            .store(last_inv_sent_time.into(), Ordering::Release);
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

    pub fn agent(&self) -> String {
        let agent = self.agent.lock().unwrap();
        agent.clone()
    }

    pub fn set_agent(&self, string: &str) {
        const MAX_LENGTH: usize = 256;
        const SAFE_CHARS: &str =
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 .,;-_/:?@()";
        let length = min(string.len(), MAX_LENGTH);
        let mut sanitized = String::with_capacity(length);
        for ch in string.chars() {
            if SAFE_CHARS.contains(ch) {
                sanitized.push(ch);
            }
        }
        let mut agent = self.agent.lock().unwrap();
        *agent = sanitized;
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

    async fn pusher(self: Arc<Self>) {
        while !self.is_established() {
            sleep(INVENTORY_SEND_TIMEOUT.try_into().unwrap()).await;
        }
        loop {
            let now = SystemClock::millis();
            if now >= self.last_inv_sent_time() + INVENTORY_SEND_TIMEOUT {
                self.send_inventory(now);
            }
            sleep(INVENTORY_SEND_TIMEOUT.try_into().unwrap()).await;
        }
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

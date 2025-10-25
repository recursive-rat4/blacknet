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

use crate::v2::{AmountInfo, EndpointInfo, HashInfo};
use blacknet_network::connection::Connection;
use blacknet_network::packet::BlockAnnounce;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PeerInfo {
    peerId: u64,
    remoteAddress: EndpointInfo,
    localAddress: EndpointInfo,
    timeOffset: i64,
    ping: i64,
    protocolVersion: u32,
    agent: String,
    outgoing: bool,
    banScore: u8,
    feeFilter: AmountInfo,
    connectedAt: i64,
    lastChain: ChainInfo,
    requestedBlocks: bool,
    totalBytesRead: u64,
    totalBytesWritten: u64,
}

impl From<&Connection> for PeerInfo {
    fn from(connection: &Connection) -> Self {
        Self {
            peerId: connection.id(),
            remoteAddress: connection.remote_endpoint().into(),
            localAddress: connection.local_endpoint().into(),
            timeOffset: connection.time_offset().into(),
            ping: connection.ping().into(),
            protocolVersion: connection.version(),
            agent: connection.agent().into(),
            outgoing: connection.state().is_outgoing(),
            banScore: connection.dos_score(),
            feeFilter: connection.fee_filter().into(),
            connectedAt: connection.connected_at().into(),
            lastChain: connection.last_block().into(),
            requestedBlocks: connection.requested_blocks(),
            totalBytesRead: connection.total_bytes_read(),
            totalBytesWritten: connection.total_bytes_written(),
        }
    }
}

#[derive(Deserialize, Serialize)]
struct ChainInfo {
    chain: HashInfo,
    cumulativeDifficulty: String,
    fork: bool,
}

impl From<&BlockAnnounce> for ChainInfo {
    fn from(block_announce: &BlockAnnounce) -> Self {
        todo!();
    }
}

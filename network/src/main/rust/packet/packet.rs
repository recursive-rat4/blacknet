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

use crate::connection::Connection;

/**
 * Packet length is used for delimiting, and as such doesn't count towards packet size.
 */
pub const PACKET_LENGTH_SIZE_BYTES: usize = 4;
pub const PACKET_HEADER_SIZE_BYTES: usize = 4;

pub trait Packet {
    fn handle(self, connection: &mut Connection);
}

#[non_exhaustive]
#[repr(u32)]
pub enum PacketKind {
    Version = 0,
    PingV1 = 1,
    Pong = 2,
    GetBlocks = 8,
    Blocks = 9,
    BlockAnnounce = 10,
    ConsensusFault = 11,
    Inventory = 12,
    GetTransactions = 13,
    Transactions = 14,
    Peers = 15,
    Ping = 16,
    Hello = 17,
}

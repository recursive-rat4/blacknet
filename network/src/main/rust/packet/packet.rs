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
use crate::packet::*;
use blacknet_log::info;
use blacknet_serialization::format::from_bytes;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/**
 * Packet length is used for delimiting, and as such doesn't count towards packet size.
 */
pub const PACKET_LENGTH_SIZE_BYTES: u32 = 4;
pub const PACKET_HEADER_SIZE_BYTES: u32 = 4;

pub trait Packet: for<'de> Deserialize<'de> + Serialize {
    fn kind() -> PacketKind;
    fn handle(self, connection: &Arc<Connection>);
}

#[derive(Clone, Copy, Debug)]
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

impl PacketKind {
    pub const fn is_handshake(self) -> bool {
        matches!(self, PacketKind::Version | PacketKind::Hello)
    }

    pub fn handle(self, bytes: &[u8], connection: &Arc<Connection>) -> bool {
        match self {
            PacketKind::Version => match from_bytes::<Version>(bytes, false) {
                Ok(packet) => packet.handle(connection),
                Err(err) => {
                    info!(connection.logger(), "{err} Disconnecting");
                    connection.close();
                    return false;
                }
            },
            PacketKind::PingV1 => match from_bytes::<PingV1>(bytes, false) {
                Ok(packet) => packet.handle(connection),
                Err(err) => {
                    info!(connection.logger(), "{err} Disconnecting");
                    connection.close();
                    return false;
                }
            },
            PacketKind::Pong => match from_bytes::<Pong>(bytes, false) {
                Ok(packet) => packet.handle(connection),
                Err(err) => {
                    info!(connection.logger(), "{err} Disconnecting");
                    connection.close();
                    return false;
                }
            },
            PacketKind::GetBlocks => match from_bytes::<GetBlocks>(bytes, false) {
                Ok(packet) => packet.handle(connection),
                Err(err) => {
                    info!(connection.logger(), "{err} Disconnecting");
                    connection.close();
                    return false;
                }
            },
            PacketKind::Blocks => match from_bytes::<Blocks>(bytes, false) {
                Ok(packet) => packet.handle(connection),
                Err(err) => {
                    info!(connection.logger(), "{err} Disconnecting");
                    connection.close();
                    return false;
                }
            },
            PacketKind::BlockAnnounce => match from_bytes::<BlockAnnounce>(bytes, false) {
                Ok(packet) => packet.handle(connection),
                Err(err) => {
                    info!(connection.logger(), "{err} Disconnecting");
                    connection.close();
                    return false;
                }
            },
            PacketKind::ConsensusFault => match from_bytes::<ConsensusFault>(bytes, false) {
                Ok(packet) => packet.handle(connection),
                Err(err) => {
                    info!(connection.logger(), "{err} Disconnecting");
                    connection.close();
                    return false;
                }
            },
            PacketKind::Inventory => match from_bytes::<Inventory>(bytes, false) {
                Ok(packet) => packet.handle(connection),
                Err(err) => {
                    info!(connection.logger(), "{err} Disconnecting");
                    connection.close();
                    return false;
                }
            },
            PacketKind::GetTransactions => match from_bytes::<GetTransactions>(bytes, false) {
                Ok(packet) => packet.handle(connection),
                Err(err) => {
                    info!(connection.logger(), "{err} Disconnecting");
                    connection.close();
                    return false;
                }
            },
            PacketKind::Transactions => match from_bytes::<Transactions>(bytes, false) {
                Ok(packet) => packet.handle(connection),
                Err(err) => {
                    info!(connection.logger(), "{err} Disconnecting");
                    connection.close();
                    return false;
                }
            },
            PacketKind::Peers => match from_bytes::<Peers>(bytes, false) {
                Ok(packet) => packet.handle(connection),
                Err(err) => {
                    info!(connection.logger(), "{err} Disconnecting");
                    connection.close();
                    return false;
                }
            },
            PacketKind::Ping => match from_bytes::<Ping>(bytes, false) {
                Ok(packet) => packet.handle(connection),
                Err(err) => {
                    info!(connection.logger(), "{err} Disconnecting");
                    connection.close();
                    return false;
                }
            },
            PacketKind::Hello => match from_bytes::<Hello>(bytes, false) {
                Ok(packet) => packet.handle(connection),
                Err(err) => {
                    info!(connection.logger(), "{err} Disconnecting");
                    connection.close();
                    return false;
                }
            },
        }
        true
    }
}

impl TryFrom<u32> for PacketKind {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => PacketKind::Version,
            1 => PacketKind::PingV1,
            2 => PacketKind::Pong,
            8 => PacketKind::GetBlocks,
            9 => PacketKind::Blocks,
            10 => PacketKind::BlockAnnounce,
            11 => PacketKind::ConsensusFault,
            12 => PacketKind::Inventory,
            13 => PacketKind::GetTransactions,
            14 => PacketKind::Transactions,
            15 => PacketKind::Peers,
            16 => PacketKind::Ping,
            17 => PacketKind::Hello,
            _ => return Err(format!("Unknown packet kind 0x{value:08X}")),
        })
    }
}

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

use crate::{connection::Connection, packet::*};
use blacknet_log::info;
use blacknet_serialization::from_bytes;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/**
 * Packet length is used for delimiting, and as such doesn't count towards packet size.
 */
pub const PACKET_LENGTH_SIZE: u32 = 4;
pub const PACKET_HEADER_SIZE: u32 = 4;

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
    FeeFilter = 18,
}

impl PacketKind {
    pub const fn is_handshake(&self) -> bool {
        matches!(self, PacketKind::Version | PacketKind::Hello)
    }

    pub fn handle(&self, bytes: &[u8], connection: &Arc<Connection>) -> bool {
        match self {
            PacketKind::Version => self.h::<Version>(bytes, connection),
            PacketKind::PingV1 => self.h::<PingV1>(bytes, connection),
            PacketKind::Pong => self.h::<Pong>(bytes, connection),
            PacketKind::GetBlocks => self.h::<GetBlocks>(bytes, connection),
            PacketKind::Blocks => self.h::<Blocks>(bytes, connection),
            PacketKind::BlockAnnounce => self.h::<BlockAnnounce>(bytes, connection),
            PacketKind::ConsensusFault => self.h::<ConsensusFault>(bytes, connection),
            PacketKind::Inventory => self.h::<Inventory>(bytes, connection),
            PacketKind::GetTransactions => self.h::<GetTransactions>(bytes, connection),
            PacketKind::Transactions => self.h::<Transactions>(bytes, connection),
            PacketKind::Peers => self.h::<Peers>(bytes, connection),
            PacketKind::Ping => self.h::<Ping>(bytes, connection),
            PacketKind::Hello => self.h::<Hello>(bytes, connection),
            PacketKind::FeeFilter => self.h::<FeeFilter>(bytes, connection),
        }
    }

    fn h<T: Packet>(&self, bytes: &[u8], connection: &Arc<Connection>) -> bool {
        match from_bytes::<T>(bytes, false) {
            Ok(packet) => {
                packet.handle(connection);
                true
            }
            Err(err) => {
                info!(connection.logger(), "{err} Disconnecting");
                connection.close();
                false
            }
        }
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
            18 => PacketKind::FeeFilter,
            _ => return Err(format!("Unknown packet kind 0x{value:08X}")),
        })
    }
}

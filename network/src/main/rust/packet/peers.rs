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
use crate::endpoint::Endpoint;
use crate::packet::{Packet, PacketKind};
use blacknet_log::debug;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub const MAX: usize = 1000;

#[derive(Deserialize, Serialize)]
pub struct Peers {
    list: Box<[Endpoint]>,
}

impl Packet for Peers {
    fn kind() -> PacketKind {
        PacketKind::Peers
    }

    fn handle(self, connection: &Arc<Connection>) {
        if self.list.len() > MAX {
            connection.dos("Invalid Peers size");
            return;
        }

        let peer_table = connection.node().peer_table();
        let added = peer_table.add(self.list.into_iter());
        if added > 0 {
            debug!(connection.logger(), "{added} new peer addresses");
        }
    }
}

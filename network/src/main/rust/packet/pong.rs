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
use crate::packet::{Packet, Ping, PingV1};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct Pong {
    response: u32,
}

impl Pong {
    pub const fn new(response: u32) -> Self {
        Self { response }
    }
}

impl Packet for Pong {
    fn handle(self, connection: &Arc<Connection>) {
        if let Some((challenge, request_time)) = connection.ping_request() {
            let magic = connection.node().mode().network_magic();
            let solution = if connection.version() >= Ping::MIN_VERSION {
                Ping::solve(magic, challenge)
            } else if connection.version() == 13 {
                PingV1::solve(magic, challenge)
            } else {
                challenge
            };

            if self.response != solution {
                connection.dos("Invalid Pong");
                return;
            }

            connection.set_ping(connection.last_packet_time() - request_time);
            connection.set_ping_request(None);
        } else {
            connection.dos("Unexpected packet Pong");
        }
    }
}

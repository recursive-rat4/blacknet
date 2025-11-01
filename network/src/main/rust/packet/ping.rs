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
use crate::node::NETWORK_TIMEOUT;
use crate::packet::{Packet, Pong};
use blacknet_time::{Seconds, SystemClock};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Ping {
    challenge: u32,
    time: Seconds,
}

impl Ping {
    pub const MIN_VERSION: u32 = 14;

    pub const fn solve(magic: u32, challenge: u32) -> u32 {
        challenge ^ magic
    }
}

impl Packet for Ping {
    fn handle(self, connection: &mut Connection) {
        connection.set_time_offset(self.time - SystemClock::secs());

        let magic = connection.node().mode().network_magic();
        connection.send_packet(Pong::new(Self::solve(magic, self.challenge)));

        let last_packet_time = connection.last_packet_time();
        let last_ping_time = connection.last_ping_time();
        connection.set_last_ping_time(last_packet_time);
        if last_packet_time > last_ping_time + NETWORK_TIMEOUT / 2 {
        } else {
            connection.dos("Too many ping requests");
        }
    }
}

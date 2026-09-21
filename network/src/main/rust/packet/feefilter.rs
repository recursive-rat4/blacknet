/*
 * Copyright (c) 2026 Pavel Vasin
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

use crate::{
    connection::Connection,
    packet::{Packet, PacketKind},
};
use blacknet_compat::feerate::FeeRate;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct FeeFilter {
    fee_rate: FeeRate,
}

impl FeeFilter {
    pub const MIN_VERSION: u32 = 16;

    pub const fn new(fee_rate: FeeRate) -> Self {
        Self { fee_rate }
    }

    pub const fn fee_rate(&self) -> FeeRate {
        self.fee_rate
    }
}

impl Packet for FeeFilter {
    fn kind() -> PacketKind {
        PacketKind::FeeFilter
    }

    fn handle(self, connection: &Arc<Connection>) {
        connection.set_fee_filter(self.fee_rate);
    }
}

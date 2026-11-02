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

use blacknet_time::{Milliseconds, Seconds};
use core::num::NonZero;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[non_exhaustive]
pub struct PeerInfo {
    pub id: NonZero<u64>,
    pub remote_endpoint: String,
    pub local_endpoint: String,
    pub time_offset: Seconds,
    pub ping: Milliseconds,
    pub protocol_version: u32,
    pub agent: String,
    pub outgoing: bool,
    pub dos_score: u8,
    pub connected_at: Milliseconds,
    pub total_bytes_read: u64,
    pub total_bytes_written: u64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PeerListResult(pub Vec<PeerInfo>);

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PeerListError;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PeerListParams;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PeerConnectResult;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PeerConnectError(pub Cow<'static, str>);

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PeerConnectParams {
    pub address: String,
    pub port: u16,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PeerDisconnectResult;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PeerDisconnectError(pub Cow<'static, str>);

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PeerDisconnectParams {
    pub id: NonZero<u64>,
}

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
use core::{fmt, num::NonZero};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[cfg(any(feature = "client", feature = "server"))]
use blacknet_json_rpc::{Method, peer::Route};

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
pub enum PeerListError {}

impl fmt::Display for PeerListError {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {}
    }
}

impl core::error::Error for PeerListError {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PeerListParams;

#[cfg(any(feature = "client", feature = "server"))]
impl Route for PeerListParams {
    type Result = PeerListResult;
    type Error = PeerListError;

    const METHOD: Method = Method::new("peer.v1.list");
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PeerConnectResult;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PeerConnectError(pub Cow<'static, str>);

impl fmt::Display for PeerConnectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl core::error::Error for PeerConnectError {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PeerConnectParams {
    pub address: String,
    pub port: u16,
}

#[cfg(any(feature = "client", feature = "server"))]
impl Route for PeerConnectParams {
    type Result = PeerConnectResult;
    type Error = PeerConnectError;

    const METHOD: Method = Method::new("peer.v1.connect");
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PeerDisconnectResult;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PeerDisconnectError(pub Cow<'static, str>);

impl fmt::Display for PeerDisconnectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl core::error::Error for PeerDisconnectError {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PeerDisconnectParams {
    pub id: NonZero<u64>,
}

#[cfg(any(feature = "client", feature = "server"))]
impl Route for PeerDisconnectParams {
    type Result = PeerDisconnectResult;
    type Error = PeerDisconnectError;

    const METHOD: Method = Method::new("peer.v1.disconnect");
}

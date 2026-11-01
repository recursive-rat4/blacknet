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

#[cfg(feature = "server")]
use crate::RpcServer;
#[cfg(feature = "server")]
use blacknet_json_rpc::{
    Method,
    server::{Handler, OccupiedError, Router},
};
#[cfg(feature = "server")]
use blacknet_network::connection::Connection;
#[cfg(feature = "server")]
use core::ops::Deref;

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

impl PeerInfo {
    #[cfg(feature = "server")]
    fn with(connection: &Connection) -> Self {
        Self {
            id: connection.id(),
            remote_endpoint: connection.remote_endpoint().to_log(true),
            local_endpoint: connection.remote_endpoint().to_log(true),
            time_offset: connection.time_offset(),
            ping: connection.ping(),
            protocol_version: connection.version(),
            agent: connection.agent().to_string(),
            outgoing: connection.state().is_outgoing(),
            dos_score: connection.dos_score(),
            connected_at: connection.connected_at(),
            total_bytes_read: connection.total_bytes_read(),
            total_bytes_written: connection.total_bytes_written(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PeerListResult(pub Vec<PeerInfo>);

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PeerListError;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PeerListParams;

#[cfg(feature = "server")]
impl Handler<RpcServer> for PeerListParams {
    type Ok = PeerListResult;
    type Error = PeerListError;

    fn handle(self, rpc_server: &RpcServer) -> Result<Self::Ok, Self::Error> {
        let connections = rpc_server.network().node().connections().read().unwrap();
        let list = connections
            .iter()
            .map(Deref::deref)
            .map(PeerInfo::with)
            .collect();
        Ok(PeerListResult(list))
    }
}

#[cfg(feature = "server")]
pub fn routes() -> Result<Router<RpcServer>, OccupiedError> {
    let mut router = Router::new();
    router.route::<PeerListParams>(Method::new("peer.v1.list"))?;
    Ok(router)
}

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
    RpcServer,
    v3::peer::{PeerInfo, PeerListError, PeerListParams, PeerListResult},
};
use blacknet_json_rpc::{
    Method,
    server::{Handler, OccupiedError, Router},
};
use blacknet_network::connection::Connection;
use core::ops::Deref;

impl PeerInfo {
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

#[expect(dead_code)]
pub fn routes() -> Result<Router<RpcServer>, OccupiedError> {
    let mut router = Router::new();
    router.route::<PeerListParams>(Method::new("peer.v1.list"))?;
    Ok(router)
}

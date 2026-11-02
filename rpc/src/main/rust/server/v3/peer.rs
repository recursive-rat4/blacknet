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

use crate::{RpcServer, v3::peer::*};
use blacknet_json_rpc::server::{Handler, OccupiedError, Router};
use blacknet_network::{connection::Connection, endpoint::Endpoint};
use blacknet_time::SystemClock;
use core::ops::Deref;
use std::borrow::Cow;

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

impl From<&'static str> for PeerConnectError {
    fn from(s: &'static str) -> Self {
        Self(Cow::Borrowed(s))
    }
}

impl From<&'static str> for PeerDisconnectError {
    fn from(s: &'static str) -> Self {
        Self(Cow::Borrowed(s))
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

impl Handler<RpcServer> for PeerConnectParams {
    type Ok = PeerConnectResult;
    type Error = PeerConnectError;

    fn handle(self, rpc_server: &RpcServer) -> Result<Self::Ok, Self::Error> {
        let endpoint = Endpoint::parse(&self.address, self.port).ok_or("Invalid endpoint")?;
        let node = rpc_server.network().node();
        let guard = node
            .peer_table()
            .try_contact(endpoint)
            .ok_or("Already in contact")?;
        let time = SystemClock::millis();
        node.runtime()
            .spawn(node.clone().add_peer(guard, time, false));
        Ok(PeerConnectResult)
    }
}

impl Handler<RpcServer> for PeerDisconnectParams {
    type Ok = PeerDisconnectResult;
    type Error = PeerDisconnectError;

    fn handle(self, rpc_server: &RpcServer) -> Result<Self::Ok, Self::Error> {
        let connections = rpc_server.network().node().connections().read().unwrap();
        if let Some(connection) = connections
            .iter()
            .find(|connection| connection.id() == self.id)
        {
            connection.close();
            Ok(PeerDisconnectResult)
        } else {
            Err("Peer not found")?
        }
    }
}

#[expect(dead_code)]
pub fn routes() -> Result<Router<RpcServer>, OccupiedError> {
    let mut router = Router::new();
    router.route::<PeerListParams>()?;
    router.route::<PeerConnectParams>()?;
    router.route::<PeerDisconnectParams>()?;
    Ok(router)
}

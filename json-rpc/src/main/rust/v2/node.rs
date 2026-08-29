/*
 * Copyright (c) 2025-2026 Pavel Vasin
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

use crate::v2::response::*;
use crate::v2::{NodeInfo, PeerInfo, TransactionInfo, TxPoolInfo, fork_cache_new};
use axum::{
    Json, Router,
    extract::{Path, State},
    response::Response,
    routing::get,
};
use blacknet_kernel::blake2b::Hash;
use blacknet_kernel::transaction::Transaction;
use blacknet_network::connection::ConnectionId;
use blacknet_network::endpoint::Endpoint;
use blacknet_network::network::Network;
use blacknet_serialization::format::from_bytes;
use std::sync::Arc;

async fn peers(State(network): State<Arc<Network>>) -> Json<Vec<PeerInfo>> {
    let node = network.node();
    let block_db = node.block_db();
    let connections = node.connections().read().unwrap();
    let mut fork_cache = fork_cache_new();
    Json(
        connections
            .iter()
            .map(|connection| PeerInfo::new(connection, &mut fork_cache, block_db))
            .collect(),
    )
}

async fn node(State(network): State<Arc<Network>>) -> Json<NodeInfo> {
    Json(NodeInfo::new(&network))
}

async fn tx_pool(State(network): State<Arc<Network>>) -> Json<TxPoolInfo> {
    let tx_pool = network.node().tx_pool().read().unwrap();
    Json(TxPoolInfo::new(&tx_pool))
}

async fn tx_pool_transaction(
    Path((hash, raw)): Path<(Hash, Option<bool>)>,
    State(network): State<Arc<Network>>,
) -> Response<String> {
    let address_codec = network.wallet_db().address_codec();
    let tx_pool = network.node().tx_pool().read().unwrap();
    if let Some(bytes) = tx_pool.get_raw(hash) {
        if let Some(raw) = raw
            && raw
        {
            respond_hex(bytes)
        } else {
            let tx = from_bytes::<Transaction>(bytes, false).unwrap();
            let info = TransactionInfo::new(&tx, hash, bytes.len(), address_codec).unwrap();
            respond_json(&info)
        }
    } else {
        respond_error("Transaction not found")
    }
}

async fn add_peer(
    Path((address, port, _force)): Path<(String, Option<u16>, Option<bool>)>,
    State(network): State<Arc<Network>>,
) -> Response<String> {
    let _endpoint = if let Some(endpoint) = Endpoint::parse(
        &address,
        port.unwrap_or_else(|| network.mode().default_p2p_port()),
    ) {
        endpoint
    } else {
        return respond_error("Invalid endpoint");
    };

    todo!();
}

async fn disconnect_peer_by_address(
    Path((address, port, _force)): Path<(String, Option<u16>, Option<bool>)>,
    State(network): State<Arc<Network>>,
) -> Response<String> {
    let endpoint = if let Some(endpoint) = Endpoint::parse(
        &address,
        port.unwrap_or_else(|| network.mode().default_p2p_port()),
    ) {
        endpoint
    } else {
        return respond_error("Invalid endpoint");
    };

    let connections = network.node().connections().read().unwrap();
    if let Some(connection) = connections
        .iter()
        .find(|connection| connection.remote_endpoint() == endpoint)
    {
        connection.close();
        respond_text("true")
    } else {
        respond_text("false")
    }
}

async fn disconnect_peer(
    Path((id, _force)): Path<(ConnectionId, Option<bool>)>,
    State(network): State<Arc<Network>>,
) -> Response<String> {
    let connections = network.node().connections().read().unwrap();
    if let Some(connection) = connections.iter().find(|connection| connection.id() == id) {
        connection.close();
        respond_text("true")
    } else {
        respond_text("false")
    }
}

pub fn routes() -> Router<Arc<Network>> {
    Router::new()
        .route("/api/v2/peers", get(peers))
        .route("/api/v2/node", get(node))
        .route("/api/v2/txpool", get(tx_pool))
        .route(
            "/api/v2/txpool/transaction/{hash}/{raw}",
            get(tx_pool_transaction),
        )
        .route("/api/v2/addpeer/{address}/{port}/{force}", get(add_peer))
        .route(
            "/api/v2/disconnectpeerbyaddress/{address}/{port}/{force}",
            get(disconnect_peer_by_address),
        )
        .route("/api/v2/disconnectpeer/{id}/{force}", get(disconnect_peer))
}

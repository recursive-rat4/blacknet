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

use crate::v2::{NodeInfo, PeerInfo, TransactionInfo, TxPoolInfo, fork_cache_new, response::*};
use axum::{
    Json, Router,
    extract::{Path, State},
    response::Response,
    routing::get,
};
use blacknet_kernel::{blake2b::Hash, transaction::Transaction};
use blacknet_network::{
    connection::ConnectionId, endpoint::Endpoint, network::Network, txpool::TxPoolCheck,
};
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
    Path(hash): Path<String>,
    State(network): State<Arc<Network>>,
) -> Response<String> {
    tx_pool_transaction_handler(&hash, false, &network)
}

async fn tx_pool_transaction_raw(
    Path((hash, raw)): Path<(String, bool)>,
    State(network): State<Arc<Network>>,
) -> Response<String> {
    tx_pool_transaction_handler(&hash, raw, &network)
}

fn tx_pool_transaction_handler(hash: &str, raw: bool, network: &Arc<Network>) -> Response<String> {
    let hash = match Hash::try_from(hash) {
        Ok(hash) => hash,
        Err(err) => return respond_error(format!("Invalid hash: {err}")),
    };
    let address_codec = network.wallet_db().address_codec();
    let tx_pool = network.node().tx_pool().read().unwrap();
    if let Some(bytes) = tx_pool.get_raw(hash) {
        if raw {
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

async fn tx_pool_check(State(network): State<Arc<Network>>) -> Json<TxPoolCheck> {
    let node = network.node();
    let mut tx_pool = node.tx_pool().write().unwrap();
    Json(tx_pool.check())
}

async fn add_peer(
    Path(address): Path<String>,
    State(network): State<Arc<Network>>,
) -> Response<String> {
    add_peer_handler(&address, network.mode().default_p2p_port(), false, &network)
}

async fn add_peer_with_port(
    Path((address, port)): Path<(String, u16)>,
    State(network): State<Arc<Network>>,
) -> Response<String> {
    add_peer_handler(&address, port, false, &network)
}

async fn add_peer_with_all(
    Path((address, port, force)): Path<(String, u16, bool)>,
    State(network): State<Arc<Network>>,
) -> Response<String> {
    add_peer_handler(&address, port, force, &network)
}

#[expect(unused_variables)]
fn add_peer_handler(
    address: &str,
    port: u16,
    _force: bool,
    network: &Arc<Network>,
) -> Response<String> {
    let Some(endpoint) = Endpoint::parse(address, port) else {
        return respond_error("Invalid endpoint");
    };

    todo!();
}

async fn disconnect_peer_by_address(
    Path(address): Path<String>,
    State(network): State<Arc<Network>>,
) -> Response<String> {
    disconnect_peer_by_address_handler(&address, network.mode().default_p2p_port(), false, &network)
}

async fn disconnect_peer_by_address_with_port(
    Path((address, port)): Path<(String, u16)>,
    State(network): State<Arc<Network>>,
) -> Response<String> {
    disconnect_peer_by_address_handler(&address, port, false, &network)
}

async fn disconnect_peer_by_address_with_all(
    Path((address, port, force)): Path<(String, u16, bool)>,
    State(network): State<Arc<Network>>,
) -> Response<String> {
    disconnect_peer_by_address_handler(&address, port, force, &network)
}

fn disconnect_peer_by_address_handler(
    address: &str,
    port: u16,
    _force: bool,
    network: &Arc<Network>,
) -> Response<String> {
    let Some(endpoint) = Endpoint::parse(address, port) else {
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
    Path(id): Path<ConnectionId>,
    State(network): State<Arc<Network>>,
) -> Response<String> {
    disconnect_peer_handler(id, false, &network)
}

async fn disconnect_peer_with_force(
    Path((id, force)): Path<(ConnectionId, bool)>,
    State(network): State<Arc<Network>>,
) -> Response<String> {
    disconnect_peer_handler(id, force, &network)
}

fn disconnect_peer_handler(
    id: ConnectionId,
    _force: bool,
    network: &Arc<Network>,
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
            "/api/v2/txpool/transaction/{hash}",
            get(tx_pool_transaction),
        )
        .route(
            "/api/v2/txpool/transaction/{hash}/{raw}",
            get(tx_pool_transaction_raw),
        )
        .route("/api/v2/txpool/check", get(tx_pool_check))
        .route("/api/v2/addpeer/{address}", get(add_peer))
        .route("/api/v2/addpeer/{address}/{port}", get(add_peer_with_port))
        .route(
            "/api/v2/addpeer/{address}/{port}/{force}",
            get(add_peer_with_all),
        )
        .route(
            "/api/v2/disconnectpeerbyaddress/{address}",
            get(disconnect_peer_by_address),
        )
        .route(
            "/api/v2/disconnectpeerbyaddress/{address}/{port}",
            get(disconnect_peer_by_address_with_port),
        )
        .route(
            "/api/v2/disconnectpeerbyaddress/{address}/{port}/{force}",
            get(disconnect_peer_by_address_with_all),
        )
        .route("/api/v2/disconnectpeer/{id}", get(disconnect_peer))
        .route(
            "/api/v2/disconnectpeer/{id}/{force}",
            get(disconnect_peer_with_force),
        )
}

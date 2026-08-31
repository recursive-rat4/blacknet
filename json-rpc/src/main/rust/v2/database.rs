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
use crate::v2::{AccountInfo, BlockIndexInfo, BlockInfo, CoinDBInfo, PeerTableInfo};
use axum::{
    Json, Router,
    extract::{Path, State},
    response::Response,
    routing::get,
};
use blacknet_kernel::{blake2b::Hash, proofofstake::DEFAULT_CONFIRMATIONS};
use blacknet_network::{
    db::{BlockDBCheck, CoinDBCheck},
    network::Network,
};
use std::{path::absolute, sync::Arc};

async fn peer_table(State(network): State<Arc<Network>>) -> Json<PeerTableInfo> {
    let peer_table = network.node().peer_table();
    Json(PeerTableInfo::new(peer_table))
}

async fn peer_table_stat(State(network): State<Arc<Network>>) -> Json<PeerTableInfo> {
    let peer_table = network.node().peer_table();
    Json(PeerTableInfo::with_stat(peer_table))
}

async fn kv_store_stat(State(network): State<Arc<Network>>) -> Response<String> {
    let db = network.node().fjall().database();
    respond_text(format!(
        "disk_space: {}\njournal_count: {}\nkeyspace_count: {}",
        match db.disk_space() {
            Ok(n) => n.to_string(),
            Err(err) => err.to_string(),
        },
        db.journal_count(),
        db.keyspace_count()
    ))
}

async fn block(Path(hash): Path<String>, State(network): State<Arc<Network>>) -> Response<String> {
    block_handler(&hash, false, &network)
}

async fn block_with_txdetail(
    Path((hash, txdetail)): Path<(String, bool)>,
    State(network): State<Arc<Network>>,
) -> Response<String> {
    block_handler(&hash, txdetail, &network)
}

fn block_handler(hash: &str, txdetail: bool, network: &Arc<Network>) -> Response<String> {
    let hash = match Hash::try_from(hash) {
        Ok(hash) => hash,
        Err(err) => return respond_error(format!("Invalid hash: {err}")),
    };
    let block_db = network.node().block_db();
    if let Some((block, size)) = block_db.get(hash) {
        let address_codec = network.wallet_db().address_codec();
        match BlockInfo::new(&block, hash, size as u32, txdetail, address_codec) {
            Ok(info) => respond_json(&info),
            Err(err) => respond_error(format!("Internal error: {err}")),
        }
    } else {
        respond_error("Block not found")
    }
}

async fn block_db_check(State(network): State<Arc<Network>>) -> Json<BlockDBCheck> {
    let node = network.node();
    let block_db = node.block_db();
    let coin_db = node.coin_db();
    let state = coin_db.state().load();
    Json(block_db.check(&state))
}

async fn block_hash(
    Path(height): Path<u32>,
    State(network): State<Arc<Network>>,
) -> Response<String> {
    let node = network.node();
    let block_db = node.block_db();
    let coin_db = node.coin_db();
    let state = coin_db.state().load();
    if let Some(hash) = block_db.hash(height, &state) {
        respond_text(hash.to_string())
    } else {
        respond_error("Block not found")
    }
}

async fn block_index(
    Path(hash): Path<String>,
    State(network): State<Arc<Network>>,
) -> Response<String> {
    let hash = match Hash::try_from(hash.as_str()) {
        Ok(hash) => hash,
        Err(err) => return respond_error(format!("Invalid hash: {err}")),
    };
    let block_db = network.node().block_db();
    if let Some(index) = block_db.index(hash) {
        respond_json(&BlockIndexInfo::new(index))
    } else {
        respond_error("Block not found")
    }
}

async fn make_bootstrap(State(network): State<Arc<Network>>) -> Response<String> {
    let node = network.node();
    let block_db = node.block_db();
    let coin_db = node.coin_db();
    let state = coin_db.state().load();
    match block_db.export(&state) {
        Some(path) => match absolute(&path) {
            Ok(path) => respond_text(path.display().to_string()),
            Err(_) => respond_text(path.display().to_string()),
        },
        None => respond_error("Not synchronized"),
    }
}

async fn coin_db(State(network): State<Arc<Network>>) -> Json<CoinDBInfo> {
    let coin_db = network.node().coin_db();
    let state = coin_db.state().load();
    Json(CoinDBInfo::new(&state))
}

async fn coin_db_check(State(network): State<Arc<Network>>) -> Json<CoinDBCheck> {
    let coin_db = network.node().coin_db();
    Json(coin_db.check())
}

async fn account(
    Path(address): Path<String>,
    State(network): State<Arc<Network>>,
) -> Response<String> {
    account_handler(&address, DEFAULT_CONFIRMATIONS, &network)
}

async fn account_with_confirmations(
    Path((address, confirmations)): Path<(String, u32)>,
    State(network): State<Arc<Network>>,
) -> Response<String> {
    account_handler(&address, confirmations, &network)
}

fn account_handler(address: &str, confirmations: u32, network: &Arc<Network>) -> Response<String> {
    let address_codec = network.wallet_db().address_codec();
    let public_key = {
        match address_codec.decode(address) {
            Ok(public_key) => public_key,
            Err(err) => {
                return respond_error(format!("Invalid address: {err}"));
            }
        }
    };

    let coin_db = network.node().coin_db();
    if let Some(account) = coin_db.account(public_key) {
        let state = coin_db.state().load();
        match AccountInfo::new(&account, state.height(), confirmations, address_codec) {
            Ok(info) => respond_json(&info),
            Err(err) => respond_error(format!("Internal error: {err}")),
        }
    } else {
        respond_error("Account not found")
    }
}

pub fn routes() -> Router<Arc<Network>> {
    Router::new()
        .route("/api/v2/peerdb", get(peer_table))
        .route("/api/v2/peerdb/networkstat", get(peer_table_stat))
        .route("/api/v2/leveldb/stats", get(kv_store_stat))
        .route("/api/v2/block/{hash}", get(block))
        .route("/api/v2/block/{hash}/{txdetail}", get(block_with_txdetail))
        .route("/api/v2/blockdb/check", get(block_db_check))
        .route("/api/v2/blockhash/{height}", get(block_hash))
        .route("/api/v2/blockindex/{hash}", get(block_index))
        .route("/api/v2/makebootstrap", get(make_bootstrap))
        .route("/api/v2/ledger", get(coin_db))
        .route("/api/v2/ledger/check", get(coin_db_check))
        .route("/api/v2/account/{address}", get(account))
        .route(
            "/api/v2/account/{address}/{confirmations}",
            get(account_with_confirmations),
        )
}

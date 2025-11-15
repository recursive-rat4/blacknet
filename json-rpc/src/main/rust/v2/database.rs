/*
 * Copyright (c) 2025 Pavel Vasin
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
use blacknet_kernel::blake2b::Hash;
use blacknet_kernel::proofofstake::DEFAULT_CONFIRMATIONS;
use blacknet_network::blockdb::Check as BlockDBCheck;
use blacknet_network::coindb::Check as CoinDBCheck;
use blacknet_network::node::Node;
use std::path::absolute;
use std::sync::Arc;

async fn peer_table(State(node): State<Arc<Node>>) -> Json<PeerTableInfo> {
    let peer_table = node.peer_table();
    Json(PeerTableInfo::new(peer_table))
}

async fn peer_table_stat(State(node): State<Arc<Node>>) -> Json<PeerTableInfo> {
    let peer_table = node.peer_table();
    Json(PeerTableInfo::with_stat(peer_table))
}

async fn kv_store_stat(State(_node): State<Arc<Node>>) -> Response<String> {
    todo!();
}

async fn block(
    Path((hash, txdetail)): Path<(Hash, Option<bool>)>,
    State(node): State<Arc<Node>>,
) -> Response<String> {
    let block_db = node.block_db();
    if let Some((block, size)) = block_db.get(hash) {
        let address_codec = node.wallet_db().address_codec();
        match BlockInfo::new(
            &block,
            hash,
            size as u32,
            txdetail.unwrap_or(false),
            address_codec,
        ) {
            Ok(info) => respond_json(&info),
            Err(err) => respond_error(format!("Internal error: {err}")),
        }
    } else {
        respond_error("Block not found".to_owned())
    }
}

async fn block_db_check(State(node): State<Arc<Node>>) -> Json<BlockDBCheck> {
    let block_db = node.block_db();
    Json(block_db.check())
}

async fn block_hash(Path(height): Path<u32>, State(node): State<Arc<Node>>) -> Response<String> {
    let block_db = node.block_db();
    if let Some(hash) = block_db.hash(height) {
        respond_text(hash.to_string())
    } else {
        respond_error("Block not found".to_owned())
    }
}

async fn block_index(Path(hash): Path<Hash>, State(node): State<Arc<Node>>) -> Response<String> {
    let block_db = node.block_db();
    if let Some(index) = block_db.index(hash) {
        respond_json(&BlockIndexInfo::new(index))
    } else {
        respond_error("Block not found".to_owned())
    }
}

async fn make_bootstrap(State(node): State<Arc<Node>>) -> Response<String> {
    let block_db = node.block_db();
    match block_db.export() {
        Some(path) => match absolute(&path) {
            Ok(path) => respond_text(path.display().to_string()),
            Err(_) => respond_text(path.display().to_string()),
        },
        None => respond_error("Not synchronized".to_owned()),
    }
}

async fn coin_db(State(node): State<Arc<Node>>) -> Json<CoinDBInfo> {
    let coin_db = node.coin_db();
    Json(CoinDBInfo::new(coin_db.state()))
}

async fn coin_db_check(State(node): State<Arc<Node>>) -> Json<CoinDBCheck> {
    let coin_db = node.coin_db();
    Json(coin_db.check())
}

async fn account(
    Path((address, confirmations)): Path<(String, Option<u32>)>,
    State(node): State<Arc<Node>>,
) -> Response<String> {
    let address_codec = node.wallet_db().address_codec();
    let public_key = {
        match address_codec.decode(&address) {
            Ok(public_key) => public_key,
            Err(err) => {
                return respond_error(format!("Invalid address: {err}"));
            }
        }
    };
    let confirmations = confirmations.unwrap_or(DEFAULT_CONFIRMATIONS);

    let coin_db = node.coin_db();
    if let Some(account) = coin_db.account(public_key) {
        let state = coin_db.state();
        match AccountInfo::new(&account, state.height(), confirmations, address_codec) {
            Ok(info) => respond_json(&info),
            Err(err) => respond_error(format!("Internal error: {err}")),
        }
    } else {
        respond_error("Account not found".to_owned())
    }
}

pub fn routes() -> Router<Arc<Node>> {
    Router::new()
        .route("/api/v2/peerdb", get(peer_table))
        .route("/api/v2/peerdb/networkstat", get(peer_table_stat))
        .route("/api/v2/leveldb/stats", get(kv_store_stat))
        .route("/api/v2/block/{hash}/{txdetail}", get(block))
        .route("/api/v2/blockdb/check", get(block_db_check))
        .route("/api/v2/blockhash/{height}", get(block_hash))
        .route("/api/v2/blockindex/{hash}", get(block_index))
        .route("/api/v2/makebootstrap", get(make_bootstrap))
        .route("/api/v2/ledger", get(coin_db))
        .route("/api/v2/ledger/check", get(coin_db_check))
        .route("/api/v2/account/{address}/{confirmations}", get(account))
}

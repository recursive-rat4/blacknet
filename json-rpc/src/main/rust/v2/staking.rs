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

use crate::v2::StakingInfo;
use crate::v2::response::*;
use axum::{
    Form, Router,
    extract::{Path, State},
    response::Response,
    routing::get,
    routing::post,
};
use blacknet_kernel::ed25519::{PublicKey, to_secret_key};
use blacknet_network::node::Node;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use zeroize::ZeroizeOnDrop;

#[derive(Deserialize, Serialize, ZeroizeOnDrop)]
pub struct StartStakingRequest {
    pub mnemonic: String,
}

async fn start_staking(
    State(node): State<Arc<Node>>,
    Form(request): Form<StartStakingRequest>,
) -> Response<String> {
    let secret_key = if let Some(secret_key) = to_secret_key(&request.mnemonic) {
        secret_key
    } else {
        return respond_error("Invalid mnemonic");
    };
    respond_bool(node.staker().start_staking(&secret_key))
}

#[derive(Deserialize, Serialize, ZeroizeOnDrop)]
pub struct StopStakingRequest {
    pub mnemonic: String,
}

async fn stop_staking(
    State(node): State<Arc<Node>>,
    Form(request): Form<StopStakingRequest>,
) -> Response<String> {
    let secret_key = if let Some(secret_key) = to_secret_key(&request.mnemonic) {
        secret_key
    } else {
        return respond_error("Invalid mnemonic");
    };
    respond_bool(node.staker().stop_staking(&secret_key))
}

#[derive(Deserialize, Serialize, ZeroizeOnDrop)]
pub struct IsStakingRequest {
    pub mnemonic: String,
}

async fn is_staking(
    State(node): State<Arc<Node>>,
    Form(request): Form<IsStakingRequest>,
) -> Response<String> {
    let secret_key = if let Some(secret_key) = to_secret_key(&request.mnemonic) {
        secret_key
    } else {
        return respond_error("Invalid mnemonic");
    };
    respond_bool(node.staker().is_staking(&secret_key))
}

async fn staking(
    State(node): State<Arc<Node>>,
    Path(address): Path<Option<String>>,
) -> Response<String> {
    let address_codec = node.wallet_db().address_codec();
    let public_key: Option<PublicKey> = match address {
        Some(address) => match address_codec.decode(&address) {
            Ok(public_key) => Some(public_key),
            Err(err) => {
                return respond_error(format!("Invalid public key: {err}"));
            }
        },
        None => None,
    };
    let stats = node.staker().stats(&public_key);
    let info = StakingInfo::new(&stats);
    respond_json(&info)
}

pub fn routes() -> Router<Arc<Node>> {
    Router::new()
        .route("/api/v2/startstaking", post(start_staking))
        .route("/api/v2/stopstaking", post(stop_staking))
        .route("/api/v2/isstaking", post(is_staking))
        .route("/api/v2/staking/{address}", get(staking))
}

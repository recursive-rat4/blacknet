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

use crate::v2::StakingInfo;
use crate::v2::response::*;
use axum::{
    Form, Json, Router,
    extract::{Path, State},
    response::Response,
    routing::get,
    routing::post,
};
use blacknet_kernel::ed25519::to_private_key;
use blacknet_network::node::Node;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct StartStakingRequest {
    pub mnemonic: String,
}

async fn start_staking(
    State(_node): State<Arc<Node>>,
    Form(request): Form<StartStakingRequest>,
) -> Response<String> {
    let _private_key = if let Some(private_key) = to_private_key(&request.mnemonic) {
        private_key
    } else {
        return respond_error("Invalid mnemonic".to_owned());
    };
    todo!();
}

#[derive(Deserialize, Serialize)]
pub struct StopStakingRequest {
    pub mnemonic: String,
}

async fn stop_staking(
    State(_node): State<Arc<Node>>,
    Form(request): Form<StopStakingRequest>,
) -> Response<String> {
    let _private_key = if let Some(private_key) = to_private_key(&request.mnemonic) {
        private_key
    } else {
        return respond_error("Invalid mnemonic".to_owned());
    };
    todo!();
}

#[derive(Deserialize, Serialize)]
pub struct IsStakingRequest {
    pub mnemonic: String,
}

async fn is_staking(
    State(_node): State<Arc<Node>>,
    Form(request): Form<IsStakingRequest>,
) -> Response<String> {
    let _private_key = if let Some(private_key) = to_private_key(&request.mnemonic) {
        private_key
    } else {
        return respond_error("Invalid mnemonic".to_owned());
    };
    todo!();
}

async fn staking(
    State(_node): State<Arc<Node>>,
    Path(_address): Path<Option<String>>,
) -> Json<StakingInfo> {
    todo!();
}

pub fn routes() -> Router<Arc<Node>> {
    Router::new()
        .route("/api/v2/startstaking", post(start_staking))
        .route("/api/v2/stopstaking", post(stop_staking))
        .route("/api/v2/isstaking", post(is_staking))
        .route("/api/v2/staking/{address}", get(staking))
}

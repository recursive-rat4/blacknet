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

use crate::v2::{
    AddressInfo, HashInfo, LeaseInfo, MnemonicInfo, NewMnemonicInfo, TransactionDataInfo,
    WalletTransactionInfo, response::*,
};
use axum::{
    Form, Json, Router,
    extract::{Path, State},
    response::Response,
    routing::{get, post},
};
use blacknet_kernel::ed25519::PublicKey;
use blacknet_network::network::Network;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use zeroize::{Zeroize, ZeroizeOnDrop};

#[expect(unused_variables)]
async fn generate_account(
    State(network): State<Arc<Network>>,
    wordlist: Option<Path<String>>,
) -> Json<NewMnemonicInfo> {
    todo!();
}

#[expect(unused_variables)]
async fn address(State(network): State<Arc<Network>>, address: Path<String>) -> Json<AddressInfo> {
    todo!();
}

#[derive(Deserialize, Serialize, ZeroizeOnDrop)]
pub struct MnemonicRequest {
    pub mnemonic: String,
}

#[expect(unused_variables)]
async fn mnemonic(
    State(network): State<Arc<Network>>,
    Form(request): Form<MnemonicRequest>,
) -> Json<MnemonicInfo> {
    todo!();
}

#[derive(Deserialize, Serialize)]
pub struct DecryptPaymentIdRequest {
    pub mnemonic: String,
    pub from: PublicKey,
    pub message: String,
}

impl Drop for DecryptPaymentIdRequest {
    fn drop(&mut self) {
        self.mnemonic.zeroize();
    }
}

#[expect(unused_variables)]
async fn decrypt_payment_id(
    State(network): State<Arc<Network>>,
    Form(request): Form<DecryptPaymentIdRequest>,
) -> Response<String> {
    todo!();
}

#[derive(Deserialize, Serialize)]
pub struct SignMessageRequest {
    pub mnemonic: String,
    pub message: String,
}

impl Drop for SignMessageRequest {
    fn drop(&mut self) {
        self.mnemonic.zeroize();
    }
}

#[expect(unused_variables)]
async fn sign_message(
    State(network): State<Arc<Network>>,
    Form(request): Form<SignMessageRequest>,
) -> Response<String> {
    todo!();
}

#[expect(unused_variables)]
async fn verify_message(
    State(network): State<Arc<Network>>,
    Path((from, signature, message)): Path<(String, String, String)>,
) -> Response<String> {
    todo!();
}

#[expect(unused_variables)]
async fn transactions(
    State(network): State<Arc<Network>>,
    address: Path<String>,
) -> Json<HashMap<String, TransactionDataInfo>> {
    todo!();
}

#[expect(unused_variables)]
async fn out_leases(
    State(network): State<Arc<Network>>,
    address: Path<String>,
) -> Json<Vec<LeaseInfo>> {
    todo!();
}

#[expect(unused_variables)]
async fn sequence(State(network): State<Arc<Network>>, address: Path<String>) -> Response<String> {
    todo!();
}

#[expect(unused_variables)]
async fn transaction(
    State(network): State<Arc<Network>>,
    Path((address, hash)): Path<(String, String)>,
) -> Response<String> {
    todo!();
}

#[expect(unused_variables)]
async fn transaction_raw(
    State(network): State<Arc<Network>>,
    Path((address, hash, raw)): Path<(String, String, bool)>,
) -> Response<String> {
    todo!();
}

#[expect(unused_variables)]
async fn confirmations(
    State(network): State<Arc<Network>>,
    Path((address, hash)): Path<(String, String)>,
) -> Response<String> {
    todo!();
}

async fn anchor(State(network): State<Arc<Network>>, _address: Path<String>) -> Response<String> {
    let (ref state, _) = **network.node().coin_db().state().load();
    let anchor = network.wallet_db().anchor(state);
    respond_text(anchor.to_string())
}

#[expect(unused_variables)]
async fn tx_count(State(network): State<Arc<Network>>, address: Path<String>) -> Response<String> {
    todo!();
}

#[expect(unused_variables)]
async fn list_transactions(
    State(network): State<Arc<Network>>,
    address: Path<String>,
) -> Json<Vec<WalletTransactionInfo>> {
    todo!();
}

#[expect(unused_variables)]
async fn list_transactions_with_offset(
    State(network): State<Arc<Network>>,
    Path((address, offset)): Path<(String, u32)>,
) -> Json<Vec<WalletTransactionInfo>> {
    todo!();
}

#[expect(unused_variables)]
async fn list_transactions_with_offset_and_max(
    State(network): State<Arc<Network>>,
    Path((address, offset, max)): Path<(String, u32, u32)>,
) -> Json<Vec<WalletTransactionInfo>> {
    todo!();
}

#[expect(unused_variables)]
async fn list_transactions_with_all(
    State(network): State<Arc<Network>>,
    Path((address, offset, max, r#type)): Path<(String, u32, u32, u8)>,
) -> Json<Vec<WalletTransactionInfo>> {
    todo!();
}

#[derive(Deserialize, Serialize)]
pub struct ListSinceBlockInfo {
    pub transactions: Vec<WalletTransactionInfo>,
    pub lastBlockHash: HashInfo,
}

#[expect(unused_variables)]
async fn list_since_block(
    State(network): State<Arc<Network>>,
    address: Path<String>,
) -> Json<ListSinceBlockInfo> {
    todo!();
}

#[expect(unused_variables)]
async fn list_since_block_with_hash(
    State(network): State<Arc<Network>>,
    Path((address, hash)): Path<(String, String)>,
) -> Json<ListSinceBlockInfo> {
    todo!();
}

pub fn routes() -> Router<Arc<Network>> {
    Router::new()
        .route("/api/v2/generateaccount", get(generate_account))
        .route("/api/v2/generateaccount/{wordlist}", get(generate_account))
        .route("/api/v2/address/{address}", get(address))
        .route("/api/v2/mnemonic", post(mnemonic))
        .route("/api/v2/decryptpaymentid", post(decrypt_payment_id))
        .route("/api/v2/decryptmessage", post(decrypt_payment_id))
        .route("/api/v2/signmessage", post(sign_message))
        .route(
            "/api/v2/verifymessage/{from}/{signature}/{message}",
            get(verify_message),
        )
        .route("/api/v2/wallet/{address}/transactions", get(transactions))
        .route("/api/v2/wallet/{address}/outleases", get(out_leases))
        .route("/api/v2/wallet/{address}/sequence", get(sequence))
        .route(
            "/api/v2/wallet/{address}/transaction/{hash}",
            get(transaction),
        )
        .route(
            "/api/v2/wallet/{address}/transaction/{hash}/{raw}",
            get(transaction_raw),
        )
        .route(
            "/api/v2/wallet/{address}/confirmations/{hash}",
            get(confirmations),
        )
        .route("/api/v2/wallet/{address}/referencechain", get(anchor))
        .route("/api/v2/wallet/{address}/txcount", get(tx_count))
        .route(
            "/api/v2/wallet/{address}/listtransactions",
            get(list_transactions),
        )
        .route(
            "/api/v2/wallet/{address}/listtransactions/{offset}",
            get(list_transactions_with_offset),
        )
        .route(
            "/api/v2/wallet/{address}/listtransactions/{offset}/{max}",
            get(list_transactions_with_offset_and_max),
        )
        .route(
            "/api/v2/wallet/{address}/listtransactions/{offset}/{max}/{type}",
            get(list_transactions_with_all),
        )
        .route(
            "/api/v2/wallet/{address}/listsinceblock",
            get(list_since_block),
        )
        .route(
            "/api/v2/wallet/{address}/listsinceblock/{hash}",
            get(list_since_block_with_hash),
        )
}

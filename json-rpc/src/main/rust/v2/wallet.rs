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
    AddressInfo, Hash256Info, LeaseInfo, MnemonicInfo, NewMnemonicInfo, TransactionDataInfo,
    WalletTransactionInfo, response::*,
};
use axum::{
    Form, Json, Router,
    extract::{Path, State},
    response::Response,
    routing::{get, post},
};
use blacknet_crypto::zeroize::ZeroizingString;
use blacknet_kernel::blake2b::Hash256;
use blacknet_network::{db::genesis, network::Network, wallet::Mnemonic};
use core::str::FromStr;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

async fn generate_account(
    State(network): State<Arc<Network>>,
    wordlist: Option<Path<String>>,
) -> Response<String> {
    let address_codec = network.wallet_db().address_codec();
    let wordlist: &str = match &wordlist {
        Some(name) => name.as_str(),
        None => "english",
    };
    let mnemonic = match Mnemonic::generate_v1(wordlist) {
        Ok(mnemonic) => mnemonic,
        Err(err) => return respond_error(format!("Generation error: {err}")),
    };
    let info = match NewMnemonicInfo::new(mnemonic, address_codec) {
        Ok(info) => info,
        Err(err) => return respond_error(format!("Internal error: {err}")),
    };
    respond_json(&info)
}

async fn address(State(network): State<Arc<Network>>, address: Path<String>) -> Response<String> {
    let address_codec = network.wallet_db().address_codec();
    match AddressInfo::new(&address, address_codec) {
        Ok(info) => respond_json(&info),
        Err(err) => respond_error(err.to_string()),
    }
}

#[derive(Deserialize, Serialize)]
pub struct MnemonicRequest {
    pub mnemonic: ZeroizingString,
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
    pub mnemonic: ZeroizingString,
    pub from: String,
    pub message: String,
}

#[expect(unused_variables)]
async fn decrypt_payment_id(
    State(network): State<Arc<Network>>,
    Form(request): Form<DecryptPaymentIdRequest>,
) -> Response<String> {
    let from = match network.wallet_db().address_codec().decode(&request.from) {
        Ok(from) => from,
        Err(err) => {
            return respond_error(format!("Invalid from: {err}"));
        }
    };
    todo!();
}

#[derive(Deserialize, Serialize)]
pub struct SignMessageRequest {
    pub mnemonic: ZeroizingString,
    pub message: String,
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

async fn sequence(State(network): State<Arc<Network>>, address: Path<String>) -> Response<String> {
    let wallet_db = network.wallet_db();
    let public_key = match wallet_db.address_codec().decode(&address) {
        Ok(public_key) => public_key,
        Err(err) => return respond_error(format!("Invalid address: {err}")),
    };
    let Some(wallet) = network.wallet_db().wallet(public_key) else {
        return respond_error("Wallet not found");
    };
    match wallet.sequence() {
        Ok(sequence) => respond_text(sequence.to_string()),
        Err(err) => respond_error(err.to_string()),
    }
}

async fn transaction(
    State(network): State<Arc<Network>>,
    Path((address, hash)): Path<(String, String)>,
) -> Response<String> {
    transaction_handler(network, address, hash, false)
}

async fn transaction_raw(
    State(network): State<Arc<Network>>,
    Path((address, hash, raw)): Path<(String, String, bool)>,
) -> Response<String> {
    transaction_handler(network, address, hash, raw)
}

#[expect(unused_variables, clippy::needless_pass_by_value)]
fn transaction_handler(
    network: Arc<Network>,
    address: String,
    hash: String,
    raw: bool,
) -> Response<String> {
    let wallet_db = network.wallet_db();
    let public_key = match wallet_db.address_codec().decode(&address) {
        Ok(public_key) => public_key,
        Err(err) => return respond_error(format!("Invalid address: {err}")),
    };
    let hash = match Hash256::from_str(hash.as_str()) {
        Ok(hash) => hash,
        Err(err) => return respond_error(format!("Invalid hash: {err}")),
    };
    let Some(wallet) = wallet_db.wallet(public_key) else {
        return respond_error("Wallet not found");
    };
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

async fn tx_count(State(network): State<Arc<Network>>, address: Path<String>) -> Response<String> {
    let wallet_db = network.wallet_db();
    let public_key = match wallet_db.address_codec().decode(&address) {
        Ok(public_key) => public_key,
        Err(err) => return respond_error(format!("Invalid address: {err}")),
    };
    let Some(wallet) = network.wallet_db().wallet(public_key) else {
        return respond_error("Wallet not found");
    };
    match wallet.count_transactions() {
        Ok(count) => respond_text(count.to_string()),
        Err(err) => respond_error(err.to_string()),
    }
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
    pub lastBlockHash: Hash256Info,
}

async fn list_since_block(
    State(network): State<Arc<Network>>,
    Path(address): Path<String>,
) -> Response<String> {
    list_since_block_handler(network, address, genesis::hash())
}

async fn list_since_block_with_hash(
    State(network): State<Arc<Network>>,
    Path((address, hash)): Path<(String, String)>,
) -> Response<String> {
    let hash = match Hash256::from_str(hash.as_str()) {
        Ok(hash) => hash,
        Err(err) => return respond_error(format!("Invalid hash: {err}")),
    };
    list_since_block_handler(network, address, hash)
}

#[expect(unused_variables, clippy::needless_pass_by_value)]
fn list_since_block_handler(
    network: Arc<Network>,
    address: String,
    hash: Hash256,
) -> Response<String> {
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

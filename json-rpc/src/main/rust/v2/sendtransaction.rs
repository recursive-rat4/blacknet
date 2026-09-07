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
use axum::{
    Form, Router,
    extract::{Path, State},
    response::Response,
    routing::get,
    routing::post,
};
use blacknet_kernel::{
    amount::Amount,
    blake2b::Hash,
    ed25519::{to_public_key, to_secret_key},
    hashlock::{HashKind, HashLock},
    timelock::{TimeKind, TimeLock},
    transaction::*,
};
use blacknet_network::{network::Network, wallet::AddressKind};
use blacknet_serialization::format::to_bytes;
use core::str::FromStr;
use data_encoding::HEXUPPER_PERMISSIVE as HEX;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use zeroize::Zeroize;

#[derive(Deserialize, Serialize)]
pub struct BundleRequest {
    pub mnemonic: String,
    pub fee: String,
    pub id: String,
    pub data: String,
    pub referenceChain: Option<String>,
}

impl Drop for BundleRequest {
    fn drop(&mut self) {
        self.mnemonic.zeroize()
    }
}

async fn bundle(
    State(network): State<Arc<Network>>,
    Form(request): Form<BundleRequest>,
) -> Response<String> {
    let message = match HEX.decode(request.data.as_bytes()) {
        Ok(message) => message,
        Err(err) => {
            return respond_error(format!("Invalid hex: {err}"));
        }
    };
    let secret_key = if let Some(secret_key) = to_secret_key(&request.mnemonic) {
        secret_key
    } else {
        return respond_error("Invalid mnemonic");
    };
    let anchor = if let Some(ref anchor) = request.referenceChain {
        match Hash::from_str(anchor) {
            Ok(anchor) => anchor,
            Err(err) => return respond_error(format!("Invalid anchor: {err}")),
        }
    } else {
        let (ref state, _) = **network.node().coin_db().state().load();
        network.wallet_db().anchor(state)
    };
    let fee = match Amount::from_str(request.fee.as_str()) {
        Ok(fee) => fee,
        Err(err) => return respond_error(format!("Invalid fee: {err}")),
    };
    let from = to_public_key(&secret_key);
    let seq = match network.wallet_db().sequence(from) {
        Ok(seq) => seq,
        Err(err) => {
            return respond_error(err.to_string());
        }
    };
    let tag = match network
        .wallet_db()
        .address_codec()
        .decode_with_kind(AddressKind::Blob, &request.id)
    {
        Ok(tg) => {
            let mut tag: Tag = Default::default();
            tag.copy_from_slice(&tg);
            tag
        }
        Err(err) => {
            return respond_error(format!("Invalid id: {err}"));
        }
    };
    let data = match to_bytes(&Blob::new(tag, message.into())) {
        Ok(data) => data,
        Err(err) => {
            return respond_error(format!("Serialization error: {err}"));
        }
    };
    let mut tx = Transaction::new(from, seq, anchor, fee, TxKind::Blob, data.into());
    let (hash, bytes) = tx.sign(&secret_key);

    match network.node().broadcast_tx(hash, &bytes) {
        Ok(()) => respond_text(hash.to_string()),
        Err(msg) => respond_error(format!("Transaction rejected: {msg}")),
    }
}

#[derive(Deserialize, Serialize)]
pub struct BurnRequest {
    pub mnemonic: String,
    pub fee: String,
    pub amount: String,
    pub message: String,
    pub referenceChain: Option<String>,
}

impl Drop for BurnRequest {
    fn drop(&mut self) {
        self.mnemonic.zeroize()
    }
}

async fn burn(
    State(network): State<Arc<Network>>,
    Form(request): Form<BurnRequest>,
) -> Response<String> {
    let message = match HEX.decode(request.message.as_bytes()) {
        Ok(message) => message,
        Err(err) => {
            return respond_error(format!("Invalid hex: {err}"));
        }
    };
    let secret_key = if let Some(secret_key) = to_secret_key(&request.mnemonic) {
        secret_key
    } else {
        return respond_error("Invalid mnemonic");
    };
    let anchor = if let Some(ref anchor) = request.referenceChain {
        match Hash::from_str(anchor) {
            Ok(anchor) => anchor,
            Err(err) => return respond_error(format!("Invalid anchor: {err}")),
        }
    } else {
        let (ref state, _) = **network.node().coin_db().state().load();
        network.wallet_db().anchor(state)
    };
    let fee = match Amount::from_str(request.fee.as_str()) {
        Ok(fee) => fee,
        Err(err) => return respond_error(format!("Invalid fee: {err}")),
    };
    let amount = match Amount::from_str(request.amount.as_str()) {
        Ok(amount) => amount,
        Err(err) => return respond_error(format!("Invalid amount: {err}")),
    };
    let from = to_public_key(&secret_key);
    let seq = match network.wallet_db().sequence(from) {
        Ok(seq) => seq,
        Err(err) => {
            return respond_error(err.to_string());
        }
    };
    let data = match to_bytes(&Burn::new(amount, message.into())) {
        Ok(data) => data,
        Err(err) => {
            return respond_error(format!("Serialization error: {err}"));
        }
    };
    let mut tx = Transaction::new(from, seq, anchor, fee, TxKind::Burn, data.into());
    let (hash, bytes) = tx.sign(&secret_key);

    match network.node().broadcast_tx(hash, &bytes) {
        Ok(()) => respond_text(hash.to_string()),
        Err(msg) => respond_error(format!("Transaction rejected: {msg}")),
    }
}

#[derive(Deserialize, Serialize)]
pub struct CancelLeaseRequest {
    pub mnemonic: String,
    pub fee: String,
    pub amount: String,
    pub to: String,
    pub height: u32,
    pub referenceChain: Option<String>,
}

impl Drop for CancelLeaseRequest {
    fn drop(&mut self) {
        self.mnemonic.zeroize()
    }
}

async fn cancel_lease(
    State(network): State<Arc<Network>>,
    Form(request): Form<CancelLeaseRequest>,
) -> Response<String> {
    let secret_key = if let Some(secret_key) = to_secret_key(&request.mnemonic) {
        secret_key
    } else {
        return respond_error("Invalid mnemonic");
    };
    let anchor = if let Some(ref anchor) = request.referenceChain {
        match Hash::from_str(anchor) {
            Ok(anchor) => anchor,
            Err(err) => return respond_error(format!("Invalid anchor: {err}")),
        }
    } else {
        let (ref state, _) = **network.node().coin_db().state().load();
        network.wallet_db().anchor(state)
    };
    let fee = match Amount::from_str(request.fee.as_str()) {
        Ok(fee) => fee,
        Err(err) => return respond_error(format!("Invalid fee: {err}")),
    };
    let from = to_public_key(&secret_key);
    let amount = match Amount::from_str(request.amount.as_str()) {
        Ok(amount) => amount,
        Err(err) => return respond_error(format!("Invalid amount: {err}")),
    };
    let seq = match network.wallet_db().sequence(from) {
        Ok(seq) => seq,
        Err(err) => {
            return respond_error(err.to_string());
        }
    };
    let to = match network.wallet_db().address_codec().decode(&request.to) {
        Ok(to) => to,
        Err(err) => {
            return respond_error(format!("Invalid to: {err}"));
        }
    };
    let data = match to_bytes(&CancelLease::new(amount, to, request.height)) {
        Ok(data) => data,
        Err(err) => {
            return respond_error(format!("Serialization error: {err}"));
        }
    };
    let mut tx = Transaction::new(from, seq, anchor, fee, TxKind::CancelLease, data.into());
    let (hash, bytes) = tx.sign(&secret_key);

    match network.node().broadcast_tx(hash, &bytes) {
        Ok(()) => respond_text(hash.to_string()),
        Err(msg) => respond_error(format!("Transaction rejected: {msg}")),
    }
}

#[derive(Deserialize, Serialize)]
pub struct ClaimSwapRequest {
    pub mnemonic: String,
    pub fee: String,
    pub id: String,
    pub preimage: String,
    pub referenceChain: Option<String>,
}

impl Drop for ClaimSwapRequest {
    fn drop(&mut self) {
        self.mnemonic.zeroize()
    }
}

async fn claim_swap(
    State(network): State<Arc<Network>>,
    Form(request): Form<ClaimSwapRequest>,
) -> Response<String> {
    let preimage = match HEX.decode(request.preimage.as_bytes()) {
        Ok(preimage) => preimage,
        Err(err) => {
            return respond_error(format!("Invalid hex: {err}"));
        }
    };
    let secret_key = if let Some(secret_key) = to_secret_key(&request.mnemonic) {
        secret_key
    } else {
        return respond_error("Invalid mnemonic");
    };
    let anchor = if let Some(ref anchor) = request.referenceChain {
        match Hash::from_str(anchor) {
            Ok(anchor) => anchor,
            Err(err) => return respond_error(format!("Invalid anchor: {err}")),
        }
    } else {
        let (ref state, _) = **network.node().coin_db().state().load();
        network.wallet_db().anchor(state)
    };
    let fee = match Amount::from_str(request.fee.as_str()) {
        Ok(fee) => fee,
        Err(err) => return respond_error(format!("Invalid fee: {err}")),
    };
    let from = to_public_key(&secret_key);
    let seq = match network.wallet_db().sequence(from) {
        Ok(seq) => seq,
        Err(err) => {
            return respond_error(err.to_string());
        }
    };
    let id = match network
        .wallet_db()
        .address_codec()
        .decode_with_kind(AddressKind::HTLC, &request.id)
    {
        Ok(i) => {
            let mut id: HashTimeLockContractId = Default::default();
            id.copy_from_slice(&i);
            id
        }
        Err(err) => {
            return respond_error(format!("Invalid id: {err}"));
        }
    };
    let data = match to_bytes(&ClaimHTLC::new(id, preimage.into())) {
        Ok(data) => data,
        Err(err) => {
            return respond_error(format!("Serialization error: {err}"));
        }
    };
    let mut tx = Transaction::new(from, seq, anchor, fee, TxKind::ClaimHTLC, data.into());
    let (hash, bytes) = tx.sign(&secret_key);

    match network.node().broadcast_tx(hash, &bytes) {
        Ok(()) => respond_text(hash.to_string()),
        Err(msg) => respond_error(format!("Transaction rejected: {msg}")),
    }
}

#[derive(Deserialize, Serialize)]
pub struct CreateSwapRequest {
    pub mnemonic: String,
    pub fee: String,
    pub amount: String,
    pub to: String,
    pub timeLockType: TimeKind,
    pub timeLockData: i64,
    pub hashLockType: HashKind,
    pub hashLockData: String,
    pub referenceChain: Option<String>,
}

impl Drop for CreateSwapRequest {
    fn drop(&mut self) {
        self.mnemonic.zeroize()
    }
}

async fn create_swap(
    State(network): State<Arc<Network>>,
    Form(request): Form<CreateSwapRequest>,
) -> Response<String> {
    let image = match HEX.decode(request.hashLockData.as_bytes()) {
        Ok(image) => image,
        Err(err) => {
            return respond_error(format!("Invalid hex: {err}"));
        }
    };
    let secret_key = if let Some(secret_key) = to_secret_key(&request.mnemonic) {
        secret_key
    } else {
        return respond_error("Invalid mnemonic");
    };
    let anchor = if let Some(ref anchor) = request.referenceChain {
        match Hash::from_str(anchor) {
            Ok(anchor) => anchor,
            Err(err) => return respond_error(format!("Invalid anchor: {err}")),
        }
    } else {
        let (ref state, _) = **network.node().coin_db().state().load();
        network.wallet_db().anchor(state)
    };
    let fee = match Amount::from_str(request.fee.as_str()) {
        Ok(fee) => fee,
        Err(err) => return respond_error(format!("Invalid fee: {err}")),
    };
    let amount = match Amount::from_str(request.amount.as_str()) {
        Ok(amount) => amount,
        Err(err) => return respond_error(format!("Invalid amount: {err}")),
    };
    let from = to_public_key(&secret_key);
    let seq = match network.wallet_db().sequence(from) {
        Ok(seq) => seq,
        Err(err) => {
            return respond_error(err.to_string());
        }
    };
    let to = match network.wallet_db().address_codec().decode(&request.to) {
        Ok(to) => to,
        Err(err) => {
            return respond_error(format!("Invalid to: {err}"));
        }
    };
    let time_lock = TimeLock::new(request.timeLockType, request.timeLockData);
    let hash_lock = HashLock::new(request.hashLockType, image.into());
    let data = match to_bytes(&CreateHTLC::new(amount, to, time_lock, hash_lock)) {
        Ok(data) => data,
        Err(err) => {
            return respond_error(format!("Serialization error: {err}"));
        }
    };
    let mut tx = Transaction::new(from, seq, anchor, fee, TxKind::CreateHTLC, data.into());
    let (hash, bytes) = tx.sign(&secret_key);

    match network.node().broadcast_tx(hash, &bytes) {
        Ok(()) => respond_text(hash.to_string()),
        Err(msg) => respond_error(format!("Transaction rejected: {msg}")),
    }
}

#[derive(Deserialize, Serialize)]
pub struct LeaseRequest {
    pub mnemonic: String,
    pub fee: String,
    pub amount: String,
    pub to: String,
    pub referenceChain: Option<String>,
}

impl Drop for LeaseRequest {
    fn drop(&mut self) {
        self.mnemonic.zeroize()
    }
}

async fn lease(
    State(network): State<Arc<Network>>,
    Form(request): Form<LeaseRequest>,
) -> Response<String> {
    let secret_key = if let Some(secret_key) = to_secret_key(&request.mnemonic) {
        secret_key
    } else {
        return respond_error("Invalid mnemonic");
    };
    let anchor = if let Some(ref anchor) = request.referenceChain {
        match Hash::from_str(anchor) {
            Ok(anchor) => anchor,
            Err(err) => return respond_error(format!("Invalid anchor: {err}")),
        }
    } else {
        let (ref state, _) = **network.node().coin_db().state().load();
        network.wallet_db().anchor(state)
    };
    let fee = match Amount::from_str(request.fee.as_str()) {
        Ok(fee) => fee,
        Err(err) => return respond_error(format!("Invalid fee: {err}")),
    };
    let amount = match Amount::from_str(request.amount.as_str()) {
        Ok(amount) => amount,
        Err(err) => return respond_error(format!("Invalid amount: {err}")),
    };
    let from = to_public_key(&secret_key);
    let seq = match network.wallet_db().sequence(from) {
        Ok(seq) => seq,
        Err(err) => {
            return respond_error(err.to_string());
        }
    };
    let to = match network.wallet_db().address_codec().decode(&request.to) {
        Ok(to) => to,
        Err(err) => {
            return respond_error(format!("Invalid to: {err}"));
        }
    };
    let data = match to_bytes(&Lease::new(amount, to)) {
        Ok(data) => data,
        Err(err) => {
            return respond_error(format!("Serialization error: {err}"));
        }
    };
    let mut tx = Transaction::new(from, seq, anchor, fee, TxKind::Lease, data.into());
    let (hash, bytes) = tx.sign(&secret_key);

    match network.node().broadcast_tx(hash, &bytes) {
        Ok(()) => respond_text(hash.to_string()),
        Err(msg) => respond_error(format!("Transaction rejected: {msg}")),
    }
}

#[derive(Deserialize, Serialize)]
pub struct RefundSwapRequest {
    pub mnemonic: String,
    pub fee: String,
    pub id: String,
    pub referenceChain: Option<String>,
}

impl Drop for RefundSwapRequest {
    fn drop(&mut self) {
        self.mnemonic.zeroize()
    }
}

async fn refund_swap(
    State(network): State<Arc<Network>>,
    Form(request): Form<RefundSwapRequest>,
) -> Response<String> {
    let secret_key = if let Some(secret_key) = to_secret_key(&request.mnemonic) {
        secret_key
    } else {
        return respond_error("Invalid mnemonic");
    };
    let anchor = if let Some(ref anchor) = request.referenceChain {
        match Hash::from_str(anchor) {
            Ok(anchor) => anchor,
            Err(err) => return respond_error(format!("Invalid anchor: {err}")),
        }
    } else {
        let (ref state, _) = **network.node().coin_db().state().load();
        network.wallet_db().anchor(state)
    };
    let fee = match Amount::from_str(request.fee.as_str()) {
        Ok(fee) => fee,
        Err(err) => return respond_error(format!("Invalid fee: {err}")),
    };
    let from = to_public_key(&secret_key);
    let seq = match network.wallet_db().sequence(from) {
        Ok(seq) => seq,
        Err(err) => {
            return respond_error(err.to_string());
        }
    };
    let id = match network
        .wallet_db()
        .address_codec()
        .decode_with_kind(AddressKind::HTLC, &request.id)
    {
        Ok(i) => {
            let mut id: HashTimeLockContractId = Default::default();
            id.copy_from_slice(&i);
            id
        }
        Err(err) => {
            return respond_error(format!("Invalid id: {err}"));
        }
    };
    let data = match to_bytes(&RefundHTLC::new(id)) {
        Ok(data) => data,
        Err(err) => {
            return respond_error(format!("Serialization error: {err}"));
        }
    };
    let mut tx = Transaction::new(from, seq, anchor, fee, TxKind::RefundHTLC, data.into());
    let (hash, bytes) = tx.sign(&secret_key);

    match network.node().broadcast_tx(hash, &bytes) {
        Ok(()) => respond_text(hash.to_string()),
        Err(msg) => respond_error(format!("Transaction rejected: {msg}")),
    }
}

#[derive(Deserialize, Serialize)]
pub struct TransferRequest {
    pub mnemonic: String,
    pub fee: String,
    pub amount: String,
    pub to: String,
    pub encrypted: Option<u8>,
    pub message: Option<String>,
    pub referenceChain: Option<String>,
}

impl Drop for TransferRequest {
    fn drop(&mut self) {
        self.mnemonic.zeroize()
    }
}

async fn transfer(
    State(network): State<Arc<Network>>,
    Form(mut request): Form<TransferRequest>,
) -> Response<String> {
    let secret_key = if let Some(secret_key) = to_secret_key(&request.mnemonic) {
        secret_key
    } else {
        return respond_error("Invalid mnemonic");
    };
    let anchor = if let Some(ref anchor) = request.referenceChain {
        match Hash::from_str(anchor) {
            Ok(anchor) => anchor,
            Err(err) => return respond_error(format!("Invalid anchor: {err}")),
        }
    } else {
        let (ref state, _) = **network.node().coin_db().state().load();
        network.wallet_db().anchor(state)
    };
    let fee = match Amount::from_str(request.fee.as_str()) {
        Ok(fee) => fee,
        Err(err) => return respond_error(format!("Invalid fee: {err}")),
    };
    let amount = match Amount::from_str(request.amount.as_str()) {
        Ok(amount) => amount,
        Err(err) => return respond_error(format!("Invalid amount: {err}")),
    };
    let from = to_public_key(&secret_key);
    let seq = match network.wallet_db().sequence(from) {
        Ok(seq) => seq,
        Err(err) => {
            return respond_error(err.to_string());
        }
    };
    let to = match network.wallet_db().address_codec().decode(&request.to) {
        Ok(to) => to,
        Err(err) => {
            return respond_error(format!("Invalid to: {err}"));
        }
    };
    let payment_id = {
        if request.encrypted.is_none() || request.encrypted == Some(0) {
            PaymentId::plain(&request.message.take().unwrap_or("".to_owned()))
        } else {
            return respond_error("Unknown encrypted");
        }
    };
    let data = match to_bytes(&Transfer::new(amount, to, payment_id)) {
        Ok(data) => data,
        Err(err) => {
            return respond_error(format!("Serialization error: {err}"));
        }
    };
    let mut tx = Transaction::new(from, seq, anchor, fee, TxKind::Transfer, data.into());
    let (hash, bytes) = tx.sign(&secret_key);

    match network.node().broadcast_tx(hash, &bytes) {
        Ok(()) => respond_text(hash.to_string()),
        Err(msg) => respond_error(format!("Transaction rejected: {msg}")),
    }
}

#[derive(Deserialize, Serialize)]
pub struct WithdrawFromLeaseRequest {
    pub mnemonic: String,
    pub fee: String,
    pub withdraw: String,
    pub amount: String,
    pub to: String,
    pub height: u32,
    pub referenceChain: Option<String>,
}

impl Drop for WithdrawFromLeaseRequest {
    fn drop(&mut self) {
        self.mnemonic.zeroize()
    }
}

async fn withdraw_from_lease(
    State(network): State<Arc<Network>>,
    Form(request): Form<WithdrawFromLeaseRequest>,
) -> Response<String> {
    let secret_key = if let Some(secret_key) = to_secret_key(&request.mnemonic) {
        secret_key
    } else {
        return respond_error("Invalid mnemonic");
    };
    let anchor = if let Some(ref anchor) = request.referenceChain {
        match Hash::from_str(anchor) {
            Ok(anchor) => anchor,
            Err(err) => return respond_error(format!("Invalid anchor: {err}")),
        }
    } else {
        let (ref state, _) = **network.node().coin_db().state().load();
        network.wallet_db().anchor(state)
    };
    let fee = match Amount::from_str(request.fee.as_str()) {
        Ok(fee) => fee,
        Err(err) => return respond_error(format!("Invalid fee: {err}")),
    };
    let withdraw = match Amount::from_str(request.withdraw.as_str()) {
        Ok(withdraw) => withdraw,
        Err(err) => return respond_error(format!("Invalid withdraw: {err}")),
    };
    let amount = match Amount::from_str(request.amount.as_str()) {
        Ok(amount) => amount,
        Err(err) => return respond_error(format!("Invalid amount: {err}")),
    };
    let from = to_public_key(&secret_key);
    let seq = match network.wallet_db().sequence(from) {
        Ok(seq) => seq,
        Err(err) => {
            return respond_error(err.to_string());
        }
    };
    let to = match network.wallet_db().address_codec().decode(&request.to) {
        Ok(to) => to,
        Err(err) => {
            return respond_error(format!("Invalid to: {err}"));
        }
    };
    let data = match to_bytes(&WithdrawFromLease::new(
        withdraw,
        amount,
        to,
        request.height,
    )) {
        Ok(data) => data,
        Err(err) => {
            return respond_error(format!("Serialization error: {err}"));
        }
    };
    let mut tx = Transaction::new(
        from,
        seq,
        anchor,
        fee,
        TxKind::WithdrawFromLease,
        data.into(),
    );
    let (hash, bytes) = tx.sign(&secret_key);

    match network.node().broadcast_tx(hash, &bytes) {
        Ok(()) => respond_text(hash.to_string()),
        Err(msg) => respond_error(format!("Transaction rejected: {msg}")),
    }
}

async fn send_raw_transaction(
    State(network): State<Arc<Network>>,
    Path(hex): Path<String>,
) -> Response<String> {
    let bytes = match HEX.decode(hex.as_bytes()) {
        Ok(bytes) => bytes,
        Err(err) => {
            return respond_error(format!("Invalid hex: {err}"));
        }
    };
    let hash = if let Some(hash) = Transaction::compute_hash(&bytes) {
        hash
    } else {
        return respond_error("Invalid transaction bytes");
    };

    match network.node().broadcast_tx(hash, &bytes) {
        Ok(()) => respond_text(hash.to_string()),
        Err(msg) => respond_error(format!("Transaction rejected: {msg}")),
    }
}

pub fn routes() -> Router<Arc<Network>> {
    Router::new()
        .route("/api/v2/bundle", post(bundle))
        .route("/api/v2/burn", post(burn))
        .route("/api/v2/cancellease", post(cancel_lease))
        .route("/api/v2/claimswap", post(claim_swap))
        .route("/api/v2/createswap", post(create_swap))
        .route("/api/v2/refundswap", post(refund_swap))
        .route("/api/v2/lease", post(lease))
        .route("/api/v2/transfer", post(transfer))
        .route("/api/v2/withdrawfromlease", post(withdraw_from_lease))
        .route(
            "/api/v2/sendrawtransaction/{hex}",
            get(send_raw_transaction),
        )
}

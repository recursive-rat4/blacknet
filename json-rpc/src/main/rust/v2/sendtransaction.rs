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
use axum::{
    Form, Router,
    extract::{Path, State},
    response::Response,
    routing::get,
    routing::post,
};
use blacknet_kernel::amount::Amount;
use blacknet_kernel::blake2b::Hash;
use blacknet_kernel::ed25519::{to_private_key, to_public_key};
use blacknet_kernel::hashlock::HashLock;
use blacknet_kernel::timelock::TimeLock;
use blacknet_kernel::transaction::*;
use blacknet_network::node::Node;
use blacknet_serialization::format::to_bytes;
use blacknet_wallet::address::AddressKind;
use data_encoding::HEXUPPER_PERMISSIVE as HEX;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct BundleRequest {
    pub mnemonic: String,
    pub fee: Amount,
    pub id: String,
    pub data: String,
    pub referenceChain: Option<Hash>,
}

async fn bundle(
    State(node): State<Arc<Node>>,
    Form(request): Form<BundleRequest>,
) -> Response<String> {
    let message = match HEX.decode(request.data.as_bytes()) {
        Ok(message) => message,
        Err(err) => {
            return respond_error(format!("Invalid hex: {err}"));
        }
    };
    let private_key = if let Some(private_key) = to_private_key(&request.mnemonic) {
        private_key
    } else {
        return respond_error("Invalid mnemonic".to_owned());
    };
    let anchor = if let Some(anchor) = request.referenceChain {
        anchor
    } else {
        node.wallet_db().anchor()
    };
    let from = to_public_key(private_key);
    let seq = match node.wallet_db().sequence(from) {
        Some(seq) => seq,
        None => {
            return respond_error("Unknown wallet".to_owned());
        }
    };
    let tag = match node
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
    let mut tx = Transaction::new(from, seq, anchor, request.fee, TxKind::Blob, data.into());
    let (hash, bytes) = tx.sign(private_key);

    match node.broadcast_tx(hash, &bytes) {
        Ok(()) => respond_text(hash.to_string()),
        Err(msg) => respond_error(format!("Transaction rejected: {msg}")),
    }
}

#[derive(Deserialize, Serialize)]
pub struct BurnRequest {
    pub mnemonic: String,
    pub fee: Amount,
    pub amount: Amount,
    pub message: String,
    pub referenceChain: Option<Hash>,
}

async fn burn(State(node): State<Arc<Node>>, Form(request): Form<BurnRequest>) -> Response<String> {
    let message = match HEX.decode(request.message.as_bytes()) {
        Ok(message) => message,
        Err(err) => {
            return respond_error(format!("Invalid hex: {err}"));
        }
    };
    let private_key = if let Some(private_key) = to_private_key(&request.mnemonic) {
        private_key
    } else {
        return respond_error("Invalid mnemonic".to_owned());
    };
    let anchor = if let Some(anchor) = request.referenceChain {
        anchor
    } else {
        node.wallet_db().anchor()
    };
    let from = to_public_key(private_key);
    let seq = match node.wallet_db().sequence(from) {
        Some(seq) => seq,
        None => {
            return respond_error("Unknown wallet".to_owned());
        }
    };
    let data = match to_bytes(&Burn::new(request.amount, message.into())) {
        Ok(data) => data,
        Err(err) => {
            return respond_error(format!("Serialization error: {err}"));
        }
    };
    let mut tx = Transaction::new(from, seq, anchor, request.fee, TxKind::Burn, data.into());
    let (hash, bytes) = tx.sign(private_key);

    match node.broadcast_tx(hash, &bytes) {
        Ok(()) => respond_text(hash.to_string()),
        Err(msg) => respond_error(format!("Transaction rejected: {msg}")),
    }
}

#[derive(Deserialize, Serialize)]
pub struct CancelLeaseRequest {
    pub mnemonic: String,
    pub fee: Amount,
    pub amount: Amount,
    pub to: String,
    pub height: u32,
    pub referenceChain: Option<Hash>,
}

async fn cancel_lease(
    State(node): State<Arc<Node>>,
    Form(request): Form<CancelLeaseRequest>,
) -> Response<String> {
    let private_key = if let Some(private_key) = to_private_key(&request.mnemonic) {
        private_key
    } else {
        return respond_error("Invalid mnemonic".to_owned());
    };
    let anchor = if let Some(anchor) = request.referenceChain {
        anchor
    } else {
        node.wallet_db().anchor()
    };
    let from = to_public_key(private_key);
    let seq = match node.wallet_db().sequence(from) {
        Some(seq) => seq,
        None => {
            return respond_error("Unknown wallet".to_owned());
        }
    };
    let to = match node.wallet_db().address_codec().decode(&request.to) {
        Ok(to) => to,
        Err(err) => {
            return respond_error(format!("Invalid to: {err}"));
        }
    };
    let data = match to_bytes(&CancelLease::new(request.amount, to, request.height)) {
        Ok(data) => data,
        Err(err) => {
            return respond_error(format!("Serialization error: {err}"));
        }
    };
    let mut tx = Transaction::new(
        from,
        seq,
        anchor,
        request.fee,
        TxKind::CancelLease,
        data.into(),
    );
    let (hash, bytes) = tx.sign(private_key);

    match node.broadcast_tx(hash, &bytes) {
        Ok(()) => respond_text(hash.to_string()),
        Err(msg) => respond_error(format!("Transaction rejected: {msg}")),
    }
}

#[derive(Deserialize, Serialize)]
pub struct ClaimSwapRequest {
    pub mnemonic: String,
    pub fee: Amount,
    pub id: String,
    pub preimage: String,
    pub referenceChain: Option<Hash>,
}

async fn claim_swap(
    State(node): State<Arc<Node>>,
    Form(request): Form<ClaimSwapRequest>,
) -> Response<String> {
    let preimage = match HEX.decode(request.preimage.as_bytes()) {
        Ok(preimage) => preimage,
        Err(err) => {
            return respond_error(format!("Invalid hex: {err}"));
        }
    };
    let private_key = if let Some(private_key) = to_private_key(&request.mnemonic) {
        private_key
    } else {
        return respond_error("Invalid mnemonic".to_owned());
    };
    let anchor = if let Some(anchor) = request.referenceChain {
        anchor
    } else {
        node.wallet_db().anchor()
    };
    let from = to_public_key(private_key);
    let seq = match node.wallet_db().sequence(from) {
        Some(seq) => seq,
        None => {
            return respond_error("Unknown wallet".to_owned());
        }
    };
    let id = match node
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
    let mut tx = Transaction::new(
        from,
        seq,
        anchor,
        request.fee,
        TxKind::ClaimHTLC,
        data.into(),
    );
    let (hash, bytes) = tx.sign(private_key);

    match node.broadcast_tx(hash, &bytes) {
        Ok(()) => respond_text(hash.to_string()),
        Err(msg) => respond_error(format!("Transaction rejected: {msg}")),
    }
}

#[derive(Deserialize, Serialize)]
pub struct CreateSwapRequest {
    pub mnemonic: String,
    pub fee: Amount,
    pub amount: Amount,
    pub to: String,
    pub timeLockType: u8,
    pub timeLockData: i64,
    pub hashLockType: u8,
    pub hashLockData: String,
    pub referenceChain: Option<Hash>,
}

async fn create_swap(
    State(node): State<Arc<Node>>,
    Form(request): Form<CreateSwapRequest>,
) -> Response<String> {
    let image = match HEX.decode(request.hashLockData.as_bytes()) {
        Ok(image) => image,
        Err(err) => {
            return respond_error(format!("Invalid hex: {err}"));
        }
    };
    let private_key = if let Some(private_key) = to_private_key(&request.mnemonic) {
        private_key
    } else {
        return respond_error("Invalid mnemonic".to_owned());
    };
    let anchor = if let Some(anchor) = request.referenceChain {
        anchor
    } else {
        node.wallet_db().anchor()
    };
    let from = to_public_key(private_key);
    let seq = match node.wallet_db().sequence(from) {
        Some(seq) => seq,
        None => {
            return respond_error("Unknown wallet".to_owned());
        }
    };
    let to = match node.wallet_db().address_codec().decode(&request.to) {
        Ok(to) => to,
        Err(err) => {
            return respond_error(format!("Invalid to: {err}"));
        }
    };
    let time_lock = TimeLock::new(request.timeLockType, request.timeLockData);
    let hash_lock = HashLock::new(request.hashLockType, image.into());
    let data = match to_bytes(&CreateHTLC::new(request.amount, to, time_lock, hash_lock)) {
        Ok(data) => data,
        Err(err) => {
            return respond_error(format!("Serialization error: {err}"));
        }
    };
    let mut tx = Transaction::new(
        from,
        seq,
        anchor,
        request.fee,
        TxKind::CreateHTLC,
        data.into(),
    );
    let (hash, bytes) = tx.sign(private_key);

    match node.broadcast_tx(hash, &bytes) {
        Ok(()) => respond_text(hash.to_string()),
        Err(msg) => respond_error(format!("Transaction rejected: {msg}")),
    }
}

#[derive(Deserialize, Serialize)]
pub struct LeaseRequest {
    pub mnemonic: String,
    pub fee: Amount,
    pub amount: Amount,
    pub to: String,
    pub referenceChain: Option<Hash>,
}

async fn lease(
    State(node): State<Arc<Node>>,
    Form(request): Form<LeaseRequest>,
) -> Response<String> {
    let private_key = if let Some(private_key) = to_private_key(&request.mnemonic) {
        private_key
    } else {
        return respond_error("Invalid mnemonic".to_owned());
    };
    let anchor = if let Some(anchor) = request.referenceChain {
        anchor
    } else {
        node.wallet_db().anchor()
    };
    let from = to_public_key(private_key);
    let seq = match node.wallet_db().sequence(from) {
        Some(seq) => seq,
        None => {
            return respond_error("Unknown wallet".to_owned());
        }
    };
    let to = match node.wallet_db().address_codec().decode(&request.to) {
        Ok(to) => to,
        Err(err) => {
            return respond_error(format!("Invalid to: {err}"));
        }
    };
    let data = match to_bytes(&Lease::new(request.amount, to)) {
        Ok(data) => data,
        Err(err) => {
            return respond_error(format!("Serialization error: {err}"));
        }
    };
    let mut tx = Transaction::new(from, seq, anchor, request.fee, TxKind::Lease, data.into());
    let (hash, bytes) = tx.sign(private_key);

    match node.broadcast_tx(hash, &bytes) {
        Ok(()) => respond_text(hash.to_string()),
        Err(msg) => respond_error(format!("Transaction rejected: {msg}")),
    }
}

#[derive(Deserialize, Serialize)]
pub struct RefundSwapRequest {
    pub mnemonic: String,
    pub fee: Amount,
    pub id: String,
    pub referenceChain: Option<Hash>,
}

async fn refund_swap(
    State(node): State<Arc<Node>>,
    Form(request): Form<RefundSwapRequest>,
) -> Response<String> {
    let private_key = if let Some(private_key) = to_private_key(&request.mnemonic) {
        private_key
    } else {
        return respond_error("Invalid mnemonic".to_owned());
    };
    let anchor = if let Some(anchor) = request.referenceChain {
        anchor
    } else {
        node.wallet_db().anchor()
    };
    let from = to_public_key(private_key);
    let seq = match node.wallet_db().sequence(from) {
        Some(seq) => seq,
        None => {
            return respond_error("Unknown wallet".to_owned());
        }
    };
    let id = match node
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
    let mut tx = Transaction::new(
        from,
        seq,
        anchor,
        request.fee,
        TxKind::RefundHTLC,
        data.into(),
    );
    let (hash, bytes) = tx.sign(private_key);

    match node.broadcast_tx(hash, &bytes) {
        Ok(()) => respond_text(hash.to_string()),
        Err(msg) => respond_error(format!("Transaction rejected: {msg}")),
    }
}

#[derive(Deserialize, Serialize)]
pub struct TransferRequest {
    pub mnemonic: String,
    pub fee: Amount,
    pub amount: Amount,
    pub to: String,
    pub encrypted: Option<u8>,
    pub message: Option<String>,
    pub referenceChain: Option<Hash>,
}

async fn transfer(
    State(node): State<Arc<Node>>,
    Form(request): Form<TransferRequest>,
) -> Response<String> {
    let private_key = if let Some(private_key) = to_private_key(&request.mnemonic) {
        private_key
    } else {
        return respond_error("Invalid mnemonic".to_owned());
    };
    let anchor = if let Some(anchor) = request.referenceChain {
        anchor
    } else {
        node.wallet_db().anchor()
    };
    let from = to_public_key(private_key);
    let seq = match node.wallet_db().sequence(from) {
        Some(seq) => seq,
        None => {
            return respond_error("Unknown wallet".to_owned());
        }
    };
    let to = match node.wallet_db().address_codec().decode(&request.to) {
        Ok(to) => to,
        Err(err) => {
            return respond_error(format!("Invalid to: {err}"));
        }
    };
    let payment_id = {
        if request.encrypted.is_none() || request.encrypted == Some(0) {
            PaymentId::plain(&request.message.unwrap_or("".to_owned()))
        } else {
            return respond_error("Unknown encrypted".to_owned());
        }
    };
    let data = match to_bytes(&Transfer::new(request.amount, to, payment_id)) {
        Ok(data) => data,
        Err(err) => {
            return respond_error(format!("Serialization error: {err}"));
        }
    };
    let mut tx = Transaction::new(
        from,
        seq,
        anchor,
        request.fee,
        TxKind::Transfer,
        data.into(),
    );
    let (hash, bytes) = tx.sign(private_key);

    match node.broadcast_tx(hash, &bytes) {
        Ok(()) => respond_text(hash.to_string()),
        Err(msg) => respond_error(format!("Transaction rejected: {msg}")),
    }
}

#[derive(Deserialize, Serialize)]
pub struct WithdrawFromLeaseRequest {
    pub mnemonic: String,
    pub fee: Amount,
    pub withdraw: Amount,
    pub amount: Amount,
    pub to: String,
    pub height: u32,
    pub referenceChain: Option<Hash>,
}

async fn withdraw_from_lease(
    State(node): State<Arc<Node>>,
    Form(request): Form<WithdrawFromLeaseRequest>,
) -> Response<String> {
    let private_key = if let Some(private_key) = to_private_key(&request.mnemonic) {
        private_key
    } else {
        return respond_error("Invalid mnemonic".to_owned());
    };
    let anchor = if let Some(anchor) = request.referenceChain {
        anchor
    } else {
        node.wallet_db().anchor()
    };
    let from = to_public_key(private_key);
    let seq = match node.wallet_db().sequence(from) {
        Some(seq) => seq,
        None => {
            return respond_error("Unknown wallet".to_owned());
        }
    };
    let to = match node.wallet_db().address_codec().decode(&request.to) {
        Ok(to) => to,
        Err(err) => {
            return respond_error(format!("Invalid to: {err}"));
        }
    };
    let data = match to_bytes(&WithdrawFromLease::new(
        request.withdraw,
        request.amount,
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
        request.fee,
        TxKind::WithdrawFromLease,
        data.into(),
    );
    let (hash, bytes) = tx.sign(private_key);

    match node.broadcast_tx(hash, &bytes) {
        Ok(()) => respond_text(hash.to_string()),
        Err(msg) => respond_error(format!("Transaction rejected: {msg}")),
    }
}

async fn send_raw_transaction(
    State(node): State<Arc<Node>>,
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
        return respond_error("Invalid transaction bytes".to_owned());
    };

    match node.broadcast_tx(hash, &bytes) {
        Ok(()) => respond_text(hash.to_string()),
        Err(msg) => respond_error(format!("Transaction rejected: {msg}")),
    }
}

pub fn routes() -> Router<Arc<Node>> {
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

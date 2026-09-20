/*
 * Copyright (c) 2018-2026 Pavel Vasin
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

use crate::{
    amount::Amount,
    blake2b::Hash256,
    ed25519::{PublicKey, SecretKey},
    error::{Error, Result},
    transaction::{CoinTx, Transaction, TxData},
    x25519::x25519,
};
use alloc::{boxed::Box, string::String, vec};
use blacknet_crypto::symmetric::{ChaCha20, chacha::IV_SIZE};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Copy, Default, Deserialize_repr, Eq, PartialEq, Serialize_repr)]
#[repr(u8)]
pub enum PayloadKind {
    #[default]
    Plain = 0,
    X25519Chacha20 = 1,
}

#[derive(Default, Deserialize, Serialize)]
pub struct PaymentId {
    kind: PayloadKind,
    payload: Box<[u8]>,
}

impl PaymentId {
    pub fn plain(payload: &str) -> Self {
        Self {
            kind: PayloadKind::Plain,
            payload: payload.as_bytes().into(),
        }
    }

    pub const fn kind(&self) -> PayloadKind {
        self.kind
    }

    pub const fn payload(&self) -> &[u8] {
        &self.payload
    }

    pub fn decrypt(secret_key: &SecretKey, public_key: PublicKey, hex: &str) -> Result<String> {
        let bytes = const_hex::decode(hex).map_err(|_| Error::invalid("Invalid hex"))?;
        let Some((iv, ct)) = bytes.split_at_checked(IV_SIZE) else {
            return Err(Error::invalid("Too short hex"));
        };
        let shared_key = x25519(secret_key, public_key)?;
        let mut chacha = ChaCha20::new(shared_key.as_ref(), iv.try_into().unwrap());
        let mut pt = vec![0u8; ct.len()];
        chacha.decrypt(&mut pt, ct);
        let decrypted = String::from_utf8(pt).map_err(|_| Error::invalid("Invalid UTF-8"))?;
        Ok(decrypted)
    }
}

#[derive(Deserialize, Serialize)]
pub struct Transfer {
    amount: Amount,
    to: PublicKey,
    payment_id: PaymentId,
}

impl Transfer {
    pub const fn new(amount: Amount, to: PublicKey, payment_id: PaymentId) -> Self {
        Self {
            amount,
            to,
            payment_id,
        }
    }

    pub const fn amount(&self) -> Amount {
        self.amount
    }

    pub const fn to(&self) -> PublicKey {
        self.to
    }

    pub const fn payment_id(&self) -> &PaymentId {
        &self.payment_id
    }
}

impl TxData for Transfer {
    fn process_impl(
        &self,
        tx: &Transaction,
        _hash: Hash256,
        _data_index: u32,
        coin_tx: &mut impl CoinTx,
    ) -> Result<()> {
        let mut account = coin_tx.get_account(tx.from())?;
        account.credit(self.amount)?;
        coin_tx.set_account(tx.from(), account);
        let mut to_account = coin_tx.get_or_create(self.to);
        to_account.debit(coin_tx.height(), self.amount);
        coin_tx.set_account(self.to, to_account);
        Ok(())
    }
}

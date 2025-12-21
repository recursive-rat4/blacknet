/*
 * Copyright (c) 2018-2025 Pavel Vasin
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

use crate::amount::Amount;
use crate::blake2b::Hash;
use crate::ed25519::PublicKey;
use crate::error::Result;
use crate::transaction::{CoinTx, Transaction, TxData};
use alloc::boxed::Box;
use serde::{Deserialize, Serialize};

const PLAIN: u8 = 0;
#[expect(dead_code)]
const X25519_CHACHA20: u8 = 1;

#[derive(Default, Deserialize, Serialize)]
pub struct PaymentId {
    kind: u8,
    payload: Box<[u8]>,
}

impl PaymentId {
    pub fn plain(payload: &str) -> Self {
        Self {
            kind: PLAIN,
            payload: payload.as_bytes().into(),
        }
    }

    pub const fn kind(&self) -> u8 {
        self.kind
    }

    pub const fn payload(&self) -> &[u8] {
        &self.payload
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
        tx: Transaction,
        _hash: Hash,
        _data_index: u32,
        coin_tx: &mut (impl CoinTx + ?Sized),
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

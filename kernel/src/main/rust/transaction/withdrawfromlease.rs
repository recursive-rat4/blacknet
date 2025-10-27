/*
 * Copyright (c) 2019-2025 Pavel Vasin
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
use crate::error::{Error, Result};
use crate::transaction::{CoinTx, Transaction, TxData};
use alloc::borrow::ToOwned;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct WithdrawFromLease {
    withdraw: Amount,
    amount: Amount,
    to: PublicKey,
    height: u32,
}

impl WithdrawFromLease {
    pub fn withdraw(&self) -> Amount {
        self.withdraw
    }

    pub fn amount(&self) -> Amount {
        self.amount
    }

    pub fn to(&self) -> PublicKey {
        self.to
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

impl TxData for WithdrawFromLease {
    fn process_impl(
        &self,
        tx: Transaction,
        _hash: Hash,
        _data_index: u32,
        coin_tx: impl CoinTx,
    ) -> Result<()> {
        if self.withdraw == Amount::ZERO || self.withdraw > self.amount {
            return Err(Error::Invalid("Invalid withdraw amount".to_owned()));
        }
        let mut to_account = coin_tx.get_account(self.to)?;
        to_account.withdraw_from_lease(self.withdraw, self.amount, self.to, self.height)?;
        coin_tx.set_account(self.to, to_account);
        let mut account = coin_tx.get_account(tx.from())?;
        account.debit(coin_tx.height(), self.withdraw);
        coin_tx.set_account(tx.from(), account);
        Ok(())
    }
}

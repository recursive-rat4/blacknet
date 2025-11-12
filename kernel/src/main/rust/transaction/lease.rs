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
use crate::error::{Error, Result};
use crate::proofofstake::MIN_LEASE;
use crate::transaction::{CoinTx, Transaction, TxData};
use alloc::format;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Lease {
    amount: Amount,
    to: PublicKey,
}

impl Lease {
    pub const fn new(amount: Amount, to: PublicKey) -> Self {
        Self { amount, to }
    }

    pub const fn amount(&self) -> Amount {
        self.amount
    }

    pub const fn to(&self) -> PublicKey {
        self.to
    }
}

impl TxData for Lease {
    fn process_impl(
        &self,
        tx: Transaction,
        _hash: Hash,
        _data_index: u32,
        coin_tx: impl CoinTx,
    ) -> Result<()> {
        if self.amount < MIN_LEASE {
            return Err(Error::Invalid(format!(
                "{0} less than minimal {MIN_LEASE}",
                self.amount
            )));
        }
        let mut account = coin_tx.get_account(tx.from())?;
        account.credit(self.amount)?;
        coin_tx.set_account(tx.from(), account);
        let mut to_account = coin_tx.get_or_create(self.to);
        to_account.add_lease(tx.from(), coin_tx.height(), self.amount);
        coin_tx.set_account(self.to, to_account);
        Ok(())
    }
}

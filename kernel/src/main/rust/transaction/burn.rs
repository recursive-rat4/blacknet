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
use crate::error::{Error, Result};
use crate::transaction::{CoinTx, Transaction, TxData};
use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Burn {
    amount: Amount,
    message: Box<[u8]>,
}

impl TxData for Burn {
    fn process_impl(
        &self,
        tx: Transaction,
        _hash: Hash,
        _data_index: u32,
        coin_tx: impl CoinTx,
    ) -> Result<()> {
        if self.amount == Amount::ZERO {
            return Err(Error::Invalid("Invalid amount".to_owned()));
        }
        let mut account = coin_tx.get_account(tx.from)?;
        account.credit(self.amount)?;
        coin_tx.set_account(tx.from, account);
        coin_tx.sub_supply(self.amount);
        Ok(())
    }
}

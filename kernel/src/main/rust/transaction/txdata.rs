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

use crate::blake2b::Hash;
use crate::error::{Error, Result};
use crate::transaction::{CoinTx, Transaction};
use alloc::format;

pub trait TxData {
    fn process_impl(
        &self,
        tx: Transaction,
        hash: Hash,
        data_index: u32,
        coin_tx: impl CoinTx,
    ) -> Result<()>;

    fn process(&self, tx: Transaction, hash: Hash, coin_tx: impl CoinTx) -> Result<()> {
        let mut account = coin_tx.get_account(tx.from())?;
        if tx.seq() != account.seq() {
            let msg = format!("sequence {} expected {}", tx.seq(), account.seq());
            if tx.seq() < account.seq() {
                return Err(Error::AlreadyHave(msg));
            } else {
                return Err(Error::InFuture(msg));
            }
        }
        account.credit(tx.fee())?;
        account.increment_seq();
        coin_tx.set_account(tx.from(), account);
        self.process_impl(tx, hash, 0, coin_tx)
    }
}

/*
 * Copyright (c) 2020-2025 Pavel Vasin
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
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Dispel;

impl Dispel {
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {}
    }
}

impl TxData for Dispel {
    fn process_impl(
        &self,
        tx: Transaction,
        _hash: Hash,
        _data_index: u32,
        _coin_tx: impl CoinTx,
    ) -> Result<()> {
        if tx.fee() > Amount::ZERO {
            Ok(())
        } else {
            Err(Error::Invalid("Invalid transaction fee".to_owned()))
        }
    }
}

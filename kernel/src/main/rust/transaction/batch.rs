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

use crate::blake2b::Hash;
use crate::error::Result;
use crate::transaction::{CoinTx, Transaction, TxData, TxKind};
use alloc::boxed::Box;
use serde::{Deserialize, Serialize};

pub const MIN_SIZE: usize = 2;
pub const MAX_SIZE: usize = 20;

#[derive(Deserialize, Serialize)]
pub struct Batchee {
    kind: TxKind,
    data: Box<[u8]>,
}

impl Batchee {
    pub const fn kind(&self) -> TxKind {
        self.kind
    }

    pub const fn raw_data(&self) -> &[u8] {
        &self.data
    }
}

#[derive(Deserialize, Serialize)]
pub struct Batch {
    multi_data: Box<[Batchee]>,
}

impl Batch {
    pub const fn new(multi_data: Box<[Batchee]>) -> Self {
        Self { multi_data }
    }

    pub const fn len(&self) -> usize {
        self.multi_data.len()
    }

    pub const fn multi_data(&self) -> &[Batchee] {
        &self.multi_data
    }
}

impl TxData for Batch {
    fn process_impl(
        &self,
        _tx: Transaction,
        _hash: Hash,
        _data_index: u32,
        _coin_tx: &mut (impl CoinTx + ?Sized),
    ) -> Result<()> {
        todo!();
    }
}

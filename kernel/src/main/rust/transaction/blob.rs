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
use crate::error::Result;
use crate::transaction::{CoinTx, Transaction, TxData};
use alloc::boxed::Box;
use serde::{Deserialize, Serialize};

pub type Tag = [u8; 4];

#[derive(Deserialize, Serialize)]
pub struct Blob {
    tag: Tag,
    data: Box<[u8]>,
}

impl Blob {
    pub fn tag(&self) -> Tag {
        self.tag
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

impl TxData for Blob {
    fn process_impl(
        &self,
        _tx: Transaction,
        _hash: Hash,
        _data_index: u32,
        _coin_tx: impl CoinTx,
    ) -> Result<()> {
        Ok(())
    }
}

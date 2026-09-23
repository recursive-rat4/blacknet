/*
 * Copyright (c) 2019-2026 Pavel Vasin
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
    blake2b::Hash256,
    error::{Error, Result},
    transaction::*,
};
use alloc::{boxed::Box, format};
use blacknet_serialization::from_bytes;
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

    pub const fn data_bytes(&self) -> &[u8] {
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

    pub const fn is_empty(&self) -> bool {
        self.multi_data.is_empty()
    }

    pub const fn len(&self) -> usize {
        self.multi_data.len()
    }

    pub const fn multi_data(&self) -> &[Batchee] {
        &self.multi_data
    }

    fn process_inner<T: TxData + for<'de> Deserialize<'de>>(
        &self,
        data_bytes: &[u8],
        tx: &Transaction,
        hash: Hash256,
        idx: usize,
        coin_tx: &mut impl CoinTx,
    ) -> Result<()> {
        let data = from_bytes::<T>(data_bytes, false)?;
        data.process_impl(tx, hash, (idx + 1) as u32, coin_tx)
    }
}

impl TxData for Batch {
    fn process_impl(
        &self,
        tx: &Transaction,
        hash: Hash256,
        data_index: u32,
        coin_tx: &mut impl CoinTx,
    ) -> Result<()> {
        if data_index != 0 {
            return Err(Error::invalid("Batch is not permitted to contain Batch"));
        }
        let len = self.multi_data.len();
        if !(MIN_SIZE..=MAX_SIZE).contains(&len) {
            return Err(Error::invalid(format!("Invalid Batch size {len}")));
        }

        for idx in 0..len {
            let kind = self.multi_data[idx].kind();
            let data_bytes = self.multi_data[idx].data_bytes();
            match kind {
                TxKind::Transfer => {
                    self.process_inner::<Transfer>(data_bytes, tx, hash, idx, coin_tx)?
                }
                TxKind::Burn => self.process_inner::<Burn>(data_bytes, tx, hash, idx, coin_tx)?,
                TxKind::Lease => self.process_inner::<Lease>(data_bytes, tx, hash, idx, coin_tx)?,
                TxKind::CancelLease => {
                    self.process_inner::<CancelLease>(data_bytes, tx, hash, idx, coin_tx)?
                }
                TxKind::Blob => self.process_inner::<Blob>(data_bytes, tx, hash, idx, coin_tx)?,
                TxKind::CreateHTLC => {
                    self.process_inner::<CreateHTLC>(data_bytes, tx, hash, idx, coin_tx)?
                }
                TxKind::RefundHTLC => {
                    self.process_inner::<RefundHTLC>(data_bytes, tx, hash, idx, coin_tx)?
                }
                TxKind::CreateMultisig => {
                    self.process_inner::<CreateMultisig>(data_bytes, tx, hash, idx, coin_tx)?
                }
                TxKind::SpendMultisig => {
                    self.process_inner::<SpendMultisig>(data_bytes, tx, hash, idx, coin_tx)?
                }
                TxKind::WithdrawFromLease => {
                    self.process_inner::<WithdrawFromLease>(data_bytes, tx, hash, idx, coin_tx)?
                }
                TxKind::ClaimHTLC => {
                    self.process_inner::<ClaimHTLC>(data_bytes, tx, hash, idx, coin_tx)?
                }
                TxKind::Batch => self.process_inner::<Batch>(data_bytes, tx, hash, idx, coin_tx)?,
                TxKind::Generated => {
                    return Err(Error::invalid("Generated as individual tx"));
                }
            }
        }

        Ok(())
    }
}

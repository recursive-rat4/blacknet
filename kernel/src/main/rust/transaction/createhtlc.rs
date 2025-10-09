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
use crate::blake2b::{Blake2b256, Hash};
use crate::ed25519::PublicKey;
use crate::error::{Error, Result};
use crate::hashlock::HashLock;
use crate::htlc::HTLC;
use crate::timelock::TimeLock;
use crate::transaction::{CoinTx, Transaction, TxData};
use alloc::borrow::ToOwned;
use digest::Digest;
use serde::{Deserialize, Serialize};

pub type HashTimeLockContractId = [u8; 32];

fn id(hash: Hash, data_index: u32) -> HashTimeLockContractId {
    let mut hasher = Blake2b256::new();
    hasher.update(hash);
    hasher.update(data_index.to_be_bytes());
    hasher.finalize().into()
}

#[derive(Deserialize, Serialize)]
pub struct CreateHTLC {
    amount: Amount,
    to: PublicKey,
    time_lock: TimeLock,
    hash_lock: HashLock,
}

impl TxData for CreateHTLC {
    fn process_impl(
        &self,
        tx: Transaction,
        hash: Hash,
        data_index: u32,
        coin_tx: impl CoinTx,
    ) -> Result<()> {
        self.time_lock.validate()?;
        self.hash_lock.validate()?;

        if self.amount == Amount::ZERO {
            return Err(Error::Invalid("Invalid amount".to_owned()));
        }

        let mut account = coin_tx.get_account(tx.from)?;
        account.credit(self.amount)?;

        let id = id(hash, data_index);
        let htlc = HTLC {
            height: coin_tx.height(),
            time: coin_tx.block_time(),
            amount: self.amount,
            from: tx.from,
            to: self.to,
            time_lock: self.time_lock.clone(),
            hash_lock: self.hash_lock.clone(),
        };
        coin_tx.set_account(tx.from, account);
        coin_tx.add_htlc(id, htlc);
        Ok(())
    }
}

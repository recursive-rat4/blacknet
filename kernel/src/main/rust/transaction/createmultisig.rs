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
use crate::ed25519::{PublicKey, Signature, verify};
use crate::error::{Error, Result};
use crate::multisig::{Deposit, Multisig};
use crate::transaction::{CoinTx, Transaction, TxData};
use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use blacknet_serialization::format::to_bytes;
use digest::Digest;
use serde::{Deserialize, Serialize};

pub type MultiSignatureLockContractId = [u8; 32];

fn id(hash: Hash, data_index: u32) -> MultiSignatureLockContractId {
    let mut hasher = Blake2b256::new();
    hasher.update(hash);
    hasher.update(data_index.to_be_bytes());
    hasher.finalize().into()
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct Dep {
    from: PublicKey,
    amount: Amount,
}

impl Dep {
    pub const fn from(self) -> PublicKey {
        self.from
    }

    pub const fn amount(self) -> Amount {
        self.amount
    }
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct Sig {
    index: u8,
    signature: Signature,
}

impl Sig {
    pub const fn index(self) -> u8 {
        self.index
    }

    pub const fn signature(self) -> Signature {
        self.signature
    }
}

#[derive(Deserialize, Serialize)]
pub struct CreateMultisig {
    n: u8,
    deposits: Box<[Dep]>,
    signatures: Box<[Sig]>,
}

impl CreateMultisig {
    pub const fn new(n: u8, deposits: Box<[Dep]>, signatures: Box<[Sig]>) -> Self {
        Self {
            n,
            deposits,
            signatures,
        }
    }

    pub const fn n(&self) -> u8 {
        self.n
    }

    pub const fn deposits(&self) -> &[Dep] {
        &self.deposits
    }

    pub const fn signatures(&self) -> &[Sig] {
        &self.signatures
    }

    fn hash(&self, from: PublicKey, seq: u32, data_index: u32) -> Result<Hash> {
        let copy = Self {
            n: self.n,
            deposits: self.deposits.clone(),
            signatures: Default::default(),
        };
        let bytes = to_bytes::<CreateMultisig>(&copy)?;
        let mut hasher = Blake2b256::new();
        hasher.update(from);
        hasher.update(seq.to_be_bytes());
        hasher.update(data_index.to_be_bytes());
        hasher.update(bytes);
        Ok(hasher.finalize().into())
    }
}

impl TxData for CreateMultisig {
    fn process_impl(
        &self,
        tx: Transaction,
        hash: Hash,
        data_index: u32,
        coin_tx: &mut (impl CoinTx + ?Sized),
    ) -> Result<()> {
        if self.n as usize > self.deposits.len() {
            return Err(Error::Invalid("Invalid n".to_owned()));
        }
        if self.deposits.len() > 20 {
            return Err(Error::Invalid("Too many deposits".to_owned()));
        }
        if self.signatures.len() > self.deposits.len() {
            return Err(Error::Invalid("Too many signatures".to_owned()));
        }
        match Amount::checked_sum(self.deposits.iter().copied().map(Dep::amount)) {
            Some(sum) => {
                if sum == Amount::ZERO {
                    return Err(Error::Invalid("Invalid total amount".to_owned()));
                }
            }
            None => return Err(Error::Invalid("Invalid total amount".to_owned())),
        };

        let multisig_hash = self.hash(tx.from(), tx.seq(), data_index)?;
        for (index, deposit) in self.deposits.iter().enumerate() {
            if deposit.amount() != Amount::ZERO {
                let sig = self
                    .signatures
                    .iter()
                    .find(|sig| sig.index == index as u8)
                    .ok_or(Error::Invalid("Unsigned deposit".to_owned()))?;
                verify(sig.signature(), multisig_hash, deposit.from())?;
                let mut deposit_account = coin_tx.get_account(deposit.from())?;
                deposit_account.credit(deposit.amount())?;
                coin_tx.set_account(deposit.from(), deposit_account);
            }
        }

        let id = id(hash, data_index);
        let multisig = Multisig::new(
            self.n,
            self.deposits
                .iter()
                .map(|dep| Deposit::new(dep.from(), dep.amount()))
                .collect(),
        );
        coin_tx.add_multisig(id, multisig);
        Ok(())
    }
}

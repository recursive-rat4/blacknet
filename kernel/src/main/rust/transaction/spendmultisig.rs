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
use crate::ed25519::{PublicKey, verify};
use crate::error::{Error, Result};
use crate::multisig::Multisig;
use crate::transaction::{CoinTx, MultiSignatureLockContractId, Sig, Transaction, TxData};
use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use blacknet_serialization::format::to_bytes;
use digest::Digest;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SpendMultisig {
    id: MultiSignatureLockContractId,
    amounts: Box<[Amount]>,
    signatures: Box<[Sig]>,
}

impl SpendMultisig {
    pub const fn new(
        id: MultiSignatureLockContractId,
        amounts: Box<[Amount]>,
        signatures: Box<[Sig]>,
    ) -> Self {
        Self {
            id,
            amounts,
            signatures,
        }
    }

    pub const fn id(&self) -> MultiSignatureLockContractId {
        self.id
    }

    pub const fn amounts(&self) -> &[Amount] {
        &self.amounts
    }

    pub const fn signatures(&self) -> &[Sig] {
        &self.signatures
    }

    fn verify_signatures(&self, multisig: &Multisig, sender: PublicKey) -> Result<()> {
        let multisig_hash = self.hash()?;
        let mut unsigned = BTreeMap::<u8, PublicKey>::new();
        for (i, deposit) in multisig.deposits().iter().enumerate() {
            unsigned.insert(i as u8, deposit.from());
        }

        for sig in self.signatures.iter() {
            let public_key = unsigned.remove(&sig.index()).ok_or(Error::Invalid(
                "Invalid or twice signed multisig".to_owned(),
            ))?;
            verify(sig.signature(), multisig_hash, public_key)?;
        }

        if unsigned.values().any(|&i| i == sender) {
            Ok(())
        } else {
            Err(Error::Invalid("Invalid sender".to_owned()))
        }
    }

    fn hash(&self) -> Result<Hash> {
        let copy = Self {
            id: self.id,
            amounts: self.amounts.clone(),
            signatures: Default::default(),
        };
        let bytes = to_bytes::<SpendMultisig>(&copy)?;
        Ok(Blake2b256::digest(bytes).into())
    }
}

impl TxData for SpendMultisig {
    fn process_impl(
        &self,
        tx: Transaction,
        _hash: Hash,
        _data_index: u32,
        coin_tx: &mut (impl CoinTx + ?Sized),
    ) -> Result<()> {
        let multisig = coin_tx.get_multisig(self.id)?;
        if self.amounts.len() != multisig.deposits().len() {
            return Err(Error::Invalid("Invalid number of amounts".to_owned()));
        }
        match Amount::checked_sum(self.amounts.iter().copied()) {
            Some(sum) => {
                if sum != multisig.amount() {
                    return Err(Error::Invalid("Invalid total amount".to_owned()));
                }
            }
            None => return Err(Error::Invalid("Invalid total amount".to_owned())),
        };
        if self.signatures.len() + 1 < multisig.n().into() {
            return Err(Error::Invalid("Invalid number of signatures".to_owned()));
        }
        self.verify_signatures(&multisig, tx.from())?;

        let height = coin_tx.height();

        for index in 0..multisig.deposits().len() {
            if self.amounts[index] != Amount::ZERO {
                let public_key = multisig.deposits()[index].from();
                let mut to_account = coin_tx.get_or_create(public_key);
                to_account.debit(height, self.amounts[index]);
                coin_tx.set_account(public_key, to_account);
            }
        }

        coin_tx.remove_multisig(self.id);
        Ok(())
    }
}

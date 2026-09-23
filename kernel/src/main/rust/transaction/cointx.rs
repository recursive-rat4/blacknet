/*
 * Copyright (c) 2018-2026 Pavel Vasin
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
    account::Account,
    amount::Amount,
    blake2b::Hash256,
    ed25519::PublicKey,
    error::{Error, Result},
    htlc::HTLC,
    multisig::Multisig,
    transaction::*,
};
use blacknet_serialization::from_bytes;
use blacknet_time::Seconds;
use serde::Deserialize;

pub trait CoinTx: Sized {
    fn add_supply(&mut self, amount: Amount);
    fn sub_supply(&mut self, amount: Amount);
    fn check_anchor(&self, hash: Hash256) -> Result<()>;
    fn block_hash(&self) -> Hash256;
    fn block_time(&self) -> Seconds;
    fn height(&self) -> u32;
    fn get_account(&mut self, key: PublicKey) -> Result<Account>;
    fn get_or_create(&mut self, key: PublicKey) -> Account;
    fn set_account(&mut self, key: PublicKey, state: Account);
    fn add_htlc(&mut self, id: HashTimeLockContractId, htlc: HTLC);
    fn get_htlc(&mut self, id: HashTimeLockContractId) -> Result<HTLC>;
    fn remove_htlc(&mut self, id: HashTimeLockContractId);
    fn add_multisig(&mut self, id: MultiSignatureLockContractId, multisig: Multisig);
    fn get_multisig(&mut self, id: MultiSignatureLockContractId) -> Result<Multisig>;
    fn remove_multisig(&mut self, id: MultiSignatureLockContractId);

    fn process_transaction_impl(&mut self, tx: &Transaction, hash: Hash256) -> Result<()> {
        tx.verify_signature(hash)?;
        self.check_anchor(tx.anchor())?;
        match tx.kind() {
            TxKind::Transfer => self.process_tx_data::<Transfer>(tx, hash),
            TxKind::Burn => self.process_tx_data::<Burn>(tx, hash),
            TxKind::Lease => self.process_tx_data::<Lease>(tx, hash),
            TxKind::CancelLease => self.process_tx_data::<CancelLease>(tx, hash),
            TxKind::Blob => self.process_tx_data::<Blob>(tx, hash),
            TxKind::CreateHTLC => self.process_tx_data::<CreateHTLC>(tx, hash),
            TxKind::RefundHTLC => self.process_tx_data::<RefundHTLC>(tx, hash),
            TxKind::CreateMultisig => self.process_tx_data::<CreateMultisig>(tx, hash),
            TxKind::SpendMultisig => self.process_tx_data::<SpendMultisig>(tx, hash),
            TxKind::WithdrawFromLease => self.process_tx_data::<WithdrawFromLease>(tx, hash),
            TxKind::ClaimHTLC => self.process_tx_data::<ClaimHTLC>(tx, hash),
            TxKind::Batch => self.process_tx_data::<Batch>(tx, hash),
            TxKind::Generated => Err(Error::invalid("Generated as individual tx")),
        }
    }

    fn process_tx_data<T: TxData + for<'de> Deserialize<'de>>(
        &mut self,
        tx: &Transaction,
        hash: Hash256,
    ) -> Result<()> {
        let data = from_bytes::<T>(tx.data_bytes(), false)?;
        data.process(tx, hash, self)
    }
}

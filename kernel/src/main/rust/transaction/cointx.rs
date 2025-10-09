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

use crate::account::Account;
use crate::amount::Amount;
use crate::blake2b::Hash;
use crate::ed25519::PublicKey;
use crate::error::Result;
use crate::htlc::HTLC;
use crate::multisig::Multisig;
use crate::transaction::{HashTimeLockContractId, MultiSignatureLockContractId};
use blacknet_time::Seconds;

pub trait CoinTx {
    fn add_supply(&self, amount: Amount);
    fn sub_supply(&self, amount: Amount);
    fn check_anchor(hash: Hash) -> bool;
    fn block_hash(&self) -> Hash;
    fn block_time(&self) -> Seconds;
    fn height(&self) -> u32;
    fn get_account(&self, key: PublicKey) -> Result<Account>;
    fn get_or_create(&self, key: PublicKey) -> Account;
    fn set_account(&self, key: PublicKey, state: Account);
    fn add_htlc(&self, id: HashTimeLockContractId, htlc: HTLC);
    fn get_htlc(&self, id: HashTimeLockContractId) -> Result<HTLC>;
    fn remove_htlc(&self, id: HashTimeLockContractId);
    fn add_multisig(&self, id: MultiSignatureLockContractId, multisig: Multisig);
    fn get_multisig(&self, id: MultiSignatureLockContractId) -> Result<Multisig>;
    fn remove_multisig(&self, id: MultiSignatureLockContractId);
}

/*
 * Copyright (c) 2025-2026 Pavel Vasin
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

use blacknet_compat::{Mode, assert_err, assert_ok};
use blacknet_kernel::account::Lease;
use blacknet_kernel::amount::Amount;
use blacknet_kernel::blake2b::Hash;
use blacknet_kernel::ed25519::PublicKey;
use blacknet_kernel::transaction::{HashTimeLockContractId, MultiSignatureLockContractId};
use blacknet_wallet::wallet::Wallet;
use rusqlite::Connection;

#[test]
fn ephemeral() {
    let mode = Mode::regtest();
    let public_key = PublicKey::default();
    let wallet = Wallet::ephemeral(public_key, &mode).unwrap();
    assert_eq!(wallet.public_key().unwrap(), public_key);
    assert_eq!(wallet.sequence().unwrap(), 0);
}

#[test]
fn magic() {
    let mode = Mode::regtest();
    let connection = Connection::open_in_memory().unwrap();
    assert_err!(Wallet::attach(connection, &mode));
}

#[test]
fn htlc() {
    let mode = Mode::regtest();
    let wallet = Wallet::ephemeral(PublicKey::default(), &mode).unwrap();
    let htlc_id = HashTimeLockContractId::default();
    assert_ok!(wallet.put_htlc(htlc_id));
    assert!(wallet.has_htlc(htlc_id).unwrap());
    assert_ok!(wallet.remove_htlc(htlc_id));
    assert!(!wallet.has_htlc(htlc_id).unwrap());
}

#[test]
fn multisig() {
    let mode = Mode::regtest();
    let wallet = Wallet::ephemeral(PublicKey::default(), &mode).unwrap();
    let multisig_id = MultiSignatureLockContractId::default();
    assert_ok!(wallet.put_multisig(multisig_id));
    assert!(wallet.has_multisig(multisig_id).unwrap());
    assert_ok!(wallet.remove_multisig(multisig_id));
    assert!(!wallet.has_multisig(multisig_id).unwrap());
}

#[test]
fn out_lease() {
    let mode = Mode::regtest();
    let wallet = Wallet::ephemeral(PublicKey::default(), &mode).unwrap();
    let lease1 = Lease::new(PublicKey::default(), 1, Amount::new(123));
    let lease2 = Lease::new(PublicKey::default(), 2, Amount::new(123));
    let lease3 = Lease::new(PublicKey::default(), 2, Amount::new(100));
    assert_ok!(wallet.put_out_lease(lease1));
    assert_ok!(wallet.set_out_lease_height(lease1, lease2.height()));
    assert_ok!(wallet.withdraw_from_out_lease(lease2, Amount::new(23)));
    assert_ok!(wallet.remove_out_lease(lease3));
}

#[test]
fn transaction() {
    let mode = Mode::regtest();
    let wallet = Wallet::ephemeral(PublicKey::default(), &mode).unwrap();
    let tx_id = Hash::ZERO;
    let tx_bytes = [10, 11, 12, 13];
    assert_ok!(wallet.put_transaction(tx_id, &tx_bytes));
    let bytes = wallet.get_transaction(tx_id).unwrap();
    assert_eq!(bytes, tx_bytes.into());
}

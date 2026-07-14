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

use blacknet_compat::Mode;
use blacknet_kernel::account::Lease;
use blacknet_kernel::amount::Amount;
use blacknet_kernel::blake2b::Hash;
use blacknet_kernel::ed25519::PublicKey;
use blacknet_kernel::transaction::{HashTimeLockContractId, MultiSignatureLockContractId};
use blacknet_wallet::wallet::Wallet;
use blacknet_wallet::walletdb::Error;
use core::assert_matches;
use rusqlite::Connection;

#[test]
fn ephemeral() {
    let mode = Mode::regtest();
    let public_key = PublicKey::default();
    let wallet = Wallet::ephemeral(public_key, &mode).unwrap();
    assert_matches!(wallet.created_at(), Ok(_));
    assert_eq!(wallet.public_key().unwrap(), public_key);
    assert_matches!(wallet.sequence(), Ok(0));
}

#[test]
fn magic() {
    let mode = Mode::regtest();
    let connection = Connection::open_in_memory().unwrap();
    assert_matches!(Wallet::attach(connection, &mode), Err(Error::WrongMagic(_)));
}

#[test]
fn htlc() {
    let mode = Mode::regtest();
    let wallet = Wallet::ephemeral(PublicKey::default(), &mode).unwrap();
    let htlc_id = HashTimeLockContractId::default();
    assert_matches!(wallet.put_htlc(htlc_id), Ok(()));
    assert_matches!(wallet.has_htlc(htlc_id), Ok(true));
    assert_matches!(wallet.remove_htlc(htlc_id), Ok(()));
    assert_matches!(wallet.has_htlc(htlc_id), Ok(false));
}

#[test]
fn multisig() {
    let mode = Mode::regtest();
    let wallet = Wallet::ephemeral(PublicKey::default(), &mode).unwrap();
    let multisig_id = MultiSignatureLockContractId::default();
    assert_matches!(wallet.put_multisig(multisig_id), Ok(()));
    assert_matches!(wallet.has_multisig(multisig_id), Ok(true));
    assert_matches!(wallet.remove_multisig(multisig_id), Ok(()));
    assert_matches!(wallet.has_multisig(multisig_id), Ok(false));
}

#[test]
fn out_lease() {
    let mode = Mode::regtest();
    let wallet = Wallet::ephemeral(PublicKey::default(), &mode).unwrap();
    let lease1 = Lease::new(PublicKey::default(), 1, Amount::new(123));
    let lease2 = Lease::new(PublicKey::default(), 2, Amount::new(123));
    let lease3 = Lease::new(PublicKey::default(), 2, Amount::new(100));
    assert_matches!(wallet.put_out_lease(lease1), Ok(()));
    assert_matches!(wallet.set_out_lease_height(lease1, lease2.height()), Ok(()));
    assert_matches!(
        wallet.withdraw_from_out_lease(lease2, Amount::new(23)),
        Ok(())
    );
    assert_matches!(wallet.remove_out_lease(lease3), Ok(()));
}

#[test]
fn transaction() {
    let mode = Mode::regtest();
    let wallet = Wallet::ephemeral(PublicKey::default(), &mode).unwrap();
    let tx_id = Hash::ZERO;
    let tx_bytes: [u8; 4] = [10, 11, 12, 13];
    assert_matches!(wallet.put_transaction(tx_id, &tx_bytes), Ok(()));
    let bytes = wallet.get_transaction(tx_id).unwrap();
    assert_eq!(tx_bytes, *bytes);
}

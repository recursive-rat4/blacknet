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
use blacknet_kernel::{
    account::Lease,
    amount::Amount,
    blake2b::Hash256,
    ed25519::{PublicKey, SecretKey},
    transaction::{HashTimeLockContractId, MultiSignatureLockContractId},
};
use blacknet_network::wallet::{DeriveAccountError, Error, OpenError, Wallet};
use blacknet_time::Seconds;
use core::{assert_matches, str::FromStr};
use rusqlite::Connection;

#[test]
fn magic() {
    let mode = Mode::regtest();
    let connection = Connection::open_in_memory().unwrap();

    assert_matches!(Wallet::attach(connection, &mode), Err(OpenError::Magic(_)));
}

#[test]
fn ephemeral() {
    let mode = Mode::regtest();
    let wallet = Wallet::ephemeral(&mode).unwrap();
    let created_at = Seconds::new(123);

    assert_matches!(wallet.created_at(), Ok(_));
    assert_matches!(wallet.set_created_at(created_at), Ok(()));
    assert_matches!(wallet.created_at(), Ok(x) if x == created_at);
    assert_matches!(wallet.is_staking(), Ok(true));
    assert_matches!(wallet.sequence(), Ok(0));
}

#[test]
fn keys() {
    let mode = Mode::regtest();
    let wallet = Wallet::ephemeral(&mode).unwrap();
    let mnemonic = "胡 允 空 桥 料 状 纱 角 钠 灌 绝 件";
    let public_key =
        PublicKey::from_str("A65AEF3E4128031285BF0367832C38AD1366A1E8D5E395BCDC7A17C3B28BAB1D")
            .unwrap();
    let secret_key =
        SecretKey::try_from("168FFB9152BE8C88F1613B54BCCEA40E5F73DBFB845CBA6CA4E5C13D8FD0D68F")
            .unwrap();

    assert_matches!(wallet.public_key(), Err(Error::QueryReturnedNoRows));
    assert_matches!(wallet.secret_key(), Err(Error::QueryReturnedNoRows));
    assert_matches!(wallet.set_mnemonic(mnemonic.into()), Ok(()));
    assert_matches!(
        wallet.set_mnemonic(mnemonic.into()),
        Err(Error::SqliteFailure(..))
    );
    assert_matches!(wallet.derive_account(), Ok(()));
    assert_matches!(wallet.derive_account(), Err(DeriveAccountError::Sqlite(..)));
    assert_matches!(wallet.public_key(), Ok(pk) if pk == public_key);
    assert_matches!(wallet.secret_key(), Ok(sk) if sk.as_ref() == secret_key.as_ref());
}

#[test]
fn htlc() {
    let mode = Mode::regtest();
    let wallet = Wallet::ephemeral(&mode).unwrap();
    let htlc_id = HashTimeLockContractId::default();

    assert_matches!(wallet.put_htlc(htlc_id), Ok(()));
    assert_matches!(wallet.has_htlc(htlc_id), Ok(true));
    assert_matches!(wallet.remove_htlc(htlc_id), Ok(()));
    assert_matches!(wallet.has_htlc(htlc_id), Ok(false));
}

#[test]
fn multisig() {
    let mode = Mode::regtest();
    let wallet = Wallet::ephemeral(&mode).unwrap();
    let multisig_id = MultiSignatureLockContractId::default();

    assert_matches!(wallet.put_multisig(multisig_id), Ok(()));
    assert_matches!(wallet.has_multisig(multisig_id), Ok(true));
    assert_matches!(wallet.remove_multisig(multisig_id), Ok(()));
    assert_matches!(wallet.has_multisig(multisig_id), Ok(false));
}

#[test]
fn out_lease() {
    let mode = Mode::regtest();
    let wallet = Wallet::ephemeral(&mode).unwrap();
    let lease1 = Lease::new(PublicKey::default(), 1, Amount::new(123));
    let lease2 = Lease::new(PublicKey::default(), 2, Amount::new(123));
    let lease3 = Lease::new(PublicKey::default(), 2, Amount::new(100));
    let leases = vec![lease2.clone(), lease3.clone()];

    assert_matches!(wallet.put_out_lease(lease1), Ok(()));
    assert_matches!(wallet.put_out_lease(lease2), Ok(()));
    assert_matches!(wallet.put_out_lease(lease3), Ok(()));
    assert_matches!(wallet.set_out_lease_height(lease1, lease2.height()), Ok(()));
    assert_matches!(
        wallet.withdraw_from_out_lease(lease2, Amount::new(23)),
        Ok(())
    );
    assert_matches!(wallet.remove_out_lease(lease3), Ok(()));
    assert_matches!(wallet.get_out_leases(), Ok(x) if x == leases);
}

#[test]
fn transaction() {
    let mode = Mode::regtest();
    let wallet = Wallet::ephemeral(&mode).unwrap();
    let tx_id = Hash256::ZERO;
    let tx_bytes: [u8; 4] = [10, 11, 12, 13];

    assert_matches!(wallet.count_transactions(), Ok(0));
    assert_matches!(wallet.put_transaction(tx_id, &tx_bytes), Ok(()));
    assert_matches!(
        wallet.put_transaction(tx_id, &tx_bytes),
        Err(Error::SqliteFailure(..))
    );
    assert_matches!(wallet.count_transactions(), Ok(1));
    let bytes = wallet.get_transaction(tx_id).unwrap();
    assert_eq!(tx_bytes, *bytes);
}

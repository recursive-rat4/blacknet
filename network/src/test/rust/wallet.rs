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
use blacknet_network::wallet::{
    DeriveAccountError, Error, OpenError, TransactionData, TransactionOutputData, Wallet,
};
use blacknet_time::Seconds;
use core::{assert_matches, str::FromStr};
use rusqlite::{Connection, ErrorCode};

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
    assert_matches!(wallet.set_staking(false), Ok(()));
    assert_matches!(wallet.is_staking(), Ok(false));
    assert_matches!(wallet.sequence(), Ok(0));
}

#[test]
fn keys() {
    let mode = Mode::regtest();
    let mnemonic = "胡 允 空 桥 料 状 纱 角 钠 灌 绝 件";
    let public_key =
        PublicKey::from_str("A65AEF3E4128031285BF0367832C38AD1366A1E8D5E395BCDC7A17C3B28BAB1D")
            .unwrap();
    let secret_key =
        SecretKey::try_from("168FFB9152BE8C88F1613B54BCCEA40E5F73DBFB845CBA6CA4E5C13D8FD0D68F")
            .unwrap();

    let wallet = Wallet::ephemeral(&mode).unwrap();
    assert_matches!(wallet.public_key(), Err(Error::QueryReturnedNoRows));
    assert_matches!(wallet.secret_key(), Err(Error::QueryReturnedNoRows));
    assert_matches!(wallet.put_watch(public_key), Ok(()));
    assert_matches!(
        wallet.put_watch(public_key),
        Err(Error::SqliteFailure(err, _)) if err.code == ErrorCode::ConstraintViolation
    );
    assert_matches!(wallet.set_mnemonic(mnemonic.into()), Ok(()));
    assert_matches!(
        wallet.set_mnemonic(mnemonic.into()),
        Err(Error::SqliteFailure(err, _)) if err.code == ErrorCode::ConstraintViolation
    );

    let wallet = Wallet::ephemeral(&mode).unwrap();
    assert_matches!(wallet.set_mnemonic(mnemonic.into()), Ok(()));
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
    let tx_time = Seconds::new(444);
    let tx_height1 = None;
    let tx_outputs = vec![TransactionOutputData::new(2, 3)];
    let tx_data1 = TransactionData::new(tx_outputs.clone(), tx_time, tx_height1);
    let tx_bytes: [u8; 4] = [10, 11, 12, 13];
    let tx_height2 = Some(100);
    let tx_data2 = TransactionData::new(tx_outputs, tx_time, tx_height2);

    assert_matches!(wallet.count_transactions(), Ok(0));
    assert_matches!(
        wallet.upsert_transaction(tx_id, &tx_data1, &tx_bytes),
        Ok(())
    );
    assert_matches!(wallet.get_transaction_data(tx_id), Ok(x) if x == tx_data1);
    assert_matches!(
        wallet.upsert_transaction(tx_id, &tx_data2, &tx_bytes),
        Ok(())
    );
    assert_matches!(wallet.count_transactions(), Ok(1));
    assert_matches!(wallet.get_transaction_data(tx_id), Ok(x) if x == tx_data2);
    assert_matches!(wallet.get_transactions_data(), Ok(x) if *x == [(tx_id, tx_data2)]);
    assert_matches!(wallet.get_transaction_bytes(tx_id), Ok(x) if *x == tx_bytes);
}

#[test]
fn page() {
    let mode = Mode::regtest();
    let wallet = Wallet::ephemeral(&mode).unwrap();
    let tx1_id = Hash256::ZERO;
    let tx1_time = Seconds::new(443);
    let tx1_height = None;
    let tx1_outputs = vec![TransactionOutputData::new(2, 3)];
    let tx1_data = TransactionData::new(tx1_outputs, tx1_time, tx1_height);
    let tx1_bytes: [u8; 4] = [10, 11, 12, 13];
    let tx2_id = Hash256::from([1; 32]);
    let tx2_time = Seconds::new(444);
    let tx2_height = None;
    let tx2_outputs = vec![TransactionOutputData::new(3, 4)];
    let tx2_data = TransactionData::new(tx2_outputs, tx2_time, tx2_height);
    let tx2_bytes: [u8; 6] = [20, 21, 22, 23, 24, 25];

    assert_matches!(
        wallet.upsert_transaction(tx1_id, &tx1_data, &tx1_bytes),
        Ok(())
    );
    assert_matches!(
        wallet.upsert_transaction(tx2_id, &tx2_data, &tx2_bytes),
        Ok(())
    );
    assert_matches!(
        wallet.page_transactions(1, 1, None),
        Ok(x) if x == [(tx1_id, tx1_data.clone(), tx1_bytes.into())]
    );
    assert_matches!(
        wallet.page_transactions(1, 0, None),
        Ok(x) if x == [(tx2_id, tx2_data, tx2_bytes.into())]
    );
    assert_matches!(
        wallet.page_transactions(10, 0, Some(3)),
        Ok(x) if x == [(tx1_id, tx1_data, tx1_bytes.into())]
    );
}

#[test]
fn since() {
    let mode = Mode::regtest();
    let wallet = Wallet::ephemeral(&mode).unwrap();
    let tx1_id = Hash256::ZERO;
    let tx1_time = Seconds::new(443);
    let tx1_height = Some(100);
    let tx1_outputs = vec![TransactionOutputData::new(2, 3)];
    let tx1_data = TransactionData::new(tx1_outputs, tx1_time, tx1_height);
    let tx1_bytes: [u8; 4] = [10, 11, 12, 13];
    let tx2_id = Hash256::from([1; 32]);
    let tx2_time = Seconds::new(444);
    let tx2_height = None;
    let tx2_outputs = vec![TransactionOutputData::new(3, 4)];
    let tx2_data = TransactionData::new(tx2_outputs, tx2_time, tx2_height);
    let tx2_bytes: [u8; 6] = [20, 21, 22, 23, 24, 25];
    let tx3_id = Hash256::from([2; 32]);
    let tx3_time = Seconds::new(444);
    let tx3_height = Some(200);
    let tx3_outputs = vec![TransactionOutputData::new(0, 0)];
    let tx3_data = TransactionData::new(tx3_outputs, tx3_time, tx3_height);
    let tx3_bytes: [u8; 4] = [30, 31, 32, 33];

    assert_matches!(
        wallet.upsert_transaction(tx1_id, &tx1_data, &tx1_bytes),
        Ok(())
    );
    assert_matches!(
        wallet.upsert_transaction(tx2_id, &tx2_data, &tx2_bytes),
        Ok(())
    );
    assert_matches!(
        wallet.upsert_transaction(tx3_id, &tx3_data, &tx3_bytes),
        Ok(())
    );
    assert_matches!(
        wallet.transactions_since(100),
        Ok(x) if x == [(tx1_id, tx1_data.clone(), tx1_bytes.into())]
    );
    assert_matches!(
        wallet.transactions_since(200),
        Ok(x) if x == [(tx1_id, tx1_data, tx1_bytes.into()), (tx3_id, tx3_data, tx3_bytes.into())]
    );
}

#[test]
fn rollback() {
    let mode = Mode::regtest();
    let wallet = Wallet::ephemeral(&mode).unwrap();
    let tx1_id = Hash256::ZERO;
    let tx1_time = Seconds::new(443);
    let tx1_height = Some(100);
    let tx1_outputs = vec![TransactionOutputData::new(2, 3)];
    let tx1_data = TransactionData::new(tx1_outputs, tx1_time, tx1_height);
    let tx1_bytes: [u8; 4] = [10, 11, 12, 13];
    let tx2_id = Hash256::from([1; 32]);
    let tx2_time = Seconds::new(444);
    let tx2_height = Some(100);
    let tx2_outputs = vec![TransactionOutputData::new(3, 4)];
    let tx2_data = TransactionData::new(tx2_outputs.clone(), tx2_time, tx2_height);
    let tx2_bytes: [u8; 6] = [20, 21, 22, 23, 24, 25];
    let tx3_id = Hash256::from([2; 32]);
    let tx2_data2 = TransactionData::new(tx2_outputs, tx2_time, None);

    assert_matches!(
        wallet.upsert_transaction(tx1_id, &tx1_data, &tx1_bytes),
        Ok(())
    );
    assert_matches!(
        wallet.upsert_transaction(tx2_id, &tx2_data, &tx2_bytes),
        Ok(())
    );
    assert_matches!(wallet.rollback(tx2_id), Ok(()));
    assert_matches!(wallet.rollback(tx3_id), Ok(()));
    assert_matches!(
        wallet.get_transactions_data(),
        Ok(x) if *x == [(tx1_id, tx1_data), (tx2_id, tx2_data2)]
    );
}

/*
 * Copyright (c) 2025 Pavel Vasin
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
use blacknet_kernel::blake2b::Hash;
use blacknet_wallet::wallet::Wallet;
use rusqlite::Connection;

#[test]
fn ephemeral() {
    let mode = Mode::regtest();
    let wallet = Wallet::ephemeral(&mode).unwrap();
    let tx_id = Hash::ZERO;
    let tx_bytes = [10, 11, 12, 13];
    assert_ok!(wallet.put_transaction(tx_id, &tx_bytes));
    let bytes = wallet.get_transaction(tx_id).unwrap();
    assert_eq!(bytes, tx_bytes.into());
}

#[test]
fn magic() {
    let mode = Mode::regtest();
    let connection = Connection::open_in_memory().unwrap();
    assert_err!(Wallet::attach(connection, &mode));
}

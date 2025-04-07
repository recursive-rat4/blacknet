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

#include <boost/test/unit_test.hpp>

#include "byte.h"
#include "sqlite.h"
#include "util.h"
#include "wallet.h"

namespace byte = blacknet::crypto::byte;
namespace sqlite = blacknet::wallet::sqlite;
using blacknet::wallet::Exception;
using blacknet::wallet::Wallet;

BOOST_AUTO_TEST_SUITE(Wallets)

BOOST_AUTO_TEST_CASE(ephemeral) {
    auto wallet = Wallet::ephemeral();

    auto txId = byte::arrayS<2>({ 1, 1 });
    auto txBytes = byte::arrayS<4>({ 10, 11, 12, 13 });
    wallet.transaction(txId, txBytes);
    auto bytes = wallet.transaction(txId);
    BOOST_CHECK_EQUAL_COLLECTIONS(txBytes.begin(), txBytes.end(), bytes.begin(), bytes.end());
}

BOOST_AUTO_TEST_CASE(magic) {
    BOOST_CHECK_THROW(Wallet::attach(sqlite::Connection::memory()), Exception);
}

BOOST_AUTO_TEST_SUITE_END()

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

#include "sqlite.h"

namespace sqlite = blacknet::wallet::sqlite;

BOOST_AUTO_TEST_SUITE(SQLites)

BOOST_AUTO_TEST_CASE(Connection) {
    auto connection = sqlite::Connection::memory();
    BOOST_TEST_REQUIRE(connection.isConnected());
    connection.execute("CREATE TABLE kv(key INTEGER PRIMARY KEY, value TEXT);");
    connection.execute("INSERT INTO kv VALUES(0, 'zero');");
    connection.execute("INSERT INTO kv VALUES(4, 'four');");
    std::size_t count = 0;
    {
        auto rows = connection.evaluate("SELECT value FROM kv WHERE key = 0;");
        for (auto&& row : rows) {
            BOOST_TEST_REQUIRE(row.columns() == 1);
            BOOST_TEST(row.text(0) == "zero");
            ++count;
        }
    } {
        auto rows = connection.evaluate("SELECT value FROM kv WHERE key = 2;");
        for (auto&& row : rows) {
            BOOST_TEST_REQUIRE(row.columns() == 1);
            BOOST_TEST(row.text(0) == "two");
            ++count;
        }
    } {
        auto rows = connection.evaluate("SELECT value FROM kv WHERE key = 4;");
        for (auto&& row : rows) {
            BOOST_TEST_REQUIRE(row.columns() == 1);
            BOOST_TEST(row.text(0) == "four");
            ++count;
        }
    }
    BOOST_TEST(count == 2);
}

BOOST_AUTO_TEST_CASE(Statement) {
    auto connection = sqlite::Connection::memory();
    BOOST_TEST_REQUIRE(connection.isConnected());
    connection.prepare("CREATE TABLE kv(key INTEGER PRIMARY KEY, value TEXT);").execute();
    connection.prepare("INSERT INTO kv VALUES(0, 'zero');").execute();
    connection.prepare("INSERT INTO kv VALUES(4, 'four');").execute();
    std::size_t count = 0;
    auto statement = connection.prepare("SELECT value FROM kv WHERE key = ?;");
    BOOST_TEST_REQUIRE(statement.isPrepared());
    {
        auto binder = statement.binder();
        binder.integer(1, 0);
        for (auto&& row : statement.evaluate()) {
            BOOST_TEST_REQUIRE(row.columns() == 1);
            BOOST_TEST(row.text(0) == "zero");
            ++count;
        }
    } {
        auto binder = statement.binder();
        binder.integer(1, 2);
        for (auto&& row : statement.evaluate()) {
            BOOST_TEST_REQUIRE(row.columns() == 1);
            BOOST_TEST(row.text(0) == "two");
            ++count;
        }
    } {
        auto binder = statement.binder();
        binder.integer(1, 4);
        for (auto&& row : statement.evaluate()) {
            BOOST_TEST_REQUIRE(row.columns() == 1);
            BOOST_TEST(row.text(0) == "four");
            ++count;
        }
    }
    BOOST_TEST(count == 2);
}

BOOST_AUTO_TEST_SUITE_END()

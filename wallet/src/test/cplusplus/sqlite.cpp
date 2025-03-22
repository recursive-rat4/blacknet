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

BOOST_AUTO_TEST_SUITE(SQLites)

BOOST_AUTO_TEST_CASE(test) {
    auto connection = sqlite::Connection::memory();
    BOOST_TEST_REQUIRE(connection.isConnected());
    auto statement = connection.prepare("PRAGMA locking_mode;");
    BOOST_TEST_REQUIRE(statement.isPrepared());
    statement.evaluate([](auto& evaluator) {
        BOOST_TEST_REQUIRE(evaluator.columns() == 1);
        BOOST_TEST(evaluator.text(0) == "exclusive");
    });
}

BOOST_AUTO_TEST_SUITE_END()

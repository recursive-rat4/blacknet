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

#include "i2psam.h"

using namespace blacknet::network::i2p;

BOOST_AUTO_TEST_SUITE(I2PSAMs)

BOOST_AUTO_TEST_CASE(values) {
    const Answer newlined{"HELLO REPLY RESULT=OK VERSION=3.3\n"};
    BOOST_TEST(newlined.get("VERSION").value() == "3.3");

    const Answer quoted{"HELLO REPLY RESULT=I2P_ERROR MESSAGE=\"Must start with HELLO VERSION\"\n"};
    BOOST_TEST(quoted.get("MESSAGE").value() == "Must start with HELLO VERSION");
}

BOOST_AUTO_TEST_CASE(oks) {
    const Answer yay{"HELLO REPLY RESULT=OK VERSION=3.3\n"};
    yay.ok();

    const Answer nay{"HELLO REPLY RESULT=I2P_ERROR MESSAGE=\"Must start with HELLO VERSION\"\n"};
    BOOST_CHECK_THROW(nay.ok(), Exception);
}

BOOST_AUTO_TEST_SUITE_END()

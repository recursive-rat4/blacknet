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

#include "z2.h"

using namespace blacknet::crypto;

BOOST_AUTO_TEST_SUITE(Z2s)

BOOST_AUTO_TEST_CASE(Representative) {
    Z2 a(-1);
    BOOST_TEST(1 == a.canonical());
    BOOST_TEST(1 == a.balanced());
    BOOST_TEST(1 == a.absolute());
}

BOOST_AUTO_TEST_CASE(Add) {
    BOOST_TEST(Z2(0) == Z2(0) + Z2(0));
    BOOST_TEST(Z2(1) == Z2(0) + Z2(1));
    BOOST_TEST(Z2(1) == Z2(1) + Z2(0));
    BOOST_TEST(Z2(0) == Z2(1) + Z2(1));
}

BOOST_AUTO_TEST_CASE(Mul) {
    BOOST_TEST(Z2(0) == Z2(0) * Z2(0));
    BOOST_TEST(Z2(0) == Z2(0) * Z2(1));
    BOOST_TEST(Z2(0) == Z2(1) * Z2(0));
    BOOST_TEST(Z2(1) == Z2(1) * Z2(1));
}

BOOST_AUTO_TEST_CASE(Sqr) {
    BOOST_TEST(Z2(0) == Z2(0).square());
    BOOST_TEST(Z2(1) == Z2(1).square());
}

BOOST_AUTO_TEST_CASE(Sub) {
    BOOST_TEST(Z2(0) == Z2(0) - Z2(0));
    BOOST_TEST(Z2(1) == Z2(0) - Z2(1));
    BOOST_TEST(Z2(1) == Z2(1) - Z2(0));
    BOOST_TEST(Z2(0) == Z2(1) - Z2(1));
}

BOOST_AUTO_TEST_CASE(Inv) {
    BOOST_TEST(Z2(1) == Z2(1).invert().value());
    BOOST_TEST(!Z2(0).invert());
}

BOOST_AUTO_TEST_CASE(InfinityNorm) {
    BOOST_TEST(!Z2(0).checkInfinityNorm(0));
    BOOST_TEST(Z2(0).checkInfinityNorm(1));
    BOOST_TEST(!Z2(1).checkInfinityNorm(0));
    BOOST_TEST(!Z2(1).checkInfinityNorm(1));
}

BOOST_AUTO_TEST_SUITE_END()

/*
 * Copyright (c) 2024 Pavel Vasin
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

#include "fermat.h"

using Zq = FermatRing;

BOOST_AUTO_TEST_SUITE(Fermats)

BOOST_AUTO_TEST_CASE(ZqAdd) {
    Zq a(981);
    Zq b(-1516);
    Zq c(-535);
    BOOST_TEST(c == a + b);
    BOOST_TEST(c == b + a);
    BOOST_TEST(c == Zq(0) + c);
    BOOST_TEST(c == c + Zq(0));
    BOOST_TEST(Zq(1) == Zq(1) + Zq(0));
    BOOST_TEST(Zq(1) == Zq(0) + Zq(1));
    BOOST_TEST(Zq(0) == Zq(-1) + Zq(1));
}

BOOST_AUTO_TEST_CASE(ZqMul) {
    Zq a(-684);
    Zq b(-133);
    Zq c(25435);
    BOOST_TEST(c == a * b);
    BOOST_TEST(c == b * a);
    BOOST_TEST(Zq(0) == Zq(0) * c);
    BOOST_TEST(Zq(0) == c * Zq(0));
    BOOST_TEST(c == c * Zq(1));
    BOOST_TEST(c == Zq(1) * c);
}

BOOST_AUTO_TEST_CASE(ZqSqr) {
    BOOST_TEST(Zq(1) == Zq(-1).square());
    BOOST_TEST(Zq(0) == Zq(0).square());
    BOOST_TEST(Zq(1) == Zq(1).square());
}

BOOST_AUTO_TEST_CASE(ZqSub) {
    Zq a(-1045);
    Zq b(32750);
    Zq c(31742);
    Zq d(-31742);
    BOOST_TEST(c == a - b);
    BOOST_TEST(d == b - a);
    BOOST_TEST(c == c - Zq(0));
    BOOST_TEST(Zq(0) == Zq(0) - Zq(0));
    BOOST_TEST(Zq(0) == Zq(1) - Zq(1));
}

BOOST_AUTO_TEST_CASE(ZqInv) {
    Zq a(24);
    Zq b(19115);
    Zq c(-25);
    Zq d(-5243);
    BOOST_TEST(a == b.invert().value());
    BOOST_TEST(b == a.invert().value());
    BOOST_TEST(c == d.invert().value());
    BOOST_TEST(d == c.invert().value());
    BOOST_TEST(!Zq(0).invert());
}

BOOST_AUTO_TEST_CASE(ZqInfiniteNorm) {
    Zq a(-30000);
    Zq b(30000);
    int64_t nb = 30000;
    int64_t ng = 30001;
    BOOST_TEST(!a.checkInfinityNorm(nb));
    BOOST_TEST(a.checkInfinityNorm(ng));
    BOOST_TEST(!b.checkInfinityNorm(nb));
    BOOST_TEST(b.checkInfinityNorm(ng));
}

BOOST_AUTO_TEST_SUITE_END()

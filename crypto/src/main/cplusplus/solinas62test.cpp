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

#include "solinas62.h"

using Zq = Solinas62Ring;

BOOST_AUTO_TEST_SUITE(Solinas62)

BOOST_AUTO_TEST_CASE(ZqHex) {
    constexpr Zq a("396a250883366674");
    constexpr Zq b("123c53eebb61ea24");
    Zq c(0x396a250883366674);
    Zq d(0x123c53eebb61ea24);
    BOOST_TEST(c == a);
    BOOST_TEST(d == b);
}

BOOST_AUTO_TEST_CASE(ZqAdd) {
    Zq a(1152921504606846974);
    Zq b(1152921504606846970);
    Zq c(-2305843009213693673);
    BOOST_TEST(c == a + b);
    BOOST_TEST(c == b + a);
    BOOST_TEST(c == Zq(0) + c);
    BOOST_TEST(c == c + Zq(0));
    BOOST_TEST(Zq(1) == Zq(1) + Zq(0));
    BOOST_TEST(Zq(1) == Zq(0) + Zq(1));
}

BOOST_AUTO_TEST_CASE(ZqDbl) {
    Zq a(1785355035602804987);
    Zq b(-1040975947221777643);
    BOOST_TEST(b == a.douple());
    BOOST_TEST(Zq(0) == Zq(0).douple());
}

BOOST_AUTO_TEST_CASE(ZqMul) {
    Zq a(1152102451225612864);
    Zq b(-32);
    Zq c(26209708199489288);
    BOOST_TEST(c == a * b);
    BOOST_TEST(c == b * a);
    BOOST_TEST(Zq(0) == Zq(0) * c);
    BOOST_TEST(Zq(0) == c * Zq(0));
    BOOST_TEST(c == c * Zq(1));
    BOOST_TEST(c == Zq(1) * c);
}

BOOST_AUTO_TEST_CASE(ZqSqr) {
    Zq a(801619525837393032);
    Zq b(1446473281281560723);
    BOOST_TEST(b == a.square());
    BOOST_TEST(Zq(0) == Zq(0).square());
    BOOST_TEST(Zq(1) == Zq(1).square());
}

BOOST_AUTO_TEST_CASE(ZqSub) {
    Zq a(-2048);
    Zq b(65536);
    Zq c(-67584);
    BOOST_TEST(c == a - b);
    BOOST_TEST(c == c - Zq(0));
    BOOST_TEST(Zq(0) == Zq(0) - Zq(0));
    BOOST_TEST(Zq(0) == Zq(1) - Zq(1));
}

BOOST_AUTO_TEST_CASE(ZqInfiniteNorm) {
    Zq a(-677133638855483916);
    Zq b(1140329745848183219);
    int64_t ab = 677133638855483916;
    int64_t ag = 677133638855483917;
    int64_t bb = 1140329745848183219;
    int64_t bg = 1140329745848183220;
    BOOST_TEST(!a.checkInfiniteNorm(ab));
    BOOST_TEST(a.checkInfiniteNorm(ag));
    BOOST_TEST(!b.checkInfiniteNorm(bb));
    BOOST_TEST(b.checkInfiniteNorm(bg));
}

BOOST_AUTO_TEST_SUITE_END()

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

#include "pervushin.h"
#include "pervushinfield.h"

using Zq = PervushinRing;
using Fe2 = PervushinRingDegree2;

BOOST_AUTO_TEST_SUITE(Pervushins)

BOOST_AUTO_TEST_CASE(ZqAdd) {
    Zq a(1152921504606846974);
    Zq b(1152921504606846970);
    Zq c(-7);
    BOOST_TEST(c == a + b);
    BOOST_TEST(c == b + a);
    BOOST_TEST(c == Zq(0) + c);
    BOOST_TEST(c == c + Zq(0));
    BOOST_TEST(Zq(1) == Zq(1) + Zq(0));
    BOOST_TEST(Zq(1) == Zq(0) + Zq(1));
    BOOST_TEST(Zq(0) == Zq(-1) + Zq(1));
}

BOOST_AUTO_TEST_CASE(ZqMul) {
    Zq a(1152102451225612864);
    Zq b(-32);
    Zq c(26209708199491568);
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
    Zq a(-2048);
    Zq b(65536);
    Zq c(-67584);
    Zq d(67584);
    BOOST_TEST(c == a - b);
    BOOST_TEST(d == b - a);
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

BOOST_AUTO_TEST_CASE(Fe2Mul) {
    Fe2 a({-562956929497444169, 136532190776072177});
    Zq b(51280928868087145);
    Fe2 c({-557186355960048698, -800938371403945454});
    Fe2 d({483463506662809566, -624462247079014308});
    BOOST_TEST(c == a * b);
    BOOST_TEST(c == b * a);
    BOOST_TEST(d == a * c);
    BOOST_TEST(d == c * a);
    BOOST_TEST(Fe2::LEFT_ADDITIVE_IDENTITY() == Fe2::LEFT_ADDITIVE_IDENTITY() * c);
    BOOST_TEST(Fe2::LEFT_ADDITIVE_IDENTITY() == c * Fe2::LEFT_ADDITIVE_IDENTITY());
    BOOST_TEST(c == c * Fe2::LEFT_MULTIPLICATIVE_IDENTITY());
    BOOST_TEST(c == Fe2::LEFT_MULTIPLICATIVE_IDENTITY() * c);
}

BOOST_AUTO_TEST_SUITE_END()

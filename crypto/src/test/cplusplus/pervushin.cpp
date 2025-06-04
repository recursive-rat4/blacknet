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
#include "pervushinextension.h"

using namespace blacknet::crypto;

using Zq = PervushinRing;
using Fe2 = PervushinRingDegree2;
using Fe3 = PervushinRingDegree3;
using Fe4 = PervushinRingDegree4;

BOOST_AUTO_TEST_SUITE(Pervushins)

BOOST_AUTO_TEST_CASE(Representative) {
    Zq a(-1);
    BOOST_TEST(2305843009213693950 == a.canonical());
    BOOST_TEST(-1 == a.balanced());
    BOOST_TEST(1 == a.absolute());
}

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

BOOST_AUTO_TEST_CASE(ZqInv) {
    Zq a(24);
    Zq b(-672537544353994069);
    Zq c(-25);
    Zq d(92233720368547758);
    BOOST_TEST(a == b.invert().value());
    BOOST_TEST(b == a.invert().value());
    BOOST_TEST(c == d.invert().value());
    BOOST_TEST(d == c.invert().value());
    BOOST_TEST(!Zq(0).invert());
}

BOOST_AUTO_TEST_CASE(ZqInfinityNorm) {
    Zq a(-677133638855483916);
    Zq b(1140329745848183219);
    int64_t ab = 677133638855483916;
    int64_t ag = 677133638855483917;
    int64_t bb = 1140329745848183219;
    int64_t bg = 1140329745848183220;
    BOOST_TEST(!a.checkInfinityNorm(ab));
    BOOST_TEST(a.checkInfinityNorm(ag));
    BOOST_TEST(!b.checkInfinityNorm(bb));
    BOOST_TEST(b.checkInfinityNorm(bg));
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

BOOST_AUTO_TEST_CASE(Fe2Inv) {
    Fe2 a({-355525067034500326, -826748688154628891});
    Fe2 b({654336260586812980, -209289517407125934});
    BOOST_TEST(a == b.invert().value());
    BOOST_TEST(b == a.invert().value());
    BOOST_TEST(!Fe2(0).invert());
}

BOOST_AUTO_TEST_CASE(Fe3Inv) {
    Fe3 a({911746837231790065, -371393672426824931, 951422396104868392});
    Fe3 b({698047765888851768, -550977866792131069, -50330903586210329});
    BOOST_TEST(a == b.invert().value());
    BOOST_TEST(b == a.invert().value());
    BOOST_TEST(!Fe3(0).invert());
}

BOOST_AUTO_TEST_CASE(Fe4Inv) {
    Fe4 a({1041665893916669997, 596398387750139097, -830556954216199038, 682214497566469235});
    Fe4 b({-21298249253959623, 682404201392544452, 42158526250790604, -476551906469917697});
    BOOST_TEST(a == b.invert().value());
    BOOST_TEST(b == a.invert().value());
    BOOST_TEST(!Fe4(0).invert());
}

BOOST_AUTO_TEST_SUITE_END()

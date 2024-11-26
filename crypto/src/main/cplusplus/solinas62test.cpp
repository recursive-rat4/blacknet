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
#include "solinas62field.h"

BOOST_AUTO_TEST_SUITE(Solinas62)

using Zq = Solinas62Ring;
using Fe2 = Solinas62RingDegree2;
using Fe3 = Solinas62RingDegree3;
using Fe4 = Solinas62RingDegree4;

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
    BOOST_TEST(Zq(0) == Zq(-1) + Zq(1));
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
    Zq d(67584);
    BOOST_TEST(c == a - b);
    BOOST_TEST(d == b - a);
    BOOST_TEST(c == c - Zq(0));
    BOOST_TEST(Zq(0) == Zq(0) - Zq(0));
    BOOST_TEST(Zq(0) == Zq(1) - Zq(1));
}

BOOST_AUTO_TEST_CASE(ZqInv) {
    Zq a(24);
    Zq b(1345075088707988055);
    Zq c(-25);
    Zq d(553402322211286514);
    BOOST_TEST(a == b.invert().value());
    BOOST_TEST(b == a.invert().value());
    BOOST_TEST(c == d.invert().value());
    BOOST_TEST(d == c.invert().value());
    BOOST_TEST(!Zq(0).invert());
}

BOOST_AUTO_TEST_CASE(ZqNeg) {
    Zq a(2074288846126676962);
    Zq b(-2074288846126676962);
    Zq c(-991508920070012190);
    Zq d(991508920070012190);
    BOOST_TEST(b == -a);
    BOOST_TEST(d == -c);
    BOOST_TEST(Zq(0) == -Zq(0));
    BOOST_TEST(Zq(1) == -(-Zq(1)));
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

BOOST_AUTO_TEST_CASE(Fe2Add) {
    Fe2 a({791265566874146615, -157309583225685341});
    Fe2 b({1904858930168201936, -1702068201505429687});
    Fe2 c({2696124497042348551, -1859377784731115028});
    BOOST_TEST(c == a + b);
    BOOST_TEST(c == b + a);
    BOOST_TEST(c == Fe2::LEFT_ADDITIVE_IDENTITY() + c);
    BOOST_TEST(c == c + Fe2::LEFT_ADDITIVE_IDENTITY());
    BOOST_TEST(Fe2::LEFT_MULTIPLICATIVE_IDENTITY() == Fe2::LEFT_MULTIPLICATIVE_IDENTITY() + Fe2::LEFT_ADDITIVE_IDENTITY());
    BOOST_TEST(Fe2::LEFT_MULTIPLICATIVE_IDENTITY() == Fe2::LEFT_ADDITIVE_IDENTITY() + Fe2::LEFT_MULTIPLICATIVE_IDENTITY());
}

BOOST_AUTO_TEST_CASE(Fe2Dbl) {
    Fe2 a({820418911954903445, -717081021288996571});
    Fe2 b({1640837823909806890, -1434162042577993142});
    BOOST_TEST(b == a.douple());
    BOOST_TEST(Fe2::LEFT_ADDITIVE_IDENTITY() == Fe2::LEFT_ADDITIVE_IDENTITY().douple());
}

BOOST_AUTO_TEST_CASE(Fe2Mul) {
    Fe2 a({-562956929497444169, -2169310818437621774});
    Zq b(51280928868087145);
    Fe2 c({-35430973369276012, 1547590517274652700});
    Fe2 d({-1508468004681921080, -1329327685600172110});
    BOOST_TEST(c == a * b);
    BOOST_TEST(d == a * c);
    BOOST_TEST(d == c * a);
    BOOST_TEST(Fe2::LEFT_ADDITIVE_IDENTITY() == Fe2::LEFT_ADDITIVE_IDENTITY() * c);
    BOOST_TEST(Fe2::LEFT_ADDITIVE_IDENTITY() == c * Fe2::LEFT_ADDITIVE_IDENTITY());
    BOOST_TEST(c == c * Fe2::LEFT_MULTIPLICATIVE_IDENTITY());
    BOOST_TEST(c == Fe2::LEFT_MULTIPLICATIVE_IDENTITY() * c);
}

BOOST_AUTO_TEST_CASE(Fe2Sub) {
    Fe2 a({-1967505154088359060, -417234341653690044});
    Fe2 b({-1501942569190230732, -1831258648329169020});
    Fe2 c({-465562584898128328, 1414024306675478976});
    BOOST_TEST(c == a - b);
    BOOST_TEST(c == c - Fe2::LEFT_ADDITIVE_IDENTITY());
    BOOST_TEST(Fe2::LEFT_ADDITIVE_IDENTITY() == Fe2::LEFT_ADDITIVE_IDENTITY() - Fe2::LEFT_ADDITIVE_IDENTITY());
    BOOST_TEST(Fe2::LEFT_ADDITIVE_IDENTITY() == Fe2::LEFT_MULTIPLICATIVE_IDENTITY() - Fe2::LEFT_MULTIPLICATIVE_IDENTITY());
}

BOOST_AUTO_TEST_CASE(Fe2Neg) {
    Fe2 a({-24524642669889579, -1380622575203430167});
    Fe2 b({24524642669889579, 1380622575203430167});
    Fe2 c({31743392249145330, 21532112689806032});
    Fe2 d({-31743392249145330, -21532112689806032});
    BOOST_TEST(b == -a);
    BOOST_TEST(d == -c);
    BOOST_TEST(Fe2(0) == -Fe2(0));
    BOOST_TEST(Fe2(1) == -(-Fe2(1)));
}

BOOST_AUTO_TEST_CASE(Fe3Mul) {
    Fe3 a({1561713001434896052, 1989274817237533064, -172458044588081573});
    Fe3 b({-2100662521769163914, 1850812799403353007, -1397409432835151044});
    Fe3 c({1902737453720704421, 532565984705842277, -845465626277180172});
    BOOST_TEST(c == a * b);
    BOOST_TEST(c == b * a);
    BOOST_TEST(Fe3::LEFT_ADDITIVE_IDENTITY() == Fe3::LEFT_ADDITIVE_IDENTITY() * c);
    BOOST_TEST(Fe3::LEFT_ADDITIVE_IDENTITY() == c * Fe3::LEFT_ADDITIVE_IDENTITY());
    BOOST_TEST(c == c * Fe3::LEFT_MULTIPLICATIVE_IDENTITY());
    BOOST_TEST(c == Fe3::LEFT_MULTIPLICATIVE_IDENTITY() * c);
}

BOOST_AUTO_TEST_CASE(Fe4Mul) {
    Fe4 a({1561713001434896052, 1989274817237533064, -172458044588081573, -567417154093050961});
    Fe4 b({-2100662521769163914, 1850812799403353007, -1397409432835151044, -185428177588484336});
    Fe4 c({873942338409168043, 1148202790106095708, -692872878060617743, -1287171804490455896});
    BOOST_TEST(c == a * b);
    BOOST_TEST(c == b * a);
    BOOST_TEST(Fe4::LEFT_ADDITIVE_IDENTITY() == Fe4::LEFT_ADDITIVE_IDENTITY() * c);
    BOOST_TEST(Fe4::LEFT_ADDITIVE_IDENTITY() == c * Fe4::LEFT_ADDITIVE_IDENTITY());
    BOOST_TEST(c == c * Fe4::LEFT_MULTIPLICATIVE_IDENTITY());
    BOOST_TEST(c == Fe4::LEFT_MULTIPLICATIVE_IDENTITY() * c);
}

BOOST_AUTO_TEST_SUITE_END()

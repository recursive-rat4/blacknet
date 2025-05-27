/*
 * Copyright (c) 2024-2025 Pavel Vasin
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

#include "bigint.h"

using namespace blacknet::crypto;

BOOST_AUTO_TEST_SUITE(BigInts)

BOOST_AUTO_TEST_CASE(even) {
    constexpr UInt256 a("0000000000000000000000000000000000000000000000000000000000000000");
    constexpr UInt256 b("0000000000000000000000000000000000000000000000000000000000000001");
    constexpr UInt256 c("8000000000000000000000000000000000000000000000000000000000000000");
    constexpr UInt256 d("8000000000000000000000000000000000000000000000000000000000000001");
    BOOST_TEST(a.isEven());
    BOOST_TEST(!b.isEven());
    BOOST_TEST(c.isEven());
    BOOST_TEST(!d.isEven());
}

BOOST_AUTO_TEST_CASE(halve) {
    constexpr UInt256 a("e268cd17fad1286c547e4f71e11d5def1cd66c71179cc6260394296a7d39caea");
    constexpr UInt256 b("7134668bfd6894362a3f27b8f08eaef78e6b36388bce631301ca14b53e9ce575");
    constexpr UInt256 c("389a3345feb44a1b151f93dc7847577bc7359b1c45e7318980e50a5a9f4e72ba");
    constexpr UInt256 d("1c4d19a2ff5a250d8a8fc9ee3c23abbde39acd8e22f398c4c072852d4fa7395d");
    constexpr UInt256 e("0e268cd17fad1286c547e4f71e11d5def1cd66c71179cc6260394296a7d39cae");
    BOOST_TEST(b == a.halve());
    BOOST_TEST(c == b.halve());
    BOOST_TEST(d == c.halve());
    BOOST_TEST(e == d.halve());
}

BOOST_AUTO_TEST_CASE(douple) {
    constexpr UInt256 a("e268cd17fad1286c547e4f71e11d5def1cd66c71179cc6260394296a7d39cae0");
    constexpr UInt256 b("7134668bfd6894362a3f27b8f08eaef78e6b36388bce631301ca14b53e9ce570");
    constexpr UInt256 c("389a3345feb44a1b151f93dc7847577bc7359b1c45e7318980e50a5a9f4e72b8");
    constexpr UInt256 d("1c4d19a2ff5a250d8a8fc9ee3c23abbde39acd8e22f398c4c072852d4fa7395c");
    constexpr UInt256 e("0e268cd17fad1286c547e4f71e11d5def1cd66c71179cc6260394296a7d39cae");
    BOOST_TEST(a == b.douple());
    BOOST_TEST(b == c.douple());
    BOOST_TEST(c == d.douple());
    BOOST_TEST(d == e.douple());
}

BOOST_AUTO_TEST_CASE(ant) {
    constexpr UInt256 a("B2DFA0FE5E6FF3E86D499069C13FC781B5BE49BE1C42D6AA2BD8853280195D86");
    constexpr UInt256 b("AB8A146F53EC7D333E18EC7F9F15BE617C9F23028210AF1BD0DCFA12BE765069");
    constexpr UInt256 c("A28A006E526C71202C08806981158601349E01020000860A00D8801280105000");
    constexpr UInt256 d("0000000000000000000000000000000000000000000000000000000000000000");
    constexpr UInt256 e("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");
    BOOST_TEST(c == (a & b));
    BOOST_TEST(d == (a & d));
    BOOST_TEST(a == (a & e));
}

BOOST_AUTO_TEST_CASE(shl) {
    constexpr UInt256 a("C2077969192C8466727494B6D4589D0913670F1ACC7FF5EE284DE8F2E73F623A");
    constexpr UInt256 b("840EF2D2325908CCE4E9296DA8B13A1226CE1E3598FFEBDC509BD1E5CE7EC474");
    constexpr UInt256 c("103BCB48C964233393A4A5B6A2C4E8489B3878D663FFAF71426F479739FB11D0");
    constexpr UInt256 d("81DE5A464B21199C9D252DB516274244D9C3C6B31FFD7B8A137A3CB9CFD88E80");
    constexpr UInt256 e("1DE5A464B21199C9D252DB516274244D9C3C6B31FFD7B8A137A3CB9CFD88E800");
    BOOST_TEST(b == a << 1);
    BOOST_TEST(c == b << 2);
    BOOST_TEST(d == c << 3);
    BOOST_TEST(e == d << 4);
    UInt256 i = a;
    BOOST_TEST(b == (i <<= 1));
    BOOST_TEST(c == (i <<= 2));
    BOOST_TEST(d == (i <<= 3));
    BOOST_TEST(e == (i <<= 4));
}

BOOST_AUTO_TEST_CASE(shr) {
    constexpr UInt256 a("BE6DEFEC052D76C02BC0AE6539ED1494C1738703E0292310FC809FEBF189F62D");
    constexpr UInt256 b("5F36F7F60296BB6015E057329CF68A4A60B9C381F01491887E404FF5F8C4FB16");
    constexpr UInt256 c("17CDBDFD80A5AED8057815CCA73DA292982E70E07C0524621F9013FD7E313EC5");
    constexpr UInt256 d("02F9B7BFB014B5DB00AF02B994E7B4525305CE1C0F80A48C43F2027FAFC627D8");
    constexpr UInt256 e("002F9B7BFB014B5DB00AF02B994E7B4525305CE1C0F80A48C43F2027FAFC627D");
    BOOST_TEST(b == a >> 1);
    BOOST_TEST(c == b >> 2);
    BOOST_TEST(d == c >> 3);
    BOOST_TEST(e == d >> 4);
    UInt256 i = a;
    BOOST_TEST(b == (i >>= 1));
    BOOST_TEST(c == (i >>= 2));
    BOOST_TEST(d == (i >>= 3));
    BOOST_TEST(e == (i >>= 4));
}

BOOST_AUTO_TEST_SUITE_END()

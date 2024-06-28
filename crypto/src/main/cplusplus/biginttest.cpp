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

#include "bigint.h"

BOOST_AUTO_TEST_SUITE(BigInts)

BOOST_AUTO_TEST_CASE(even) {
    UInt256 a; std::istringstream("0000000000000000000000000000000000000000000000000000000000000000") >> a;
    UInt256 b; std::istringstream("0000000000000000000000000000000000000000000000000000000000000001") >> b;
    UInt256 c; std::istringstream("8000000000000000000000000000000000000000000000000000000000000000") >> c;
    UInt256 d; std::istringstream("8000000000000000000000000000000000000000000000000000000000000001") >> d;
    BOOST_TEST(a.isEven());
    BOOST_TEST(!b.isEven());
    BOOST_TEST(c.isEven());
    BOOST_TEST(!d.isEven());
}

BOOST_AUTO_TEST_CASE(halve) {
    UInt256 a; std::istringstream("e268cd17fad1286c547e4f71e11d5def1cd66c71179cc6260394296a7d39caea") >> a;
    UInt256 b; std::istringstream("7134668bfd6894362a3f27b8f08eaef78e6b36388bce631301ca14b53e9ce575") >> b;
    UInt256 c; std::istringstream("389a3345feb44a1b151f93dc7847577bc7359b1c45e7318980e50a5a9f4e72ba") >> c;
    UInt256 d; std::istringstream("1c4d19a2ff5a250d8a8fc9ee3c23abbde39acd8e22f398c4c072852d4fa7395d") >> d;
    UInt256 e; std::istringstream("0e268cd17fad1286c547e4f71e11d5def1cd66c71179cc6260394296a7d39cae") >> e;
    BOOST_TEST(b == a.halve());
    BOOST_TEST(c == b.halve());
    BOOST_TEST(d == c.halve());
    BOOST_TEST(e == d.halve());
}

BOOST_AUTO_TEST_SUITE_END()

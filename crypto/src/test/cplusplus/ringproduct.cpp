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
#include "pervushinextension.h"
#include "ringproduct.h"
#include "solinas62.h"
#include "solinas62extension.h"

using namespace blacknet::crypto;

BOOST_AUTO_TEST_SUITE(ResidueNumberSystems)

using Z1 = FermatRing;
using Z2 = Solinas62Ring;
using Z = RingProduct<Z1, Z2>;

BOOST_AUTO_TEST_CASE(Adds) {
    Z a(Z1(2), Z2(3));
    Z b(Z1(4), Z2(5));
    Z c(Z1(6), Z2(8));
    BOOST_TEST(c == a + b);
    BOOST_TEST(c == b + a);
    BOOST_TEST(a == Z::additive_identity() + a);
    BOOST_TEST(a == a + Z::additive_identity());
}

BOOST_AUTO_TEST_CASE(Dbls) {
    Z a(Z1(7), Z2(9));
    Z b(Z1(14), Z2(18));
    BOOST_TEST(b == a.douple());
    BOOST_TEST(Z::additive_identity() == Z::additive_identity().douple());
}

BOOST_AUTO_TEST_CASE(Muls) {
    Z a(Z1(11), Z2(17));
    Z b(Z1(2), Z2(3));
    Z c(Z1(22), Z2(51));
    BOOST_TEST(c == a * b);
    BOOST_TEST(c == b * a);
    BOOST_TEST(a == Z::multiplicative_identity() * a);
    BOOST_TEST(a == a * Z::multiplicative_identity());
}

BOOST_AUTO_TEST_CASE(Sqrs) {
    Z a(Z1(11), Z2(17));
    Z b(Z1(121), Z2(289));
    BOOST_TEST(b == a.square());
}

BOOST_AUTO_TEST_CASE(Subs) {
    Z a(Z1(80), Z2(90));
    Z b(Z1(20), Z2(10));
    Z c(Z1(60), Z2(80));
    BOOST_TEST(c == a - b);
}

BOOST_AUTO_TEST_CASE(Negs) {
    Z a(Z1(50), Z2(60));
    Z b(Z1(-50), Z2(-60));
    BOOST_TEST(b == -a);
    BOOST_TEST(a == -(-a));
}

BOOST_AUTO_TEST_SUITE_END()

BOOST_AUTO_TEST_SUITE(RingIsomorphisms)

using R1 = PervushinRingDegree2;
using R2 = Solinas62RingDegree2;
using R = RingProduct<R1, R2>;

BOOST_AUTO_TEST_CASE(Adds) {
    R a(R1{2, 3}, R2{4, 5});
    R b(R1{4, 7}, R2{8, 9});
    R c(R1{6, 10}, R2{12, 14});
    BOOST_TEST(c == a + b);
    BOOST_TEST(c == b + a);
    BOOST_TEST(a == R::additive_identity() + a);
    BOOST_TEST(a == a + R::additive_identity());
}

BOOST_AUTO_TEST_CASE(Dbls) {
    R a(R1{2, 3}, R2{4, 5});
    R b(R1{4, 6}, R2{8, 10});
    BOOST_TEST(b == a.douple());
    BOOST_TEST(R::additive_identity() == R::additive_identity().douple());
}

BOOST_AUTO_TEST_CASE(Muls) {
    R a(R1{2, 3}, R2{4, 5});
    R b(R1{4, 7}, R2{8, 9});
    R c(R1{-13, 26}, R2{-1387961572270747680, 76});
    BOOST_TEST(c == a * b);
    BOOST_TEST(c == b * a);
    BOOST_TEST(a == R::multiplicative_identity() * a);
    BOOST_TEST(a == a * R::multiplicative_identity());
}

BOOST_AUTO_TEST_CASE(Sqrs) {
    R a(R1{2, 3}, R2{4, 5});
    R b(R1{-5, 12}, R2{1790958025642577741, 40});
    BOOST_TEST(b == a.square());
    BOOST_TEST(R::multiplicative_identity() == R::multiplicative_identity().square());
}

BOOST_AUTO_TEST_CASE(Subs) {
    R a(R1{4, 7}, R2{8, 9});
    R b(R1{2, 3}, R2{4, 5});
    R c(R1{2, 4}, R2{4, 4});
    BOOST_TEST(c == a - b);
}

BOOST_AUTO_TEST_CASE(Negs) {
    R a(R1{2, 3}, R2{4, 5});
    R b(R1{-2, -3}, R2{-4, -5});
    BOOST_TEST(b == -a);
    BOOST_TEST(a == -(-a));
}

BOOST_AUTO_TEST_SUITE_END()

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

#include "hypercube.h"
#include "latticefold.h"
#include "matrix.h"
#include "solinas62.h"
#include "vector.h"

BOOST_AUTO_TEST_SUITE(LatticeFolds)

using namespace latticefold;

using Z = Solinas62Ring;
using R = Rq<Z>;

BOOST_AUTO_TEST_CASE(Gadget) {
    auto g = gadget<Z>(1, 4);
    auto a = Vector<Z>{ 3, 2, 1, 0 };
    auto b = Vector<Z>{ 4295098371 };
    BOOST_TEST(b == g * a);
}

BOOST_AUTO_TEST_CASE(G1s) {
    std::vector<Z> r1{0, 0, 0, 0, 0, 0};
    std::vector<Z> r2{0, 0, 0, 0, 0, 1};
    R f{3, 4};
    auto g1_1 = G1<Z, R>(r1, f);
    auto g1_2 = G1<Z, R>(r2, f);
    BOOST_TEST(6 == g1_1.variables());
    BOOST_TEST(2 == g1_1.degree());
    BOOST_TEST(Z(3) == g1_1(r1));
    BOOST_TEST(Z(0) == g1_1(r2));
    BOOST_TEST(Z(4) == g1_2(r2));
    BOOST_TEST(Z(0) == g1_2(r1));
}

BOOST_AUTO_TEST_CASE(G2s) {
    std::vector<Z> beta{0, 0, 0, 0, 0, 0};
    R f1{1, -1};
    R f2{2, -2};
    auto g2_1 = G2<Z, R>(beta, f1);
    auto g2_2 = G2<Z, R>(beta, f2);
    BOOST_TEST(6 == g2_1.variables());
    BOOST_TEST(4 == g2_1.degree());
    BOOST_TEST(Hypercube<Z>::checkZero(g2_1));
    BOOST_TEST(!Hypercube<Z>::checkZero(g2_2));
}

BOOST_AUTO_TEST_SUITE_END()

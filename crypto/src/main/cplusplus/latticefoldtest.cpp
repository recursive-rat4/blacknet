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
    Vector<R> f{R{3, 4}};
    auto g1_1 = G1<Z>(r1, f);
    auto g1_2 = G1<Z>(r2, f);
    BOOST_TEST(6 == g1_1.variables());
    BOOST_TEST(2 == g1_1.degree());
    BOOST_TEST(Z(3) == g1_1(r1));
    BOOST_TEST(Z(0) == g1_1(r2));
    BOOST_TEST(Z(4) == g1_2(r2));
    BOOST_TEST(Z(0) == g1_2(r1));
}

BOOST_AUTO_TEST_CASE(G2s) {
    // Commented out deviation from paper
    /*std::vector<Z> beta{0, 0, 0, 0, 0, 0};*/
    Vector<R> f1{R{1, -1}};
    Vector<R> f2{R{2, -2}};
    auto g2_1 = G2<Z>(/*beta,*/ f1);
    auto g2_2 = G2<Z>(/*beta,*/ f2);
    BOOST_TEST(6 == g2_1.variables());
    BOOST_TEST(/*4*/3 == g2_1.degree());
    BOOST_TEST(Hypercube<Z>::checkZero(g2_1));
    /*BOOST_TEST(!Hypercube<Z>::checkZero(g2_2));*/
}

BOOST_AUTO_TEST_CASE(GEvals) {
    std::vector<Z> alpha(k + k, Z(2));
    std::vector<std::vector<Z>> r(k + k, {0, 0, 0, 0, 1, 0});
    std::vector<Vector<R>> f;
    for (std::size_t i = 0; i < k + k; ++i) {
        R rq(0);
        rq.coefficients[i] = Z(i);
        f.emplace_back(Vector<R>{rq});
    }
    auto geval = GEval<Z>(alpha, r, f);
    BOOST_TEST(6 == geval.variables());
    BOOST_TEST(2 == geval.degree());
    BOOST_TEST(Z(0) == geval({0, 0, 0, 0, 0, 1}));
    BOOST_TEST(Z(4) == geval({0, 0, 0, 0, 1, 0}));
}

BOOST_AUTO_TEST_CASE(GNorms) {
    std::vector<Z> beta{0, 0, 0, 0, 0, 0};
    std::vector<Z> mu{1, 1, 1, 1, 1, 1};
    std::vector<Vector<R>> f1(k + k, Vector<R>{R{1, 1, 0, -1}});
    std::vector<Vector<R>> f2(k + k, Vector<R>{R{2, 0, 0, -2}});
    auto geval_1 = GNorm<Z>(beta, mu, f1);
    auto geval_2 = GNorm<Z>(beta, mu, f2);
    BOOST_TEST(6 == geval_1.variables());
    BOOST_TEST(4 == geval_1.degree());
    BOOST_TEST(Hypercube<Z>::checkZero(geval_1));
    BOOST_TEST(!Hypercube<Z>::checkZero(geval_2));
}

BOOST_AUTO_TEST_SUITE_END()

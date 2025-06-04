/*
 * Copyright (c) 2025 Pavel Vasin
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
#include <random>

#include "fastrng.h"
#include "johnsonlindenstrauss.h"
#include "matrix.h"
#include "pervushin.h"
#include "vector.h"

using namespace blacknet::crypto;

static FastDRG rng;

BOOST_AUTO_TEST_SUITE(Johnson_Lindenstrauss)

using Z = PervushinRing;
using JohnsonLindenstrauss = JohnsonLindenstrauss<Z>;

BOOST_AUTO_TEST_CASE(test) {
    int b = 128;
    std::size_t n = 16;
    std::size_t k = 32;
    int slack_inf = 6;
    int slack_ecd = 3;

    std::uniform_int_distribution<int> dst{-b+1, +b-1};
    Matrix<Z> map = JohnsonLindenstrauss::random(rng, n, k);
    Vector<Z> high = Vector<Z>::random(rng, dst, k);
    Vector<Z> low = JohnsonLindenstrauss::project(map, high);

    BOOST_TEST(map * high == low);
    BOOST_TEST(low.checkInfinityNorm(b * slack_inf));
    BOOST_TEST(low.euclideanNorm() < high.euclideanNorm() * slack_ecd);
}

BOOST_AUTO_TEST_SUITE_END()

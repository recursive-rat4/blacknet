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

#include "matrix.h"
#include "solinas62.h"
#include "vector.h"

BOOST_AUTO_TEST_SUITE(Matrices)

using R = Solinas62Ring;

BOOST_AUTO_TEST_CASE(product) {
    Matrix<R> a(3, 2, {
        R(17), R(18),
        R(33), R(34),
        R(49), R(50),
    });
    Vector<R> b{
        R(2),
        R(3),
    };
    Vector<R> c{
        R(88),
        R(168),
        R(248),
    };
    BOOST_TEST(c == a * b);
}

BOOST_AUTO_TEST_SUITE_END()

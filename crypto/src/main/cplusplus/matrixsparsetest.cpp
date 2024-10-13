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
#include "matrixsparse.h"
#include "pervushin.h"
#include "vector.h"

BOOST_AUTO_TEST_SUITE(MatriceSparses)

using R = PervushinRing;
using RE = PervushinRingDegree2;

BOOST_AUTO_TEST_CASE(Conversion) {
    MatrixSparse<R> ms(
        4,
        { 0, 2, 5, 7, 9 },
        { 0, 1, 1, 2, 3, 0, 3, 1, 3 },
        { R(1), R(2), R(3), R(4), R(5), R(6), R(7), R(8), R(9) }
    );
    Matrix<R> md(4, 4, {
        R(1), R(2), R(0), R(0),
        R(0), R(3), R(4), R(5),
        R(6), R(0), R(0), R(7),
        R(0), R(8), R(0), R(9),
    });
    BOOST_TEST(ms == MatrixSparse<R>(md));
    BOOST_TEST(md == ms.dense());
}

BOOST_AUTO_TEST_CASE(Product) {
    MatrixSparse<R> a(
        4,
        { 0, 3, 3, 6, 9, 11 },
        { 0, 1, 3, 0, 1, 3, 0, 1, 3, 1, 3 },
        { R(11), R(12), R(14), R(31), R(32), R(34), R(41), R(42), R(44), R(52), R(54) }
    );
    Vector<R> b{
        R(61),
        R(67),
        R(71),
        R(73),
    };
    Vector<R> c{
        R(2497),
        R(0),
        R(6517),
        R(8527),
        R(7426),
    };
    BOOST_TEST(c == a * b);
    BOOST_TEST(c.template homomorph<RE>() == a.template homomorph<RE>() * b.template homomorph<RE>());
}

BOOST_AUTO_TEST_SUITE_END()

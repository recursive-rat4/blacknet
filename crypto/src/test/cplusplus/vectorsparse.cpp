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

#include "fermat.h"
#include "matrixdense.h"
#include "vectordense.h"
#include "vectorsparse.h"

using namespace blacknet::crypto;

BOOST_AUTO_TEST_SUITE(VectorSparses)

using R = FermatRing;

BOOST_AUTO_TEST_CASE(Conversion) {
    VectorSparse<R> vs(
        8,
        { 0, 2, 5, 7 },
        { R(1), R(2), R(3), R(4) }
    );
    VectorDense<R> vd({
        R(1), R(0), R(2), R(0), R(0), R(3), R(0), R(4),
    });
    BOOST_TEST(vs == VectorSparse<R>(vd));
    BOOST_TEST(vd == vs.dense());
}

BOOST_AUTO_TEST_CASE(Product) {
    MatrixDense<R> a(2, 4, {
        R(11), R(13), R(17), R(19),
        R(23), R(29), R(31), R(37),
    });
    VectorSparse<R> b(
        4,
        { 1, 2 },
        { R(3), R(5) }
    );
    VectorDense<R> c{
        R(124),
        R(242),
    };
    BOOST_TEST(c == a * b);
}

BOOST_AUTO_TEST_SUITE_END()

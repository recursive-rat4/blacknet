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
#include "pastacurves.h"

BOOST_AUTO_TEST_SUITE(Matrices)

BOOST_AUTO_TEST_CASE(product) {
    Matrix<PallasField> a(3, 2, {
        PallasField(17), PallasField(18),
        PallasField(33), PallasField(34),
        PallasField(49), PallasField(50),
    });
    Vector<PallasField> b{
        PallasField(2),
        PallasField(3),
    };
    Vector<PallasField> c{
        PallasField(88),
        PallasField(168),
        PallasField(248),
    };
    BOOST_TEST(c == a * b);
}

BOOST_AUTO_TEST_SUITE_END()

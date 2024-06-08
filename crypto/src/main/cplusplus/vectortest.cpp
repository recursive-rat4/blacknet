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

#include "pastacurves.h"
#include "vector.h"

BOOST_AUTO_TEST_SUITE(Vectors)

BOOST_AUTO_TEST_CASE(HadamardSummation) {
    Vector<PallasField> a{
        PallasField(0),
        PallasField(4),
        PallasField(2),
    };
    Vector<PallasField> b{
        PallasField(7),
        PallasField(3),
        PallasField(5),
    };
    Vector<PallasField> c{
        PallasField(7),
        PallasField(7),
        PallasField(7),
    };
    BOOST_TEST(c == a + b);
    BOOST_TEST(c == b + a);
}

BOOST_AUTO_TEST_CASE(HadamardProduct) {
    Vector<PallasField> a{
        PallasField(2),
        PallasField(2),
        PallasField(2),
    };
    Vector<PallasField> b{
        PallasField(1),
        PallasField(2),
        PallasField(4),
    };
    Vector<PallasField> c{
        PallasField(2),
        PallasField(4),
        PallasField(8),
    };
    BOOST_TEST(c == a * b);
    BOOST_TEST(c == b * a);
}

BOOST_AUTO_TEST_CASE(ScalarProduct) {
    Vector<PallasField> a{
        PallasField(4),
        PallasField(5),
        PallasField(6),
    };
    PallasField b(2);
    Vector<PallasField> c{
        PallasField(8),
        PallasField(10),
        PallasField(12),
    };
    BOOST_TEST(c == a * b);
}

BOOST_AUTO_TEST_SUITE_END()

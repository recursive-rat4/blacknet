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

#include "latticefold.h"
#include "matrix.h"
#include "solinas62.h"
#include "vector.h"

using Z = Solinas62Ring;

BOOST_AUTO_TEST_SUITE(LatticeFolds)

BOOST_AUTO_TEST_CASE(gadget) {
    auto g = latticefold::gadget<Z>(1, 4);
    auto a = Vector<Z>{ 3, 2, 1, 0 };
    auto b = Vector<Z>{ 4295098371 };
    BOOST_TEST(b == g * a);
}

BOOST_AUTO_TEST_SUITE_END()

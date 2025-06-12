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
#include "latticegadget.h"
#include "vector.h"

using namespace blacknet::crypto;

BOOST_AUTO_TEST_SUITE(LatticeGadgets)

using Z = FermatRing;

BOOST_AUTO_TEST_CASE(Zs) {
    Z a(-18135);
    Vector<Z> b{0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 1, 0};
    auto c = LatticeGadget<Z>::decompose(2, Z::bits(), a);
    BOOST_TEST(b == c);
}

BOOST_AUTO_TEST_SUITE_END()

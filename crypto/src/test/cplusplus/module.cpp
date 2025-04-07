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

#include "module.h"
#include "pervushin.h"

using namespace blacknet::crypto;

BOOST_AUTO_TEST_SUITE(Modules)

using R = PervushinRing;
using M = Module<R, 2>;

BOOST_AUTO_TEST_CASE(test) {
    R r(3);
    R s(5);
    M x({R(7), R(11)});
    M y({R(13), R(17)});

    BOOST_TEST(r * (x + y) == r * x + r * y);
    BOOST_TEST((r + s) * x == r * x + s * x);
    BOOST_TEST((r * s) * x == r * (s * x));
    BOOST_TEST(R(1) * x == x);
}

BOOST_AUTO_TEST_SUITE_END()

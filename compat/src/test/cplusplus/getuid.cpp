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

#include "getuid.h"

using namespace blacknet;

BOOST_AUTO_TEST_SUITE(getuids)

BOOST_AUTO_TEST_CASE(call) {
#ifdef BLACKNET_HAVE_UNISTD
    BOOST_TEST(-1 != compat::getuid());
#else
    BOOST_TEST(-1 == compat::getuid());
#endif
}

BOOST_AUTO_TEST_SUITE_END()

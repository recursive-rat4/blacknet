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

#include <string_view>

#include "base64.h"
#include "boost-print.h"
#include "byte.h"

using namespace blacknet::codec;
using namespace blacknet::compat;

BOOST_AUTO_TEST_SUITE(Base64s)

BOOST_AUTO_TEST_CASE(tests) {
    const auto bytes = byte::arrayU<4>({ 0, 1, 2, 3 });
    const std::string_view string{"AAECAw=="};

    auto encoded = base64::encode(bytes);
    auto decoded = base64::decode(string);

    BOOST_TEST(string == encoded);
    BOOST_CHECK_EQUAL_COLLECTIONS(bytes.begin(), bytes.end(), decoded.begin(), decoded.end());
}

BOOST_AUTO_TEST_SUITE_END()

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

#include "byte.h"
#include "siphash.h"

namespace byte = blacknet::compat::byte;
using namespace blacknet::crypto;

BOOST_AUTO_TEST_SUITE(SipHashs)

BOOST_AUTO_TEST_CASE(Paper) {
    const auto data = byte::arrayU<15>({
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14
    });
    const auto key = byte::arrayU<16>({
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15
    });
    const uint64_t hash = 0xA129CA6149BE45E5;
    siphash_64 hasher(key);
    hasher.update(data.data(), data.size());
    BOOST_TEST(hash == hasher.result());
}

BOOST_AUTO_TEST_SUITE_END()

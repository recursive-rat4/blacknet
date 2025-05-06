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
#include "ripemd.h"

namespace byte = blacknet::compat::byte;
using namespace blacknet::crypto;

BOOST_AUTO_TEST_SUITE(RIPEMD160s)

BOOST_AUTO_TEST_CASE(empty) {
    const auto hash = byte::arrayU<20>({
        0x9C, 0x11, 0x85, 0xA5,
        0xC5, 0xE9, 0xFC, 0x54,
        0x61, 0x28, 0x08, 0x97,
        0x7E, 0xE8, 0xF5, 0x48,
        0xB2, 0x25, 0x8D, 0x31
    });
    ripemd_160 hasher;
    BOOST_TEST(hash == hasher.result());
}

BOOST_AUTO_TEST_SUITE_END()

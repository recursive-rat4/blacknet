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

#include "poseidon2solinas62.h"

BOOST_AUTO_TEST_SUITE(Poseidons)

BOOST_AUTO_TEST_CASE(block64) {
    using E = Solinas62Ring;
    const auto& params = Poseidon2Solinas62;
    std::vector<E> a{
        0x0000000000000000,
        0x0000000000000001,
        0x0000000000000002,
        0x0000000000000003,
        0x0000000000000004,
        0x0000000000000005,
        0x0000000000000006,
        0x0000000000000007,
        0x0000000000000008,
        0x0000000000000009,
        0x000000000000000a,
        0x000000000000000b,
    };
    std::vector<E> b{
        0x367dbec705769f69,
        0x12b6981be17dd745,
        0x1452150cc1c0ac4e,
        0x3daf481da739b2c7,
        0x260239f977f3729f,
        0x3e9ec143319375c2,
        0x16e5963a9ff7fde6,
        0x08a35acef1bc9fb7,
        0x2bb9c91efc879f55,
        0x059393d79ef06150,
        0x121111905f948bd4,
        0x0bce9d1ef0e19aeb,
    };
    BOOST_TEST(b == poseidon2::permute(params, a));
}

BOOST_AUTO_TEST_SUITE_END()

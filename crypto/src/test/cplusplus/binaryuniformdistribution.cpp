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
#include <algorithm>
#include <cstdint>

#include "binaryuniformdistribution.h"

using namespace blacknet::crypto;

BOOST_AUTO_TEST_SUITE(BinaryUniformDistributions)

struct FixedGenerator {
    using result_type = std::uint16_t;

    result_type i{1234};

    result_type operator () () {
        return i++;
    }

    consteval static result_type min() {
        return std::numeric_limits<result_type>::min();
    }
    consteval static result_type max() {
        return std::numeric_limits<result_type>::max();
    }
};

BOOST_AUTO_TEST_CASE(Reproducible) {
    FixedGenerator g;
    BinaryUniformDistribution<uint8_t, FixedGenerator> bud;
    const std::array<uint8_t, 16> a{0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0};
    std::array<uint8_t, 16> b;
    std::ranges::generate(b, [&]() { return bud(g); });
    BOOST_TEST(a == b);
}

BOOST_AUTO_TEST_SUITE_END()

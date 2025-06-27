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
#include <array>
#include <cstdint>
#include <limits>

#include "fermat.h"
#include "ternaryuniformdistribution.h"

using namespace blacknet::crypto;

BOOST_AUTO_TEST_SUITE(TernaryUniformDistributions)

struct FixedGenerator {
    using result_type = std::uint16_t;

    result_type i{0xE2E4};

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
    using Z = FermatRing;
    FixedGenerator g;
    TernaryUniformDistribution<Z, FixedGenerator> tud;
    const std::array<Z, 6> a{-1, 0, 1, 1, -1, 1};
    std::array<Z, 6> b;
    std::ranges::generate(b, [&]() { return tud(g); });
    BOOST_TEST(a == b);
}

BOOST_AUTO_TEST_SUITE_END()

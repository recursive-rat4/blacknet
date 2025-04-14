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

#include "fastrng.h"

using namespace blacknet::crypto;

BOOST_AUTO_TEST_SUITE(FastRNGs)

BOOST_AUTO_TEST_CASE(discards) {
    constexpr std::size_t size = FastDRG::word_count * 2 + 1;
    using Buf = std::array<FastDRG::result_type, size>;
    FastDRG drg;

    Buf buf1;
    std::ranges::generate(buf1, [&drg] { return drg(); });
    Buf buf2;
    std::ranges::generate(buf2, [&drg] { return drg(); });

    drg.seed(FastDRG::default_seed);
    drg.discard(size);
    Buf buf3;
    std::ranges::generate(buf3, [&drg] { return drg(); });

    BOOST_TEST(buf2 == buf3);
}

BOOST_AUTO_TEST_SUITE_END()

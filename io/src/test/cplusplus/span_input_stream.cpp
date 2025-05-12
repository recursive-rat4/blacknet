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
#include <bit>

#include "byte.h"
#include "span_input_stream.h"

using namespace blacknet;

BOOST_AUTO_TEST_SUITE(SpanInputStreams)

BOOST_AUTO_TEST_CASE(tests) {
    const auto bytes = compat::byte::arrayU<22>({
        0, 0, 1, 2,
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14,
        0x34, 0x34, 0x34
    });
    const auto bs = compat::byte::arrayU<3>({
        0, 1, 2
    });
    const std::string_view string{"444"};

    std::array<std::byte, 3> bd;
    io::span_input_stream<std::endian::big> sis(bytes);
    BOOST_TEST((std::byte{} == sis.read()));
    sis.read(bd);
    BOOST_TEST(bs == bd);
    BOOST_TEST(0x00 == sis.read_u8());
    BOOST_TEST(0x0102 == sis.read_u16());
    BOOST_TEST(0x03040506 == sis.read_u32());
    BOOST_TEST(0x0708090A0B0C0D0E == sis.read_u64());
    std::string ss(3, '\0');
    sis.read_str(ss);
    BOOST_TEST(string == ss);

    BOOST_CHECK_THROW(sis.read(), std::out_of_range);
    BOOST_CHECK_THROW(sis.read(bd), std::out_of_range);
}

BOOST_AUTO_TEST_SUITE_END()

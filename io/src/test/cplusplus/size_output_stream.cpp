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
#include "size_output_stream.h"

using namespace blacknet;

BOOST_AUTO_TEST_SUITE(SizeOutputStreams)

BOOST_AUTO_TEST_CASE(tests) {
    const auto bytes = compat::byte::arrayU<3>({
        0, 1, 2
    });
    const std::string_view string{"444"};

    io::size_output_stream sos;
    sos.write(std::byte{});
    sos.write(bytes);
    sos.write_u8(0x00);
    sos.write_u16(0x0102);
    sos.write_u32(0x03040506);
    sos.write_u64(0x0708090A0B0C0D0E);
    sos.write_str(string);

    BOOST_TEST(22 == sos.size);
}

BOOST_AUTO_TEST_SUITE_END()

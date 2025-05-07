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
#include "hash_output_stream.h"
#include "siphash.h"

using namespace blacknet;

BOOST_AUTO_TEST_SUITE(HashOutputStreams)

BOOST_AUTO_TEST_CASE(tests) {
    const auto key = compat::byte::arrayU<16>({
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15
    });
    const uint64_t hash = 0xA129CA6149BE45E5;

    io::hash_output_stream<crypto::siphash_64, std::endian::big> os(key);

    os.write_u8(0x00);
    os.write_u16(0x0102);
    os.write_u32(0x03040506);
    os.write_u64(0x0708090A0B0C0D0E);

    BOOST_TEST(hash == os.digest());
}

BOOST_AUTO_TEST_SUITE_END()

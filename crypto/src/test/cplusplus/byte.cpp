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

#include <cstddef>
#include <array>
#include <bit>

#include "byte.h"
#include "util.h"

BOOST_AUTO_TEST_SUITE(Bytes)

BOOST_AUTO_TEST_CASE(array) {
    constexpr std::array<int8_t, 3> a{ 1, 2, -1 };
    constexpr std::array<uint8_t, 3> b{ 1, 2, 255 };
    constexpr std::array<std::byte, 3> c{ std::byte(1), std::byte(2), std::byte(0xFF) };

    BOOST_TEST(c == byte::arrayS<3>(a));
    BOOST_TEST(c == byte::arrayU<3>(b));
}

BOOST_AUTO_TEST_CASE(big) {
    constexpr std::endian endian = std::endian::big;
    const uint16_t a1 = 0x01FF;
    const uint32_t b1 = 0x0201FFFE;
    const uint64_t c1 = 0x04030201FFFEFDFC;
    const auto a2 = byte::arrayS<2>({ 1, -1 });
    const auto b2 = byte::arrayS<4>({ 2, 1, -1, -2 });
    const auto c2 = byte::arrayS<8>({ 4, 3, 2, 1, -1, -2, -3, -4 });

    BOOST_TEST((a1 == byte::read<uint16_t, endian>(a2.data())));
    BOOST_TEST((b1 == byte::read<uint32_t, endian>(b2.data())));
    BOOST_TEST((c1 == byte::read<uint64_t, endian>(c2.data())));

    std::array<std::byte, 2> a3;
    std::array<std::byte, 4> b3;
    std::array<std::byte, 8> c3;
    byte::write<uint16_t, endian>(a3.data(), a1);
    byte::write<uint32_t, endian>(b3.data(), b1);
    byte::write<uint64_t, endian>(c3.data(), c1);

    BOOST_TEST(a2 == a3);
    BOOST_TEST(b2 == b3);
    BOOST_TEST(c2 == c3);
}

BOOST_AUTO_TEST_CASE(little) {
    constexpr std::endian endian = std::endian::little;
    const uint16_t a1 = 0x01FF;
    const uint32_t b1 = 0x0201FFFE;
    const uint64_t c1 = 0x04030201FFFEFDFC;
    const auto a2 = byte::arrayS<2>({ -1, 1 });
    const auto b2 = byte::arrayS<4>({ -2, -1, 1, 2 });
    const auto c2 = byte::arrayS<8>({ -4, -3, -2, -1, 1, 2, 3, 4 });

    BOOST_TEST((a1 == byte::read<uint16_t, endian>(a2.data())));
    BOOST_TEST((b1 == byte::read<uint32_t, endian>(b2.data())));
    BOOST_TEST((c1 == byte::read<uint64_t, endian>(c2.data())));

    std::array<std::byte, 2> a3;
    std::array<std::byte, 4> b3;
    std::array<std::byte, 8> c3;
    byte::write<uint16_t, endian>(a3.data(), a1);
    byte::write<uint32_t, endian>(b3.data(), b1);
    byte::write<uint64_t, endian>(c3.data(), c1);

    BOOST_TEST(a2 == a3);
    BOOST_TEST(b2 == b3);
    BOOST_TEST(c2 == c3);
}

BOOST_AUTO_TEST_SUITE_END()

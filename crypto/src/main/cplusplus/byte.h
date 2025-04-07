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

#ifndef BLACKNET_CRYPTO_BYTE_H
#define BLACKNET_CRYPTO_BYTE_H

#include <cstddef>
#include <algorithm>
#include <array>
#include <bit>

namespace blacknet::crypto {

namespace byte {
/**
 * Reads an `Integer` value from the byte representation in the `endian` order.
 */
template<typename Integer, std::endian endian>
Integer read(const std::byte* memory) {
    static_assert(
        std::endian::native == std::endian::big ||
        std::endian::native == std::endian::little,
        "Mixed endian is not implemented"
    );
    Integer integer;
    std::byte* const pointer = reinterpret_cast<std::byte*>(&integer);
    if constexpr (endian == std::endian::native)
        std::ranges::copy(memory, memory + sizeof(Integer), pointer);
    else
        std::ranges::reverse_copy(memory, memory + sizeof(Integer), pointer);
    return integer;
}

/**
 * Writes an `Integer` value into the byte representation in the `endian` order.
 */
template<typename Integer, std::endian endian>
void write(std::byte* memory, Integer integer) {
    static_assert(
        std::endian::native == std::endian::big ||
        std::endian::native == std::endian::little,
        "Mixed endian is not implemented"
    );
    const std::byte* const pointer = reinterpret_cast<const std::byte*>(&integer);
    if constexpr (endian == std::endian::native)
        std::ranges::copy(pointer, pointer + sizeof(Integer), memory);
    else
        std::ranges::reverse_copy(pointer, pointer + sizeof(Integer), memory);
}

/**
 * Returns a `std::array<std::byte, N>` containing the specified bytes
 * represented as `std::array<int8_t, N>`.
 */
template<std::size_t N>
consteval std::array<std::byte, N> arrayS(const std::array<int8_t, N>& ints) {
    std::array<std::byte, N> result;
    std::ranges::transform(ints, result.begin(), [](int8_t i) { return std::byte(i); });
    return result;
}

/**
 * Returns a `std::array<std::byte, N>` containing the specified bytes
 * represented as `std::array<uint8_t, N>`.
 */
template<std::size_t N>
consteval std::array<std::byte, N> arrayU(const std::array<uint8_t, N>& ints) {
    std::array<std::byte, N> result;
    std::ranges::transform(ints, result.begin(), [](uint8_t i) { return std::byte(i); });
    return result;
}
}

}

#endif

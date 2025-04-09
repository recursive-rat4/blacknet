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

#ifndef BLACKNET_COMPAT_BYTE_H
#define BLACKNET_COMPAT_BYTE_H

#include <cstddef>
#include <algorithm>
#include <array>
#include <bit>
#include <ostream>
#include <type_traits>
#include <fmt/format.h>
#include <fmt/ostream.h>

namespace blacknet::compat {

namespace byte {
/**
 * Reads a `T` object from the byte representation in the `endian` order.
 */
template<typename T, std::endian endian>
requires(std::is_trivially_copyable_v<T>)
T read(const std::byte* memory) {
    static_assert(
        std::endian::native == std::endian::big ||
        std::endian::native == std::endian::little,
        "Mixed endian is not implemented"
    );
    T object;
    std::byte* const pointer = reinterpret_cast<std::byte*>(&object);
    if constexpr (endian == std::endian::native)
        std::ranges::copy(memory, memory + sizeof(T), pointer);
    else
        std::ranges::reverse_copy(memory, memory + sizeof(T), pointer);
    return object;
}

/**
 * Writes a `T` object into the byte representation in the `endian` order.
 */
template<typename T, std::endian endian>
requires(std::is_trivially_copyable_v<T>)
void write(std::byte* memory, T object) {
    static_assert(
        std::endian::native == std::endian::big ||
        std::endian::native == std::endian::little,
        "Mixed endian is not implemented"
    );
    const std::byte* const pointer = reinterpret_cast<const std::byte*>(&object);
    if constexpr (endian == std::endian::native)
        std::ranges::copy(pointer, pointer + sizeof(T), memory);
    else
        std::ranges::reverse_copy(pointer, pointer + sizeof(T), memory);
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

namespace std {
    inline std::ostream& operator << (std::ostream& out, const std::byte val)
    {
        fmt::print(out, "0x{:02X}", val);
        return out;
    }
}

#endif

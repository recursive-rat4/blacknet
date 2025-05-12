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

#ifndef BLACKNET_IO_DATA_INPUT_STREAM_H
#define BLACKNET_IO_DATA_INPUT_STREAM_H

#include <cstdint>
#include <array>
#include <bit>

#include "byte.h"
#include "input_stream.h"

namespace blacknet::io {

template<std::endian endian>
struct data_input_stream : public input_stream {
    std::uint8_t read_u8() override {
        std::array<std::byte, 1> scratch;
        read(scratch);
        return compat::byte::read<std::uint8_t, endian>(scratch.data());
    }
    std::uint16_t read_u16() override {
        std::array<std::byte, 2> scratch;
        read(scratch);
        return compat::byte::read<std::uint16_t, endian>(scratch.data());
    }
    std::uint32_t read_u32() override {
        std::array<std::byte, 4> scratch;
        read(scratch);
        return compat::byte::read<std::uint32_t, endian>(scratch.data());
    }
    std::uint64_t read_u64() override {
        std::array<std::byte, 8> scratch;
        read(scratch);
        return compat::byte::read<std::uint64_t, endian>(scratch.data());
    }
    void read_str(const std::span<char>& s) override {
        read({reinterpret_cast<std::byte*>(s.data()), s.size()});
    }
};

}

#endif

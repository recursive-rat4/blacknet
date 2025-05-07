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

#ifndef BLACKNET_IO_DATA_OUTPUT_STREAM_H
#define BLACKNET_IO_DATA_OUTPUT_STREAM_H

#include <cstdint>
#include <array>
#include <bit>

#include "byte.h"
#include "output_stream.h"

namespace blacknet::io {

template<std::endian endian>
struct data_output_stream : public output_stream {
    void write_u8(std::uint8_t u) override {
        std::array<std::byte, 1> scratch;
        compat::byte::write<std::uint8_t, endian>(scratch.data(), u);
        write(scratch);
    }
    void write_u16(std::uint16_t u) override {
        std::array<std::byte, 2> scratch;
        compat::byte::write<std::uint16_t, endian>(scratch.data(), u);
        write(scratch);
    }
    void write_u32(std::uint32_t u) override {
        std::array<std::byte, 4> scratch;
        compat::byte::write<std::uint32_t, endian>(scratch.data(), u);
        write(scratch);
    }
    void write_u64(std::uint64_t u) override {
        std::array<std::byte, 8> scratch;
        compat::byte::write<std::uint64_t, endian>(scratch.data(), u);
        write(scratch);
    }
};

}

#endif

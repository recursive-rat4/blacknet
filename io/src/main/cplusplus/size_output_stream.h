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

#ifndef BLACKNET_IO_SIZE_OUTPUT_STREAM_H
#define BLACKNET_IO_SIZE_OUTPUT_STREAM_H

#include <cstddef>
#include <cstdint>
#include <span>
#include <string_view>

#include "output_stream.h"

namespace blacknet::io {

struct size_output_stream final : public output_stream {
    std::size_t size{0};

    void write(std::byte) override {
        size += 1;
    }
    void write(const std::span<const std::byte>& b) override {
        size += b.size();
    }

    void write_u8(std::uint8_t) override {
        size += 1;
    }
    void write_u16(std::uint16_t) override {
        size += 2;
    }
    void write_u32(std::uint32_t) override {
        size += 4;
    }
    void write_u64(std::uint64_t) override {
        size += 8;
    }
    void write_str(const std::string_view& s) override {
        size += s.size();
    }
};

}

#endif

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

#ifndef BLACKNET_IO_OUTPUT_STREAM_H
#define BLACKNET_IO_OUTPUT_STREAM_H

#include <cstddef>
#include <cstdint>
#include <span>

namespace blacknet::io {

struct output_stream {
    virtual ~output_stream() noexcept = default;

    virtual void write(std::byte b) = 0;
    virtual void write(const std::span<const std::byte>& b) = 0;

    virtual void write_u8(std::uint8_t u) = 0;
    virtual void write_u16(std::uint16_t u) = 0;
    virtual void write_u32(std::uint32_t u) = 0;
    virtual void write_u64(std::uint64_t u) = 0;
};

}

#endif

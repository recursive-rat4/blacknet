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

#ifndef BLACKNET_IO_INPUT_STREAM_H
#define BLACKNET_IO_INPUT_STREAM_H

#include <cstddef>
#include <cstdint>
#include <span>

namespace blacknet::io {

struct input_stream {
    virtual ~input_stream() noexcept = default;

    virtual std::byte read() = 0;
    virtual void read(const std::span<std::byte>& b) = 0;

    virtual std::uint8_t read_u8() = 0;
    virtual std::uint16_t read_u16() = 0;
    virtual std::uint32_t read_u32() = 0;
    virtual std::uint64_t read_u64() = 0;
    virtual void read_str(const std::span<char>& s) = 0;
};

}

#endif

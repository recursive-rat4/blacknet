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

#ifndef BLACKNET_IO_SPAN_OUTPUT_STREAM_H
#define BLACKNET_IO_SPAN_OUTPUT_STREAM_H

#include <cstddef>
#include <algorithm>
#include <span>

#include "data_output_stream.h"

namespace blacknet::io {

template<std::endian endian, std::size_t extent = std::dynamic_extent>
class span_output_stream final : public data_output_stream<endian> {
    std::span<std::byte, extent> span;
    std::size_t pos{0};
public:
    span_output_stream(const std::span<std::byte, extent>& span) :
        data_output_stream<endian>(),
        span(span) {}

    void write(std::byte b) override {
        span[pos] = b;
        pos += 1;
    }
    void write(const std::span<const std::byte>& b) override {
        std::ranges::copy(b, span.data() + pos);
        pos += b.size();
    }
};

}

#endif

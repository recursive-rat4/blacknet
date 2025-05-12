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

#ifndef BLACKNET_IO_SPAN_INPUT_STREAM_H
#define BLACKNET_IO_SPAN_INPUT_STREAM_H

#include <cstddef>
#include <algorithm>
#include <span>
#include <stdexcept>

#include "data_input_stream.h"

namespace blacknet::io {

template<std::endian endian, std::size_t extent = std::dynamic_extent>
class span_input_stream final : public data_input_stream<endian> {
    std::span<const std::byte, extent> span;
    std::size_t pos{0};
public:
    span_input_stream(const std::span<const std::byte, extent>& span) :
        data_input_stream<endian>(),
        span(span) {}

    std::byte read() override {
        if (pos < span.size()) {
            return span[pos++];
        } else {
            throw std::out_of_range("End of stream");
        }
    }
    void read(const std::span<std::byte>& b) override {
        if (pos + b.size() < span.size() + 1) {
            std::ranges::copy(span.data() + pos, span.data() + pos + b.size(), b.data());
            pos += b.size();
        } else {
            throw std::out_of_range("End of stream");
        }
    }
};

}

#endif

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

#ifndef BLACKNET_IO_HASH_OUTPUT_STREAM_H
#define BLACKNET_IO_HASH_OUTPUT_STREAM_H

#include <cstddef>
#include <span>
#include <utility>

#include "data_output_stream.h"

namespace blacknet::io {

template<typename Hasher, std::endian endian>
class hash_output_stream final : public data_output_stream<endian> {
    Hasher hasher;
public:
    template<typename... Args>
    hash_output_stream(Args... args) :
        data_output_stream<endian>(),
        hasher(std::forward<Args>(args)...) {}

    void write(std::byte b) override {
        hasher.update(b);
    }
    void write(const std::span<const std::byte>& b) override {
        hasher.update(b.data(), b.size());
    }

    decltype(auto) digest() {
        return hasher.result();
    }
};

}

#endif

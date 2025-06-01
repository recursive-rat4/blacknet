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

#ifndef BLACKNET_IO_FILE_INPUT_STREAM_H
#define BLACKNET_IO_FILE_INPUT_STREAM_H

#include <cstddef>
#include <filesystem>
#include <fstream>

#include "data_input_stream.h"

namespace blacknet::io {

template<std::endian endian>
class file_input_stream final : public data_input_stream<endian> {
    std::ifstream ifs;
public:
    file_input_stream(
        const std::filesystem::path& path,
        std::ios::openmode mode = std::ios::in
    ) :
        data_input_stream<endian>(),
        ifs(path, mode)
    {
        ifs.exceptions(std::ifstream::failbit);
    }

    std::byte read() override {
        std::byte result;
        ifs.read(reinterpret_cast<char*>(&result), 1);
        return result;
    }
    void read(const std::span<std::byte>& b) override {
        ifs.read(reinterpret_cast<char*>(b.data()), b.size());
    }

    void close() {
        ifs.close();
    }
};

}

#endif

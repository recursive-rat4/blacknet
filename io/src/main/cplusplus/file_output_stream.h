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

#ifndef BLACKNET_IO_FILE_OUTPUT_STREAM_H
#define BLACKNET_IO_FILE_OUTPUT_STREAM_H

#include <cstddef>
#include <filesystem>
#include <fstream>

#include "data_output_stream.h"
#include "fdatasync.h"

namespace blacknet::io {

template<std::endian endian>
class file_output_stream final : public data_output_stream<endian> {
    std::ofstream ofs;
public:
    file_output_stream(
        const std::filesystem::path& path,
        std::ios::openmode mode = std::ios::out
    ) :
        data_output_stream<endian>(),
        ofs(path, mode)
    {
        ofs.exceptions(std::ofstream::failbit);
    }
    file_output_stream(
        std::ofstream&& ofs
    ) :
        data_output_stream<endian>(),
        ofs(std::move(ofs))
    {
        ofs.exceptions(std::ofstream::failbit);
    }

    void write(std::byte b) override {
        ofs.write(reinterpret_cast<char*>(&b), 1);
    }
    void write(const std::span<const std::byte>& b) override {
        ofs.write(reinterpret_cast<const char*>(b.data()), b.size());
    }

    void flush() {
        ofs.flush();
    }
    void datasync() {
        compat::fdatasync(ofs.native_handle());
    }
    void close() {
        ofs.close();
    }
};

}

#endif

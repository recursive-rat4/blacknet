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

#ifndef BLACKNET_IO_FILE_H
#define BLACKNET_IO_FILE_H

#include "blacknet-config.h"

#include <chrono>
#include <exception>
#include <filesystem>
#include <fstream>
#include <functional>
#include <string_view>
#include <utility>
#include <fmt/format.h>

#include "fastrng.h"
#include "fdatasync.h"

namespace blacknet::io {

namespace file {

// In milliseconds since UNIX epoch not counting leap seconds
inline int64_t last_write_time(
    const std::filesystem::path& path
) {
    auto fs_time = std::filesystem::last_write_time(path);
    auto unix_time = std::chrono::clock_cast<std::chrono::system_clock>(fs_time);
    auto milli_time = std::chrono::time_point_cast<std::chrono::milliseconds>(unix_time);
    return milli_time.time_since_epoch().count();
}

inline std::pair<
    std::filesystem::path,
    std::ofstream
> create_temp_file(
    const std::filesystem::path& dir,
    const std::string_view& prefix
) {
    auto& rng = crypto::tls_fast_rng;
    std::filesystem::path path;
    std::ofstream ofs;
    do {
        path = dir / fmt::format("{}-{}", prefix, rng());
        ofs.open(path, std::ios::out | std::ios::noreplace);
    } while (!ofs.is_open());
    return { std::move(path), std::move(ofs) };
}

// Atomically replace file
inline void replace(
    const std::filesystem::path& dir,
    const std::string_view& name,
    const std::function<void(std::ostream&)>& writer
) {
    auto [path, ofs] = create_temp_file(dir, name);
    try {
        writer(ofs);
        ofs.flush();
        compat::fdatasync(ofs.native_handle());
#ifdef BLACKNET_HAVE_FILEAPI
        ofs.close();
#endif
        std::filesystem::rename(path, dir / name);
    } catch (...) {
        std::exception_ptr eptr = std::current_exception();
#ifdef BLACKNET_HAVE_FILEAPI
        try {
            ofs.close();
        } catch (...) {
            // Ignore failed cleanup
        }
#endif
        try {
            std::filesystem::remove(path);
        } catch (...) {
            // Ignore failed cleanup
        }
        std::rethrow_exception(eptr);
    }
}

}

}

#endif

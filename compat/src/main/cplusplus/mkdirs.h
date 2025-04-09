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

#ifndef BLACKNET_COMPAT_MKDIRS_H
#define BLACKNET_COMPAT_MKDIRS_H

#include "blacknet-config.h"

#ifdef BLACKNET_HAVE_SYS_STAT
#include <sys/stat.h>
#endif

#include <filesystem>

namespace blacknet::compat {

#ifdef BLACKNET_HAVE_SYS_STAT
class UMasker {
    mode_t prev;
public:
    UMasker(std::filesystem::perms permissions) noexcept {
        prev = umask(~static_cast<mode_t>(permissions));
    }
    ~UMasker() noexcept {
        umask(prev);
    }
    UMasker(const UMasker&) = delete;
    UMasker(UMasker&&) = delete;
    UMasker& operator = (const UMasker&) = delete;
    UMasker& operator = (UMasker&&) = delete;
};
#endif

inline bool mkdirs(
    const std::filesystem::path& path,
    std::filesystem::perms permissions
) {
#ifdef BLACKNET_HAVE_SYS_STAT
    auto umasker = UMasker(permissions);
#else
    #warning "Filesystem permissions are not implemented for this OS"
#endif
    return std::filesystem::create_directories(path);
}

}

#endif

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

#ifndef BLACKNET_COMPAT_GETENTROPY_H
#define BLACKNET_COMPAT_GETENTROPY_H

#include "blacknet-config.h"

#include <span>
#include <stdexcept>
#include <system_error>

#ifdef BLACKNET_HAVE_GETENTROPY
#ifdef BLACKNET_HAVE_UNISTD
#include <unistd.h>
#endif
#ifdef BLACKNET_HAVE_SYS_RANDOM
#include <sys/random.h>
#endif
#include <limits.h>
#ifdef GETENTROPY_MAX
#define BLACKNET_GETENTROPY_CHUNK GETENTROPY_MAX
#else
#define BLACKNET_GETENTROPY_CHUNK 256
#endif
#elifdef BLACKNET_HAVE_NTSECAPI
#include <limits.h>
#include <windef.h>
#include <winnt.h>
#include <ntsecapi.h>
#define BLACKNET_GETENTROPY_CHUNK ULONG_MAX
#endif

namespace blacknet::compat {

inline void getentropy(const std::span<std::byte>& memory) {
    std::size_t offset = 0;
    std::size_t remain = memory.size();
    while (remain) {
        std::size_t process = std::min<std::size_t>(remain, BLACKNET_GETENTROPY_CHUNK);
#ifdef BLACKNET_HAVE_GETENTROPY
        int rc = ::getentropy(memory.data() + offset, process);
        if (rc == 0) {
            remain -= process;
            offset += process;
            continue;
        }
        throw std::system_error(errno, std::system_category(), "getentropy");
#elifdef BLACKNET_HAVE_NTSECAPI
        BOOLEAN rc = ::RtlGenRandom(memory.data() + offset, static_cast<ULONG>(process));
        if (rc != FALSE) {
            remain -= process;
            offset += process;
            continue;
        }
        throw std::runtime_error("RtlGenRandom failed");
#else
        #warning "Random device is not implemented for this OS"
        throw std::runtime_error("Random device is not implemented for this OS");
#endif
    }
}

}

#endif

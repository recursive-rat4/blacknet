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

#ifndef BLACKNET_COMPAT_FDATASYNC_H
#define BLACKNET_COMPAT_FDATASYNC_H

#include "blacknet-config.h"

#include <stdexcept>
#include <system_error>

#ifdef BLACKNET_HAVE_FDATASYNC
#include <unistd.h>
#elifdef BLACKNET_HAVE_FILEAPI
#include <errhandlingapi.h>
#include <fileapi.h>
#elifdef BLACKNET_HAVE_FULLFSYNC
#include <fcntl.h>
#endif

namespace blacknet::compat {

inline void fdatasync(auto fd) {
#ifdef BLACKNET_HAVE_FDATASYNC
    int rc;
    do {
        rc = ::fdatasync(fd);
    } while (rc != 0 && errno == EINTR);
    if (rc == 0)
        return;
    throw std::system_error(errno, std::system_category(), "fdatasync");
#elifdef BLACKNET_HAVE_FILEAPI
    BOOL rc = ::FlushFileBuffers(fd);
    if (rc != 0)
        return;
    throw std::system_error(GetLastError(), std::system_category(), "FlushFileBuffers");
#elifdef BLACKNET_HAVE_FULLFSYNC
    int rc;
    do {
        rc = ::fcntl(fd, F_FULLFSYNC, 1);
    } while (rc != 0 && errno == EINTR);
    if (rc == 0)
        return;
    throw std::system_error(errno, std::system_category(), "fcntl");
#else
    #warning "Durable disk operations are not implemented for this OS"
    throw std::runtime_error("Durable disk operations are not implemented for this OS");
#endif
}

}

#endif

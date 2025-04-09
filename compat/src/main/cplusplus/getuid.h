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

#ifndef BLACKNET_COMPAT_GETUID_H
#define BLACKNET_COMPAT_GETUID_H

#include "blacknet-config.h"

#ifdef BLACKNET_HAVE_UNISTD
#include <unistd.h>
#else
typedef int uid_t;
#endif

namespace blacknet::compat {

inline uid_t getuid() noexcept {
#ifdef BLACKNET_HAVE_UNISTD
    return ::getuid();
#else
    return -1;
#endif
}

}

#endif

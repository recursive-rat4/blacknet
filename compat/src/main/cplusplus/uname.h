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

#ifndef BLACKNET_COMPAT_UNAME_H
#define BLACKNET_COMPAT_UNAME_H

#include "blacknet-config.h"

#ifdef BLACKNET_HAVE_SYS_UTSNAME
#include <sys/utsname.h>
#endif

#include <string>
#include <system_error>
#include <tuple>

namespace blacknet::compat {

inline std::tuple<std::string, std::string, std::string> uname() {
#ifdef BLACKNET_HAVE_SYS_UTSNAME
    struct utsname name = {};
    int rc = uname(&name);
    if (rc == 0)
        return { name.sysname, name.release, name.machine };
    throw std::system_error(errno, std::system_category(), "uname");
#else
    #warning "OS version is not implemented for this OS"
    return { BLACKNET_HOST_SYSTEM, "unknown", BLACKNET_HOST_ARCH };
#endif
}

}

#endif

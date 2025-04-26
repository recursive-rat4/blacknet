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

#ifndef BLACKNET_NETWORK_BACKGROUND_H
#define BLACKNET_NETWORK_BACKGROUND_H

#include "blacknet-config.h"

#include <exception>

#include "logger.h"

namespace blacknet::network {

struct background_t {
    template<typename... Args>
    void operator () (std::exception_ptr eptr, Args...) {
        if (eptr == nullptr) return;
        try {
            std::rethrow_exception(eptr);
        } catch (const std::exception& e) {
            log::Logger logger{"background"};
#if FMT_VERSION >= 100000
            logger->error("{:t}", e);
#else
            logger->error("{}", e.what());
#endif
        } catch (...) {
            log::Logger logger{"background"};
            logger->error("Unknown exception");
        }
    }
};

// A completion token for background coroutines.
// Similar to `boost::asio::detached`.
constexpr background_t background;

}

#endif

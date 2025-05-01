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

#ifndef BLACKNET_TIME_SYSTEMCLOCK_H
#define BLACKNET_TIME_SYSTEMCLOCK_H

#include <chrono>

#include "milliseconds.h"

namespace blacknet::time {

// In milliseconds since UNIX epoch not counting leap seconds
struct SystemClock {
    static Milliseconds now() {
        auto unix_chrono = std::chrono::system_clock::now();
        auto milli_chrono = std::chrono::time_point_cast<std::chrono::milliseconds>(unix_chrono);
        return milli_chrono.time_since_epoch().count();
    }

    template<typename Chrono>
    static Milliseconds cast(const Chrono& chrono) {
        auto unix_chrono = std::chrono::clock_cast<std::chrono::system_clock>(chrono);
        auto milli_chrono = std::chrono::time_point_cast<std::chrono::milliseconds>(unix_chrono);
        return milli_chrono.time_since_epoch().count();
    }
};

}

#endif

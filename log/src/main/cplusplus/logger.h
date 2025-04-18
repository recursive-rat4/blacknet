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

#ifndef BLACKNET_LOG_LOGGER_H
#define BLACKNET_LOG_LOGGER_H

#include <memory>
#include <vector>
#include <spdlog/spdlog.h>

namespace blacknet::log {

class Logger {
    friend class LogManager;

    constexpr static spdlog::level::level_enum& filter_level() {
        static spdlog::level::level_enum instance;
        return instance;
    }

    constexpr static std::vector<spdlog::sink_ptr>& sinks() {
        static std::vector<spdlog::sink_ptr> instance;
        return instance;
    }

    std::unique_ptr<spdlog::logger> logger;
public:
    consteval Logger() : logger() {}

    Logger(const char* name) : logger(
        std::make_unique<spdlog::logger>(
            name,
            sinks().cbegin(),
            sinks().cend()
        )
    ) {
        logger->set_pattern("%+", spdlog::pattern_time_type::utc);
        logger->set_level(filter_level());
        logger->flush_on(spdlog::level::err);
    }

    explicit constexpr operator bool () const noexcept {
        return logger.operator bool ();
    }

    constexpr spdlog::logger* operator -> () noexcept {
        return logger.operator -> ();
    }

    constexpr void reset() noexcept {
        logger.reset();
    }
};

}

#endif

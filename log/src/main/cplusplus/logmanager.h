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

#ifndef BLACKNET_LOG_LOGMANAGER_H
#define BLACKNET_LOG_LOGMANAGER_H

#include <cstdlib>
#include <iostream>
#include <memory>
#include <spdlog/spdlog.h>
#include <spdlog/sinks/rotating_file_sink.h>
#include <spdlog/sinks/stdout_color_sinks.h>

#include "logger.h"
#include "xdgdirectories.h"

namespace blacknet::log {

class LogManager {
    constexpr static Logger& logger() {
        static Logger instance;
        return instance;
    }

    static void sinkConsole() {
        Logger::sinks().push_back(std::make_shared<spdlog::sinks::stdout_color_sink_mt>(
            spdlog::color_mode::automatic
        ));
    }

    static void sinkFile() {
        Logger::sinks().push_back(std::make_shared<spdlog::sinks::rotating_file_sink_mt>(
            compat::stateDir() / "debug.log",
            5000000,
            2,
            false
        ));
    }

    static void shutDown() {
        if (logger().logger) {
            logger().logger->info("Shutting down logging");
            logger().logger.reset();
        } else {
            std::cerr << "LogManager not yet initialized or already shutten" << std::endl;
        }
        Logger::sinks().clear();
        spdlog::shutdown();
    }
public:
    enum Regime {
        Daemon,
        Desktop,
        UnitTest,
    };

    LogManager(Regime regime) {
        switch (regime) {
            case Regime::Daemon:
                Logger::filter_level() = spdlog::level::info;
                sinkConsole();
                sinkFile();
                break;
            case Regime::Desktop:
                Logger::filter_level() = spdlog::level::info;
                sinkFile();
                break;
            case Regime::UnitTest:
                Logger::filter_level() = spdlog::level::err;
                sinkConsole();
                break;
        }

        const char* env_level_ptr = std::getenv("BLACKNET_LOGLEVEL");
        if (env_level_ptr != nullptr) {
            Logger::filter_level() = spdlog::level::from_str(env_level_ptr);
        }

        logger() = Logger("LogManager");
        logger()->info("Initialized logging");
    }
    ~LogManager() {
        shutDown();
    }

    constexpr LogManager(const LogManager&) = delete;
    constexpr LogManager(LogManager&&) = delete;
    constexpr LogManager& operator = (const LogManager&) = delete;
    constexpr LogManager& operator = (LogManager&&) = delete;
};

}

#endif

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

#include "blacknet-config.h"

#include <cstring>
#include <exception>
#include <boost/asio/thread_pool.hpp>
#include <fmt/format.h>
#include <fmt/std.h>

#include "node.h"

using blacknet::log::LogManager;
using blacknet::network::Node;

int main(int argc, char* argv[]) {
    if (argc == 2 && std::strcmp(argv[1], "--version") == 0) {
        fmt::println("Blacknet Daemon {}", BLACKNET_VERSION_STRING);
        return 0;
    }
    try {
        Node node(LogManager::Regime::Daemon);
        boost::asio::thread_pool thread_pool;
        node.co_spawn(thread_pool);
        thread_pool.wait();
    } catch (const std::exception& e) {
#if FMT_VERSION >= 100000
        fmt::println(stderr, "{:t}", e);
#else
        fmt::println(stderr, "{}", e.what());
#endif
        return 1;
    }
}

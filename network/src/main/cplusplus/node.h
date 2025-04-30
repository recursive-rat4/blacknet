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

#ifndef BLACKNET_NETWORK_NODE_H
#define BLACKNET_NETWORK_NODE_H

#include "blacknet-config.h"

#include <atomic>
#include <filesystem>
#include <thread>
#include <boost/asio/io_context.hpp>

#include "concurrent_vector.h"
#include "connection.h"
#include "getuid.h"
#include "logmanager.h"
#include "mode.h"
#include "networksettings.h"
#include "peertable.h"
#include "router.h"
#include "uname.h"
#include "xdgdirectories.h"

namespace blacknet::network {

class Node {
    std::atomic<connection_id> next_peer_id{1};
    concurrent_vector<connection_ptr> connections;

    compat::ModeManager mode_manager;
    compat::DirManager dir_manager;
    log::LogManager log_manager;
    NetworkSettings settings;
    PeerTable peer_table;
    Router router;
public:
    Node(log::LogManager::Regime regime) :
        mode_manager(),
        dir_manager(),
        log_manager(regime),
        settings(),
        peer_table(settings),
        router(settings, peer_table) {}

    void co_spawn(boost::asio::io_context& io_context) {
        auto [os_name, os_version, os_machine] = compat::uname();

        log::Logger logger("Node");
        logger->info("Starting up {} node {}", compat::mode()->agent_name(), BLACKNET_VERSION_STRING);
        logger->info("CPU: {} cores {}", std::thread::hardware_concurrency(), os_machine);
        logger->info("OS: {} version {}", os_name, os_version);
        logger->info("Using config directory {}", std::filesystem::absolute(compat::configDir()));
        logger->info("Using data directory {}", std::filesystem::absolute(compat::dataDir()));
        logger->info("Using state directory {}", std::filesystem::absolute(compat::stateDir()));

        if (compat::getuid() == 0)
            logger->warn("Running as root");
#if 0
        if (compat::getsid() == "S-1-5-18")
            logger->warn("Running as SYSTEM");
#endif

        peer_table.co_spawn(io_context);
        router.co_spawn(io_context);
    }
};

}

#endif

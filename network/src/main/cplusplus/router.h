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

#ifndef BLACKNET_NETWORK_ROUTER_H
#define BLACKNET_NETWORK_ROUTER_H

#include <chrono>
#include <boost/asio/awaitable.hpp>
#include <boost/asio/co_spawn.hpp>
#include <boost/asio/detached.hpp>
#include <boost/asio/steady_timer.hpp>
#include <boost/asio/thread_pool.hpp>
#include <boost/asio/use_awaitable.hpp>

#include "i2psam.h"
#include "logger.h"

namespace blacknet::network {

class Router {
    constexpr static const std::chrono::milliseconds init_timeout = std::chrono::minutes{1};
    constexpr static const std::chrono::milliseconds max_timeout = std::chrono::hours{2};

    log::Logger logger{"Router"};
    i2p::SAM i2p_sam;

    boost::asio::awaitable<void> listen_i2p(boost::asio::thread_pool& thread_pool) {
        auto timeout = init_timeout;
        while (true) {
            try {
                i2p::session_ptr i2p_session = co_await i2p_sam.create_session(thread_pool);
                logger->info("Created I2P session {} listening on {}", i2p_session->id, "TODO");
                //TODO localAddress
                //TODO loop
                co_await i2p_session->accept(thread_pool);
                //TODO remoteAddress
                logger->info("Closing I2P session {}", i2p_session->id);
                timeout = init_timeout;
            } catch (const i2p::Exception& e) {
                logger->info("I2P: {}", e.what());
            } catch (const boost::system::system_error& e) {
                logger->debug("Can't connect to I2P SAM: {}", e.what());
            }
            auto now = std::chrono::steady_clock::now();
            boost::asio::steady_timer timer(thread_pool, now + timeout);
            co_await timer.async_wait(boost::asio::use_awaitable);
            timeout = std::min(timeout * 2, max_timeout);
        }
    }
public:
    Router()
        : i2p_sam()
    {
    }

    void co_spawn(boost::asio::thread_pool& thread_pool) {
        //TODO settings
        boost::asio::co_spawn(thread_pool, listen_i2p(thread_pool), boost::asio::detached);
    }
};

}

#endif

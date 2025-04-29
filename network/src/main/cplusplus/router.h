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
#include <boost/asio/io_context.hpp>
#include <boost/asio/steady_timer.hpp>
#include <boost/asio/use_awaitable.hpp>
#include <boost/asio/ip/tcp.hpp>
#include <boost/asio/ip/v6_only.hpp>
#include <stdexcept>

#include "background.h"
#include "endpoint.h"
#include "i2psam.h"
#include "logger.h"
#include "networksettings.h"

namespace blacknet::network {

class Router {
    constexpr static const std::chrono::milliseconds init_timeout = std::chrono::minutes{1};
    constexpr static const std::chrono::milliseconds max_timeout = std::chrono::hours{2};

    log::Logger logger{"Router"};
    const NetworkSettings& settings;
    i2p::SAM i2p_sam;

    void add_listener(endpoint_ptr endpoint) {
        logger->info("Listening on {}", endpoint->to_log(settings.logips));
        //TODO set
    }
    void remove_listener(endpoint_ptr endpoint) {
        logger->info("Lost binding to {}", endpoint->to_log(settings.logips));
        //TODO set
    }

    boost::asio::awaitable<void> listen_ip(boost::asio::io_context& io_context) {
        endpoint_ptr endpoint;
        if (settings.ipv6) {
            endpoint = endpoint::IPv6::any(settings.port);
        } else if (settings.ipv4) {
            endpoint = endpoint::IPv4::any(settings.port);
        } else {
            throw std::logic_error("Both IPv4 and IPv6 are disabled");
        }
        auto timeout = init_timeout;
        while (true) {
            bool added = false;
            try {
                auto boost_endpoint = endpoint->to_boost();
                boost::asio::ip::tcp::acceptor acceptor(io_context);
                acceptor.open(boost_endpoint.protocol());
                if (settings.ipv6)
                    acceptor.set_option(boost::asio::ip::v6_only(!settings.ipv4));
                acceptor.set_option(boost::asio::ip::tcp::acceptor::reuse_address(true));
                acceptor.bind(boost_endpoint);
                acceptor.listen(settings.max_incoming_connections);
                add_listener(endpoint);
                added = true;
                //TODO localAddress
                //TODO loop
                co_await acceptor.async_accept(io_context, boost::asio::use_awaitable);
                //TODO remoteAddress
            } catch (const boost::system::system_error& e) {
                logger->warn("{}", e.what());
            }
            if (added) {
                remove_listener(endpoint);
            }

            auto now = std::chrono::steady_clock::now();
            boost::asio::steady_timer timer(io_context, now + timeout);
            co_await timer.async_wait(boost::asio::use_awaitable);
            timeout = std::min(timeout * 2, max_timeout);
        }
    }

    boost::asio::awaitable<void> listen_tor(boost::asio::io_context& io_context) {
        //TODO
        co_return;
    }

    boost::asio::awaitable<void> listen_i2p(boost::asio::io_context& io_context) {
        endpoint_ptr endpoint;
        auto timeout = init_timeout;
        while (true) {
            try {
                i2p::session_ptr i2p_session = co_await i2p_sam.create_session(io_context);
                endpoint = i2p_session->local_endpoint;
                add_listener(endpoint);
                //TODO localAddress
                //TODO loop
                co_await i2p_session->accept(io_context);
                //TODO remoteAddress
                logger->info("Closing I2P session {}", i2p_session->id);
                timeout = init_timeout;
            } catch (const i2p::Exception& e) {
                logger->info("I2P: {}", e.what());
            } catch (const boost::system::system_error& e) {
                logger->debug("Can't connect to I2P SAM: {}", e.what());
            }
            if (endpoint) {
                remove_listener(endpoint);
                endpoint.reset();
            }

            auto now = std::chrono::steady_clock::now();
            boost::asio::steady_timer timer(io_context, now + timeout);
            co_await timer.async_wait(boost::asio::use_awaitable);
            timeout = std::min(timeout * 2, max_timeout);
        }
    }
public:
    Router(const NetworkSettings& settings) :
        settings(settings),
        i2p_sam(settings) {}

    void co_spawn(boost::asio::io_context& io_context) {
        if (settings.ipv6 || settings.ipv4)
            boost::asio::co_spawn(io_context, listen_ip(io_context), background);
        if (settings.tor)
            boost::asio::co_spawn(io_context, listen_tor(io_context), background);
        if (settings.i2p)
            boost::asio::co_spawn(io_context, listen_i2p(io_context), background);
    }
};

}

#endif

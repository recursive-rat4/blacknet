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

#ifndef BLACKNET_NETWORK_TORCONTROLLER_H
#define BLACKNET_NETWORK_TORCONTROLLER_H

#include "blacknet-config.h"

#include <exception>
#include <filesystem>
#include <fstream>
#include <memory>
#include <utility>
#include <boost/asio/awaitable.hpp>
#include <boost/asio/buffer.hpp>
#include <boost/asio/co_spawn.hpp>
#include <boost/asio/io_context.hpp>
#include <boost/asio/read_until.hpp>
#include <boost/asio/streambuf.hpp>
#include <boost/asio/use_awaitable.hpp>
#include <boost/asio/ip/tcp.hpp>
#include <fmt/format.h>
#include <fmt/std.h>

#include "background.h"
#include "endpoint.h"
#include "file.h"
#include "logger.h"
#include "networksettings.h"
#include "systemclock.h"
#include "xdgdirectories.h"

namespace blacknet::network {
// https://spec.torproject.org/control-spec/
namespace tor {
class Exception : public std::exception {
    std::string message;
public:
    Exception(const std::string_view& message) : message(message) {}
    virtual const char* what() const noexcept override {
        return message.c_str();
    }
};

class Connection;
using connection_ptr = std::unique_ptr<Connection>;
class Connection {
    log::Logger logger{"tor::Connection"};
    boost::asio::ip::tcp::socket socket;
    boost::asio::streambuf read_buf;
    constexpr static const std::size_t max_line{32768};
public:
    Connection(boost::asio::ip::tcp::socket&& socket)
        : socket(std::move(socket)), read_buf(max_line) {}

    boost::asio::awaitable<std::string> read() {
        std::size_t n = co_await boost::asio::async_read_until(socket, read_buf, '\n', boost::asio::use_awaitable);
        auto begin = boost::asio::buffers_begin(read_buf.data());
        std::string raw(begin, begin + n);
        read_buf.consume(n);
        logger->trace("<- {:?}", raw);
        co_return raw;
    }

    boost::asio::awaitable<void> write(const std::string_view& message) {
        logger->trace("-> {:?}", message);
        co_await socket.async_send(boost::asio::buffer(message), boost::asio::use_awaitable);
    }

    boost::asio::awaitable<std::string> request(const std::string_view& request) {
        co_await write(request);
        std::string raw = co_await read();
        co_return raw;
    }

    static boost::asio::awaitable<connection_ptr> connect(
        const boost::asio::ip::tcp::endpoint& tc_endpoint,
        boost::asio::io_context& io_context
    ) {
        boost::asio::ip::tcp::socket socket(io_context);
        co_await socket.async_connect(tc_endpoint, boost::asio::use_awaitable);
        auto connection = std::make_unique<Connection>(std::move(socket));
        co_return connection;
    }

    boost::asio::awaitable<void> authenticate(
    ) {
        //TODO cookie, password
        auto request = "AUTHENTICATE\r\n";
        auto reply_line = co_await this->request(request);
        if (reply_line == "250 OK\r\n")
            co_return;
        throw Exception(fmt::format("Unknown Tor reply line {:?}", reply_line));
    }

    boost::asio::awaitable<std::pair<std::string, std::string>> add_onion(
        const std::string_view& private_key,
        uint16_t tor_port
    ) {
        auto request = fmt::format("ADD_ONION {} Port={}\r\n", private_key, tor_port);
        co_await write(request);
        std::string service_id;
        std::string new_key;
        while (true) {
            auto reply_line = co_await read();
            if (reply_line == "250 OK\r\n")
                break;
            else if (reply_line.starts_with("250-ServiceID=") && reply_line.ends_with("\r\n"))
                service_id = reply_line.substr(14, reply_line.length() - 14 - 2);
            else if (reply_line.starts_with("250-PrivateKey=") && reply_line.ends_with("\r\n"))
                new_key = reply_line.substr(15, reply_line.length() - 15 - 2);
            else if (!reply_line.starts_with("250-"))
                throw Exception(fmt::format("Unknown Tor reply line {:?}", reply_line));
        }
        co_return std::make_pair(std::move(service_id), std::move(new_key));
    }
};

class Session {
    log::Logger logger{"tor::Session"};
public:
    const endpoint_ptr local_endpoint;
private:
    connection_ptr connection;
    boost::asio::ip::tcp::endpoint tc_endpoint;
public:
    Session(
        endpoint_ptr&& local_endpoint,
        connection_ptr&& connection,
        const boost::asio::ip::tcp::endpoint& tc_endpoint
    ) :
        local_endpoint(std::move(local_endpoint)),
        connection(std::move(connection)),
        tc_endpoint(tc_endpoint) {}

    boost::asio::awaitable<std::string> request(const std::string_view& request) {
        auto answer = co_await connection->request(request);
        co_return answer;
    }

    boost::asio::awaitable<void> loop() {
        try {
            while (true) {
                std::string reply_line = co_await connection->read();
                throw Exception(fmt::format("Unknown Tor reply line {:?}", reply_line));
            }
        } catch (const Exception& e) {
            logger->warn("{}", e.what());
        } catch (const boost::system::system_error&) {
            // Socket closed
        } catch (const std::exception& e) {
#if FMT_VERSION >= 100000
            logger->error("{:t}", e);
#else
            logger->error("{}", e.what());
#endif
        }
    }
};
using session_ptr = std::unique_ptr<Session>;

class Controller {
    constexpr static std::string_view file_name{"privateKey.tor"};
    constexpr static std::string_view transient_key{"NEW:ED25519-V3"};

    log::Logger logger{"tor::Controller"};
    const NetworkSettings& settings;

    std::string private_key{transient_key};
    boost::asio::ip::tcp::endpoint tc_endpoint;

    void save_private_key(const std::string_view& new_key) {
        private_key = new_key;
        logger->info("Saving Tor private key");
        io::file::replace<std::endian::big>(compat::dataDir(), file_name, [&](auto& os) {
            os.write_str(new_key);
        });
    }
public:
    Controller(const NetworkSettings& settings)
        : settings(settings)
    {
    }

    boost::asio::awaitable<session_ptr> create_session(boost::asio::io_context& io_context) {
        auto connection = co_await Connection::connect(tc_endpoint, io_context);
        co_await connection->authenticate();
        //XXX port
        auto [service_id, new_key] = co_await connection->add_onion(private_key, settings.port);
        auto local_endpoint = endpoint::TORv3::parse(service_id + ".onion", settings.port);
        if (private_key.starts_with("NEW:")) {
            if (!new_key.empty())
                save_private_key(new_key);
            else
                throw Exception("Failed to get new private key");
        }
        session_ptr session = std::make_unique<Session>(
            std::move(local_endpoint),
            std::move(connection),
            tc_endpoint
        );
        logger->info("Created session");
        co_return session;
    }

    void co_spawn(
        [[maybe_unused]]
        boost::asio::io_context& io_context
    ) {
        tc_endpoint = boost::asio::ip::tcp::endpoint(
            boost::asio::ip::make_address(settings.torcontrolhost),
            settings.torcontrolport
        );

        try {
            auto path = compat::dataDir() / file_name;
            std::size_t size = std::filesystem::file_size(path);
            std::ifstream ifs(path);
            std::string buf(size, '\0');
            ifs.read(buf.data(), size);
            if (buf.starts_with("ED25519-V3:")) {
                private_key = std::move(buf);
            } else {
                auto new_name = fmt::format("privateKey.{}.tor", time::SystemClock::now());
                std::filesystem::rename(path, compat::dataDir() / new_name);
                logger->info("Renamed private key file to {}", new_name);
            }
        } catch (const std::exception& e) {
#if FMT_VERSION >= 100000
            logger->debug("{:t}", e);
#else
            logger->debug("{}", e.what());
#endif
        }
    }
};
}

}

#endif

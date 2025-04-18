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

#ifndef BLACKNET_NETWORK_I2PSAM_H
#define BLACKNET_NETWORK_I2PSAM_H

#include "blacknet-config.h"

#include <exception>
#include <filesystem>
#include <fstream>
#include <memory>
#include <boost/asio/awaitable.hpp>
#include <boost/asio/buffer.hpp>
#include <boost/asio/co_spawn.hpp>
#include <boost/asio/detached.hpp>
#include <boost/asio/thread_pool.hpp>
#include <boost/asio/ip/tcp.hpp>
#include <boost/asio/read_until.hpp>
#include <boost/asio/streambuf.hpp>
#include <boost/asio/use_awaitable.hpp>
#include <fmt/format.h>
#include <fmt/std.h>

#include "fastrng.h"
#include "file.h"
#include "logger.h"
#include "xdgdirectories.h"

namespace blacknet::network {
// https://geti2p.net/en/docs/api/samv3
namespace i2p {
class Exception : public std::exception {
    std::string message;
public:
    Exception(const std::string_view& message) : message(message) {}
    virtual const char* what() const noexcept override {
        return message.c_str();
    }
};

class Answer {
    const std::string raw;
public:
    constexpr Answer(std::string&& raw) : raw(std::move(raw)) {}

    constexpr bool operator == (const Answer&) const = default;

    constexpr std::optional<std::string_view> get(
        const std::string_view& key
    ) const {
        auto key_pattern = fmt::format(" {}=", key);
        std::size_t i = raw.find(key_pattern);
        if (i == std::string_view::npos)
            return std::nullopt;
        std::size_t value_start = i + key_pattern.length();
        if (value_start == raw.length())
            return std::string_view();
        if (raw[value_start] == '"') {
            std::size_t value_end = raw.find('"', value_start + 1);
            if (value_end == std::string_view::npos)
                return std::nullopt;
            return std::string_view(raw.data() + value_start + 1, raw.data() + value_end);
        }
        std::size_t value_end = raw.find(' ', value_start);
        if (value_end == std::string_view::npos)
            return std::string_view(raw.data() + value_start, raw.data() + raw.length() - 1);
        return std::string_view(raw.data() + value_start, raw.data() + value_end);
    }

    constexpr void ok() const {
        if (auto maybeResult = get("RESULT")) {
            const auto& result = *maybeResult;
            if (result.empty()) {
                throw Exception("Empty RESULT");
            } else if (result != "OK") {
                if (auto maybeMessage = get("MESSAGE")) {
                    const auto& message = *maybeMessage;
                    if (message.empty())
                        throw Exception(result);
                    else
                        throw Exception(fmt::format("{} {}", result, message));
                } else {
                    throw Exception(result);
                }
            }
        } else {
            throw Exception("No RESULT");
        }
    }
};

class Connection {
    log::Logger logger{"i2p::Connection"};
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

    boost::asio::awaitable<Answer> request(const std::string_view& request) {
        co_await write(request);
        std::string raw = co_await read();
        Answer answer(std::move(raw));
        answer.ok();
        co_return answer;
    }

    boost::asio::awaitable<void> connect(const boost::asio::ip::tcp::endpoint& endpoint) {
        co_await socket.async_connect(endpoint, boost::asio::use_awaitable);
        co_await request("HELLO VERSION MIN=3.2 MAX=3.3\n");
    }

    boost::asio::awaitable<std::string> lookup(const std::string_view& name) {
        auto request = fmt::format("NAMING LOOKUP NAME={}\n", name);
        auto answer = co_await this->request(request);
        co_return answer.get("VALUE").value();
    }
};
using connection_ptr = std::unique_ptr<Connection>;

class Session {
    log::Logger logger{"i2p::Session"};
public:
    const std::string id;
private:
    connection_ptr connection;
    boost::asio::ip::tcp::endpoint sam_endpoint;

    boost::asio::awaitable<connection_ptr> connect_to_sam(boost::asio::thread_pool& thread_pool) {
        boost::asio::ip::tcp::socket socket(thread_pool);
        connection_ptr connection = std::make_unique<Connection>(std::move(socket));
        co_await connection->connect(sam_endpoint);
        co_return connection;
    }

    boost::asio::awaitable<void> loop() {
        try {
            while (true) {
                std::string message = co_await connection->read();

                if (message.starts_with("PING")) {
                    message[1] = 'O';
                    co_await connection->write(message);
                } else if (message.starts_with("PONG")) {
                    logger->warn("Unexpected PONG message");
                } else {
                    Answer answer(std::move(message));
                    answer.ok();
                }
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
public:
    Session(
        std::string&& id,
        const boost::asio::ip::tcp::endpoint& sam_endpoint
    ) :
        id(std::move(id)),
        sam_endpoint(sam_endpoint) {}

    boost::asio::awaitable<Answer> create(
        const std::string_view& private_key,
        boost::asio::thread_pool& thread_pool
    ) {
        connection = co_await connect_to_sam(thread_pool);
        // i2cp.leaseSetEncType 0 for connectivity with `Node::PROTOCOL_VERSION` <= 15
        auto request = fmt::format("SESSION CREATE STYLE=STREAM ID={} DESTINATION={} SIGNATURE_TYPE=EdDSA_SHA512_Ed25519 i2cp.leaseSetEncType=4,0\n", id, private_key);
        auto answer = co_await connection->request(request);
        co_return answer;
    }

    boost::asio::awaitable<void> accept(boost::asio::thread_pool& thread_pool) {
        connection_ptr connection = co_await connect_to_sam(thread_pool);
        auto request = fmt::format("STREAM ACCEPT ID={}\n", id);
        co_await connection->request(request);
        std::string message = co_await connection->read();
        if (message.starts_with("STREAM STATUS")) {
            Answer answer(std::move(message));
            answer.ok();
            co_return;
        }
        //TODO accept
    }

    boost::asio::awaitable<Answer> request(const std::string_view& request) {
        auto answer = co_await connection->request(request);
        co_return answer;
    }

    boost::asio::awaitable<std::string> lookup(const std::string_view& name) {
        auto value = co_await connection->lookup(name);
        co_return value;
    }

    void co_spawn(boost::asio::thread_pool& thread_pool) {
        boost::asio::co_spawn(thread_pool, loop(), boost::asio::detached);
    }
};
using session_ptr = std::unique_ptr<Session>;

class SAM {
    constexpr static std::string_view file_name{"privateKey.i2p"};
    constexpr static std::string_view transient_key{"TRANSIENT"};

    log::Logger logger{"i2p::SAM"};
    crypto::FastRNG rng;

    std::string private_key{transient_key};
    boost::asio::ip::tcp::endpoint sam_endpoint;

    std::string generate_id() {
        constexpr std::size_t size = 8;
        constexpr std::string_view alphabet{"ABCDEFGHIJKLMNOPQRSTUVWXYZ"};
        std::uniform_int_distribution<std::size_t> ud(0, alphabet.length() - 1);
        std::string id(size, '\0');
        for (auto& c : id)
            c = alphabet[ud(rng)];
        return id;
    }

    void save_private_key(const std::string_view& destination) {
        private_key = destination;
        logger->info("Saving I2P private key");
        io::file::replace(rng, compat::dataDir(), file_name, [&](auto& os) {
            os.write(destination.data(), destination.size());
        });
    }
public:
    SAM()
    {
        //TODO settings
        std::string i2psamhost{"127.0.0.1"};
        uint16_t i2psamport{7656};

        sam_endpoint = boost::asio::ip::tcp::endpoint(
            boost::asio::ip::make_address(i2psamhost), i2psamport
        );

        try {
            auto path = compat::dataDir() / file_name;
            auto timestamp = io::file::last_write_time(path);
            if (timestamp != 0 && timestamp < 1550000000000) {
                auto new_name = fmt::format("privateKey.{}.i2p", timestamp);
                std::filesystem::rename(path, compat::dataDir() / new_name);
                logger->info("Renamed private key file to {}", new_name);
            } else {
                std::size_t size = std::filesystem::file_size(path);
                std::ifstream ifs(path);
                std::string buf(size, '\0');
                ifs.read(buf.data(), size);
                private_key = std::move(buf);
            }
        } catch (const std::exception& e) {
#if FMT_VERSION >= 100000
            logger->debug("{:t}", e);
#else
            logger->debug("{}", e.what());
#endif
        }
    }

    boost::asio::awaitable<session_ptr> create_session(boost::asio::thread_pool& thread_pool) {
        session_ptr session = std::make_unique<Session>(generate_id(), sam_endpoint);
        auto answer = co_await session->create(private_key, thread_pool);
        auto destination = co_await session->lookup("ME");
        //TODO localAddress
        if (private_key == transient_key)
            save_private_key(answer.get("DESTINATION").value());
        session->co_spawn(thread_pool);
        co_return session;
    }
};
}

}

#endif

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

#include <boost/asio/awaitable.hpp>
#include <boost/asio/buffer.hpp>
#include <boost/asio/io_context.hpp>
#include <boost/asio/ip/tcp.hpp>
#include <boost/asio/read_until.hpp>
#include <boost/asio/streambuf.hpp>
#include <boost/asio/use_awaitable.hpp>
#include <fmt/format.h>

#include "logger.h"

namespace blacknet::network {

class I2PSAMException : public std::exception {
    std::string message;
public:
    I2PSAMException(const std::string_view& message) : message(message) {}
    virtual const char* what() const noexcept override {
        return message.c_str();
    }
};

// https://geti2p.net/en/docs/api/samv3
class I2PSAM {
    boost::asio::io_context& io_context;

    //TODO settings
    std::string i2psamhost{"127.0.0.1"};
    uint16_t i2psamport{7656};
public:
    I2PSAM(boost::asio::io_context& io_context) : io_context(io_context) {}

    class Connection {
        log::Logger logger{"I2PSAM"};
        boost::asio::ip::tcp::socket socket;
        boost::asio::streambuf read_buf;
        constexpr static const std::size_t max_line{32768};

        static std::optional<std::string_view> value(
            const std::string_view& answer,
            const std::string_view& key
        ) {
            auto key_pattern = fmt::format(" {}=", key);
            std::size_t i = answer.find(key_pattern);
            if (i == std::string_view::npos)
                return std::nullopt;
            std::size_t value_start = i + key_pattern.length();
            if (value_start == answer.length())
                return std::string_view();
            if (answer[value_start] == '"') {
                std::size_t value_end = answer.find('"', value_start + 1);
                if (value_end == std::string_view::npos)
                    return std::nullopt;
                return std::string_view(answer.data() + value_start + 1, answer.data() + value_end);
            }
            std::size_t value_end = answer.find(' ', value_start);
            if (value_end == std::string_view::npos)
                return answer.substr(value_start);
            return std::string_view(answer.data() + value_start, answer.data() + value_end);
        }

        static void ok(const std::string_view& answer) {
            if (auto maybeResult = value(answer, "RESULT")) {
                const auto& result = *maybeResult;
                if (result.empty()) {
                    throw I2PSAMException("Empty RESULT");
                } else if (result != "OK") {
                    if (auto maybeMessage = value(answer, "MESSAGE")) {
                        const auto& message = *maybeMessage;
                        if (message.empty())
                            throw I2PSAMException(result);
                        else
                            throw I2PSAMException(fmt::format("{} {}", result, message));
                    } else {
                        throw I2PSAMException(result);
                    }
                }
            } else {
                throw I2PSAMException("No RESULT");
            }
        }

        boost::asio::awaitable<std::string> request(const std::string& request) {
            logger->debug("-> {:?}", request);
            co_await socket.async_send(boost::asio::buffer(request), boost::asio::use_awaitable);

            std::size_t n = co_await boost::asio::async_read_until(socket, read_buf, '\n', boost::asio::use_awaitable);
            auto begin = boost::asio::buffers_begin(read_buf.data());
            std::string answer(begin, begin + n);
            logger->debug("<- {:?}", answer);

            ok(answer);
            co_return answer;
        }
    public:
        Connection(boost::asio::io_context& io_context)
            : socket(io_context), read_buf(max_line) {}

        boost::asio::awaitable<void> connect(const std::string& host, uint16_t port) {
            boost::asio::ip::tcp::endpoint endpoint(
                boost::asio::ip::make_address(host), port
            );
            co_await socket.async_connect(endpoint, boost::asio::use_awaitable);
            co_await request("HELLO VERSION MIN=3.2 MAX=3.3\n");
        }
    };

    boost::asio::awaitable<std::unique_ptr<Connection>> connectToSAM() {
        std::unique_ptr<Connection> connection = std::make_unique<Connection>(io_context);
        co_await connection->connect(i2psamhost, i2psamport);
        co_return connection;
    }
};

}

#endif

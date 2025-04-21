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

#ifndef BLACKNET_NETWORK_ENDPOINT_H
#define BLACKNET_NETWORK_ENDPOINT_H

#include <array>
#include <string>
#include <string_view>
#include <boost/asio/ip/tcp.hpp>
#include <fmt/format.h>

#include "base32.h"

namespace blacknet::network {

namespace endpoint {

class Exception : public std::exception {
    std::string message;
public:
    Exception(const char* message) : message(message) {}
    virtual const char* what() const noexcept override {
        return message.c_str();
    }
};

class I2P {
    using base32 = codec::base32::codec<codec::base32::i2p>;
    constexpr static const std::string_view suffix{".b32.i2p"};

    const uint16_t port;
    const std::array<std::byte, 32> address;
public:
    constexpr I2P(uint16_t port, const std::array<std::byte, 32>& address)
        : port(port), address(address) {}

    constexpr bool operator == (const I2P&) const = default;

    constexpr bool is_local() const {
        return false;
    }

    constexpr bool is_private() const {
        return false;
    }

    constexpr boost::asio::ip::tcp::endpoint to_boost() const {
        throw Exception("Can't convert I2P endpoint to TCP/IP");
    }

    constexpr std::string to_log(bool detail) const {
        if (detail)
            return fmt::format("{}{}:{}", base32::encode(address), suffix, port);
        else
            return "I2P endpoint";
    }
};

}

}

#endif

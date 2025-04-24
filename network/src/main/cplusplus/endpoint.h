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

#include <algorithm>
#include <array>
#include <memory>
#include <string>
#include <string_view>
#include <boost/asio/ip/address_v4.hpp>
#include <boost/asio/ip/address_v6.hpp>
#include <boost/asio/ip/tcp.hpp>
#include <fmt/format.h>

#include "base32.h"
#include "byte.h"

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

class Endpoint {
public:
    virtual ~Endpoint() noexcept = default;

    virtual bool is_permissionless() const = 0;
    virtual bool is_local() const = 0;
    virtual bool is_private() const = 0;
    virtual boost::asio::ip::tcp::endpoint to_boost() const = 0;
    virtual std::string to_host() const = 0;
    virtual std::string to_log(bool detail) const = 0;
};
using endpoint_ptr = std::unique_ptr<Endpoint>;

class IPv4 final : public Endpoint {
    constexpr static const std::array<std::byte, 4> any_address =
        compat::byte::arrayU<4>({ 0, 0, 0, 0});
    constexpr static const std::array<std::byte, 4> loopback_address =
        compat::byte::arrayU<4>({ 127, 0, 0, 1 });

    const uint16_t port;
    const std::array<std::byte, 4> address;

    boost::asio::ip::address_v4 to_addr() const {
        const std::array<unsigned char, 4>& chars = reinterpret_cast<const std::array<unsigned char, 4>&>(address);
        return boost::asio::ip::address_v4(chars);
    }
public:
    constexpr IPv4(uint16_t port, const std::array<std::byte, 4>& address)
        : port(port), address(address) {}

    constexpr bool operator == (const IPv4&) const = default;

    bool is_permissionless() const override {
        return false;
    }

    bool is_local() const override {
        // 0.0.0.0 – 0.255.255.255
        if (address[0] == std::byte{0}) return true;
        // 127.0.0.0 – 127.255.255.255
        if (address[0] == std::byte{127}) return true;
        // 169.254.0.0 – 169.254.255.255
        if (address[0] == std::byte{169} && address[1] == std::byte{254}) return true;

        return false;
    }

    bool is_private() const override {
        // 10.0.0.0 – 10.255.255.255
        if (address[0] == std::byte{10}) return true;
        // 100.64.0.0 – 100.127.255.255
        if (address[0] == std::byte{100} && address[1] >= std::byte{64} && address[1] <= std::byte{127}) return true;
        // 172.16.0.0 – 172.31.255.255
        if (address[0] == std::byte{172} && address[1] >= std::byte{16} && address[1] <= std::byte{31}) return true;
        // 192.0.0.0 – 192.0.0.255
        if (address[0] == std::byte{192} && address[1] == std::byte{0} && address[2] == std::byte{0}) return true;
        // 192.168.0.0 – 192.168.255.255
        if (address[0] == std::byte{192} && address[1] == std::byte{168}) return true;
        // 198.18.0.0 – 198.19.255.255
        if (address[0] == std::byte{198} && address[1] >= std::byte{18} && address[1] <= std::byte{19}) return true;

        return false;
    }

    boost::asio::ip::tcp::endpoint to_boost() const override {
        return { to_addr(), port };
    }

    std::string to_host() const override {
        return to_addr().to_string();
    }

    std::string to_log(bool detail) const override {
        if (detail)
            return fmt::format("{}:{}", to_host(), port);
        else if (is_local())
            return "IPv4 local";
        else
            return "IPv4 endpoint";
    }

    static endpoint_ptr parse(const std::string_view& string, uint16_t port) {
        try {
            auto chars = boost::asio::ip::make_address_v4(string).to_bytes();
            return std::make_unique<IPv4>(
                port,
                reinterpret_cast<const std::array<std::byte, 4>&>(chars)
            );
        } catch (const boost::system::system_error& e) {
            return nullptr;
        }
    }

    static endpoint_ptr any(uint16_t port) {
        return std::make_unique<IPv4>(port, any_address);
    }

    static endpoint_ptr loopback(uint16_t port) {
        return std::make_unique<IPv4>(port, loopback_address);
    }
};

class IPv6 final : public Endpoint {
    constexpr static const std::array<std::byte, 16> any_address =
        compat::byte::arrayU<16>({ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 });
    constexpr static const std::array<std::byte, 16> loopback_address =
        compat::byte::arrayU<16>({ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1 });

    const uint16_t port;
    const std::array<std::byte, 16> address;

    boost::asio::ip::address_v6 to_addr() const {
        const std::array<unsigned char, 16>& chars = reinterpret_cast<const std::array<unsigned char, 16>&>(address);
        return boost::asio::ip::address_v6(chars);
    }
public:
    constexpr IPv6(uint16_t port, const std::array<std::byte, 16>& address)
        : port(port), address(address) {}

    constexpr bool operator == (const IPv6&) const = default;

    bool is_permissionless() const override {
        return false;
    }

    bool is_local() const override {
        // ::
        if (address == any_address) return true;
        // ::1
        if (address == loopback_address) return true;
        // fe80:: - febf:ffff:ffff:ffff:ffff:ffff:ffff:ffff
        if (   address[0] == std::byte{0xFE}
            && address[1] == std::byte{0x80}
            && address[2] == std::byte{0x00}
            && address[3] == std::byte{0x00}
            && address[4] == std::byte{0x00}
            && address[5] == std::byte{0x00}
            && address[6] == std::byte{0x00}
            && address[7] == std::byte{0x00}
        ) return true;

        return false;
    }

    bool is_private() const override {
        // 0200:: - 03ff:ffff:ffff:ffff:ffff:ffff:ffff:ffff
        if ((address[0] & std::byte{0xFE}) == std::byte{0x02}) return true;
        // fc00:: - fdff:ffff:ffff:ffff:ffff:ffff:ffff:ffff
        if ((address[0] & std::byte{0xFE}) == std::byte{0xFC}) return true;

        return false;
    }

    boost::asio::ip::tcp::endpoint to_boost() const override {
        return { to_addr(), port };
    }

    std::string to_host() const override {
        return to_addr().to_string();
    }

    std::string to_log(bool detail) const override {
        if (detail)
            return fmt::format("[{}]:{}", to_host(), port);
        else if (is_local())
            return "IPv6 local";
        else
            return "IPv6 endpoint";
    }

    static endpoint_ptr parse(const std::string_view& string, uint16_t port) {
        try {
            auto chars = boost::asio::ip::make_address_v6(string).to_bytes();
            return std::make_unique<IPv6>(
                port,
                reinterpret_cast<const std::array<std::byte, 16>&>(chars)
            );
        } catch (...) {
            return nullptr;
        }
    }

    static endpoint_ptr any(uint16_t port) {
        return std::make_unique<IPv6>(port, any_address);
    }

    static endpoint_ptr loopback(uint16_t port) {
        return std::make_unique<IPv6>(port, loopback_address);
    }
};

class I2P final : public Endpoint {
    using base32 = codec::base32::codec<codec::base32::i2p>;
    constexpr static const std::string_view suffix{".b32.i2p"};

    const uint16_t port;
    const std::array<std::byte, 32> address;
public:
    constexpr I2P(uint16_t port, const std::array<std::byte, 32>& address)
        : port(port), address(address) {}

    constexpr bool operator == (const I2P&) const = default;

    bool is_permissionless() const override {
        return true;
    }

    bool is_local() const override {
        return false;
    }

    bool is_private() const override {
        return false;
    }

    boost::asio::ip::tcp::endpoint to_boost() const override {
        throw Exception("Can't convert I2P endpoint to TCP/IP");
    }

    std::string to_host() const override {
        return fmt::format("{}{}", base32::encode(address), suffix);
    }

    std::string to_log(bool detail) const override {
        if (detail)
            return fmt::format("{}:{}", to_host(), port);
        else
            return "I2P endpoint";
    }

    static endpoint_ptr parse(const std::string_view& string, uint16_t port) {
        if (string.ends_with(suffix)) {
            try {
                auto bytes = base32::decode({string.begin(), string.length() - suffix.length()});
                if (bytes.size() == 32) {
                    std::array<std::byte, 32> address;
                    std::ranges::copy(bytes, address.data());
                    return std::make_unique<I2P>(port, address);
                }
            } catch (const codec::base32::Exception&) {
                return nullptr;
            }
        }
        return nullptr;
    }
};

inline endpoint_ptr parse(const std::string_view& string, uint16_t port) {
    if (auto endpoint = I2P::parse(string, port))
        return endpoint;
    else if (auto endpoint = IPv6::parse(string, port))
        return endpoint;
    else if (auto endpoint = IPv4::parse(string, port))
        return endpoint;
    else
        return nullptr;
}

}

using endpoint_ptr = endpoint::endpoint_ptr;

}

#endif

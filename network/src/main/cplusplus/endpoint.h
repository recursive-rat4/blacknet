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
#include <bit>
#include <memory>
#include <string>
#include <string_view>
#include <type_traits>
#include <boost/asio/ip/address_v4.hpp>
#include <boost/asio/ip/address_v6.hpp>
#include <boost/asio/ip/tcp.hpp>
#include <fmt/format.h>

#include "base32.h"
#include "byte.h"
#include "fastrng.h"
#include "hash_output_stream.h"
#include "input_stream.h"
#include "output_stream.h"
#include "sha3.h"
#include "siphash.h"

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

enum class Enum : uint8_t {
    IPv4 = 128,
    IPv6 = 129,
    TORv2 = 130,
    TORv3 = 131,
    I2P = 132,
};

struct Endpoint {
    virtual ~Endpoint() noexcept = default;

    virtual bool operator == (const Endpoint& other) const = 0;

    virtual Enum ordinal() const = 0;
    virtual bool is_permissionless() const = 0;
    virtual bool is_local() const = 0;
    virtual bool is_private() const = 0;
    virtual boost::asio::ip::tcp::endpoint to_boost() const = 0;
    virtual std::string to_host() const = 0;
    virtual std::string to_log(bool detail) const = 0;

    virtual void serialize(io::output_stream& os) const = 0;
};
using endpoint_ptr = std::shared_ptr<Endpoint>;

class IPv4 final : public Endpoint {
    constexpr static const std::array<std::byte, 4> any_address =
        compat::byte::arrayU<4>({ 0, 0, 0, 0});
    constexpr static const std::array<std::byte, 4> loopback_address =
        compat::byte::arrayU<4>({ 127, 0, 0, 1 });

    uint16_t port;
    std::array<std::byte, 4> address;

    boost::asio::ip::address_v4 to_addr() const {
        const std::array<unsigned char, 4>& chars = reinterpret_cast<const std::array<unsigned char, 4>&>(address);
        return boost::asio::ip::address_v4(chars);
    }
public:
    constexpr IPv4() noexcept = default;
    constexpr IPv4(uint16_t port, const std::array<std::byte, 4>& address)
        : port(port), address(address) {}

    constexpr bool operator == (const IPv4& other) const {
        if (port != other.port)
            return false;
        if (address != other.address)
            return false;
        return true;
    }

    bool operator == (const Endpoint& other) const override {
        if (other.ordinal() != Enum::IPv4)
            return false;
        return (*this) == static_cast<const IPv4&>(other);
    }

    Enum ordinal() const override {
        return Enum::IPv4;
    }

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

    void serialize(io::output_stream& os) const override {
        os.write(static_cast<std::byte>(Enum::IPv4));
        os.write_u16(port);
        os.write(address);
    }

    static endpoint_ptr parse(const std::string_view& string, uint16_t port) {
        try {
            auto chars = boost::asio::ip::make_address_v4(string).to_bytes();
            return std::make_shared<IPv4>(
                port,
                reinterpret_cast<const std::array<std::byte, 4>&>(chars)
            );
        } catch (const boost::system::system_error& e) {
            return nullptr;
        }
    }

    static endpoint_ptr deserialize(io::input_stream& is) {
        auto endpoint = std::make_shared<IPv4>();
        endpoint->port = is.read_u16();
        is.read(endpoint->address);
        return endpoint;
    }

    static endpoint_ptr any(uint16_t port) {
        return std::make_shared<IPv4>(port, any_address);
    }

    static endpoint_ptr loopback(uint16_t port) {
        return std::make_shared<IPv4>(port, loopback_address);
    }
};

class IPv6 final : public Endpoint {
    constexpr static const std::array<std::byte, 16> any_address =
        compat::byte::arrayU<16>({ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 });
    constexpr static const std::array<std::byte, 16> loopback_address =
        compat::byte::arrayU<16>({ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1 });

    uint16_t port;
    std::array<std::byte, 16> address;

    boost::asio::ip::address_v6 to_addr() const {
        const std::array<unsigned char, 16>& chars = reinterpret_cast<const std::array<unsigned char, 16>&>(address);
        return boost::asio::ip::address_v6(chars);
    }
public:
    constexpr IPv6() noexcept = default;
    constexpr IPv6(uint16_t port, const std::array<std::byte, 16>& address)
        : port(port), address(address) {}

    constexpr bool operator == (const IPv6& other) const {
        if (port != other.port)
            return false;
        if (address != other.address)
            return false;
        return true;
    }

    bool operator == (const Endpoint& other) const override {
        if (other.ordinal() != Enum::IPv6)
            return false;
        return (*this) == static_cast<const IPv6&>(other);
    }

    Enum ordinal() const override {
        return Enum::IPv6;
    }

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

    void serialize(io::output_stream& os) const override {
        os.write(static_cast<std::byte>(Enum::IPv6));
        os.write_u16(port);
        os.write(address);
    }

    static endpoint_ptr parse(const std::string_view& string, uint16_t port) {
        try {
            auto chars = boost::asio::ip::make_address_v6(string).to_bytes();
            return std::make_shared<IPv6>(
                port,
                reinterpret_cast<const std::array<std::byte, 16>&>(chars)
            );
        } catch (...) {
            return nullptr;
        }
    }

    static endpoint_ptr deserialize(io::input_stream& is) {
        auto endpoint = std::make_shared<IPv6>();
        endpoint->port = is.read_u16();
        is.read(endpoint->address);
        return endpoint;
    }

    static endpoint_ptr any(uint16_t port) {
        return std::make_shared<IPv6>(port, any_address);
    }

    static endpoint_ptr loopback(uint16_t port) {
        return std::make_shared<IPv6>(port, loopback_address);
    }
};

// https://gitlab.torproject.org/tpo/core/torspec/-/blob/main/spec/rend-spec/encoding-onion-addresses.md
class TORv3 final : public Endpoint {
    using base32 = codec::base32::codec<codec::base32::tor>;
    constexpr static const std::string_view suffix{".onion"};
    constexpr static const std::byte version{3};

    uint16_t port;
    std::array<std::byte, 32> address;

    constexpr static std::array<std::byte, 2> checksum(const std::span<const std::byte>& bytes) {
        constexpr std::string_view constant{".onion checksum"};
        crypto::sha3_256 hasher;
        hasher.update(constant.data(), constant.size());
        hasher.update(bytes.data(), bytes.size());
        hasher.update(&version, sizeof(version));
        std::array<std::byte, 32> hash = hasher.result();
        std::array<std::byte, 2> checksum;
        std::ranges::copy(hash.begin(), hash.begin() + checksum.size(), checksum.begin());
        return checksum;
    }
public:
    constexpr TORv3() noexcept = default;
    constexpr TORv3(uint16_t port, const std::array<std::byte, 32>& address)
        : port(port), address(address) {}

    constexpr bool operator == (const TORv3& other) const {
        if (port != other.port)
            return false;
        if (address != other.address)
            return false;
        return true;
    }

    bool operator == (const Endpoint& other) const override {
        if (other.ordinal() != Enum::TORv3)
            return false;
        return (*this) == static_cast<const TORv3&>(other);
    }

    Enum ordinal() const override {
        return Enum::TORv3;
    }

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
        throw Exception("Can't convert TORv3 endpoint to TCP/IP");
    }

    std::string to_host() const override {
        auto chksum = checksum(address);
        std::array<std::byte, 35> bytes;
        std::ranges::copy(address, bytes.begin());
        std::ranges::copy(chksum, bytes.begin() + 32);
        bytes.back() = version;
        return fmt::format("{}{}", base32::encode(bytes), suffix);
    }

    std::string to_log(bool detail) const override {
        if (detail)
            return fmt::format("{}:{}", to_host(), port);
        else
            return "TORv3 endpoint";
    }

    void serialize(io::output_stream& os) const override {
        os.write(static_cast<std::byte>(Enum::TORv3));
        os.write_u16(port);
        os.write(address);
    }

    static endpoint_ptr parse(const std::string_view& string, uint16_t port) {
        if (string.ends_with(suffix)) {
            try {
                auto bytes = base32::decode({string.begin(), string.length() - suffix.length()});
                if (bytes.size() == 35) {
                    if (bytes.back() != version) return nullptr;
                    auto pubkey = std::span<std::byte>(bytes.data(), 32);
                    auto chksum = std::span<std::byte>(bytes.data() + 32, 2);
                    if (!std::ranges::equal(chksum, checksum(pubkey))) return nullptr;
                    std::array<std::byte, 32> address;
                    std::ranges::copy(pubkey, address.data());
                    return std::make_shared<TORv3>(port, address);
                }
            } catch (const codec::base32::Exception&) {
                return nullptr;
            }
        }
        return nullptr;
    }

    static endpoint_ptr deserialize(io::input_stream& is) {
        auto endpoint = std::make_shared<TORv3>();
        endpoint->port = is.read_u16();
        is.read(endpoint->address);
        return endpoint;
    }
};

class I2P final : public Endpoint {
    using base32 = codec::base32::codec<codec::base32::i2p>;
    constexpr static const std::string_view suffix{".b32.i2p"};

    uint16_t port;
    std::array<std::byte, 32> address;
public:
    constexpr I2P() noexcept = default;
    constexpr I2P(uint16_t port, const std::array<std::byte, 32>& address)
        : port(port), address(address) {}

    constexpr bool operator == (const I2P& other) const {
        if (port != other.port)
            return false;
        if (address != other.address)
            return false;
        return true;
    }

    bool operator == (const Endpoint& other) const override {
        if (other.ordinal() != Enum::I2P)
            return false;
        return (*this) == static_cast<const I2P&>(other);
    }

    Enum ordinal() const override {
        return Enum::I2P;
    }

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

    void serialize(io::output_stream& os) const override {
        os.write(static_cast<std::byte>(Enum::I2P));
        os.write_u16(port);
        os.write(address);
    }

    static endpoint_ptr parse(const std::string_view& string, uint16_t port) {
        if (string.ends_with(suffix)) {
            try {
                auto bytes = base32::decode({string.begin(), string.length() - suffix.length()});
                if (bytes.size() == 32) {
                    std::array<std::byte, 32> address;
                    std::ranges::copy(bytes, address.data());
                    return std::make_shared<I2P>(port, address);
                }
            } catch (const codec::base32::Exception&) {
                return nullptr;
            }
        }
        return nullptr;
    }

    static endpoint_ptr deserialize(io::input_stream& is) {
        auto endpoint = std::make_shared<I2P>();
        endpoint->port = is.read_u16();
        is.read(endpoint->address);
        return endpoint;
    }
};

inline endpoint_ptr parse(const std::string_view& string, uint16_t port) {
    if (auto endpoint = I2P::parse(string, port))
        return endpoint;
    else if (auto endpoint = TORv3::parse(string, port))
        return endpoint;
    else if (auto endpoint = IPv6::parse(string, port))
        return endpoint;
    else if (auto endpoint = IPv4::parse(string, port))
        return endpoint;
    else
        return nullptr;
}

inline endpoint_ptr deserialize(io::input_stream& is) {
    Enum ordinal = static_cast<Enum>(is.read());
    if (ordinal == Enum::I2P)
        return I2P::deserialize(is);
    else if (ordinal == Enum::TORv3)
        return TORv3::deserialize(is);
    else if (ordinal == Enum::IPv6)
        return IPv6::deserialize(is);
    else if (ordinal == Enum::IPv4)
        return IPv4::deserialize(is);
    else
        return nullptr;
}

class comparator {
public:
    constexpr bool operator()(const endpoint_ptr& lps, const endpoint_ptr& rps) const {
        return (*lps) == (*rps);
    }
};

class hasher {
    static std::array<std::byte, 16> generate_key() {
        auto& rng = crypto::tls_fast_rng;
        std::array<std::byte, 16> key;
        std::uniform_int_distribution<unsigned char> ud;
        std::ranges::generate(key, [&] { return std::byte{ ud(rng) }; });
        return key;
    }
    static const std::array<std::byte, 16>& key() {
        static std::array<std::byte, 16> key = generate_key();
        return key;
    }
public:
    using is_avalanching = std::true_type;

    std::size_t operator () (const endpoint_ptr& endpoint) const {
        io::hash_output_stream<crypto::siphash_64, std::endian::native> os(key());
        endpoint->serialize(os);
        return os.digest();
    }
};

}

using endpoint_ptr = endpoint::endpoint_ptr;

}

#endif

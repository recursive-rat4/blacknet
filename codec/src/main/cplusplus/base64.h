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

#ifndef BLACKNET_CODEC_BASE64_H
#define BLACKNET_CODEC_BASE64_H

#include <cstddef>
#include <array>
#include <exception>
#include <span>
#include <string>
#include <string_view>
#include <vector>
#include <fmt/format.h>

#include "byte.h"

namespace blacknet::codec {

namespace base64 {

class Exception : public std::exception {
    std::string message;
public:
    Exception(const std::string_view& message) : message(message) {}
    virtual const char* what() const noexcept override {
        return message.c_str();
    }
};

namespace {
[[noreturn]] constexpr void throw_invalid_symbol(char symbol) {
    throw Exception(fmt::format("Symbol {:?} not in base64 alphabet", symbol));
}
}

struct rfc4648 {
    constexpr static const std::string_view alphabet
    {"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"};
    constexpr static const std::array<std::byte, 128> decode_table = compat::byte::arrayS<128>
    ({
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, 62, -1, -1, -1, 63, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, -1, -1,
        -1, -1, -1, -1, -1,  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14,
        15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, -1, -1, -1, -1, -1, -1, 26, 27, 28,
        29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
        49, 50, 51, -1, -1, -1, -1, -1
    });
    constexpr static const bool pad{true};
};

struct i2p {
    constexpr static const std::string_view alphabet
    {"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-~"};
    constexpr static const std::array<std::byte, 128> decode_table = compat::byte::arrayS<128>
    ({
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, 62, -1, -1, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, -1, -1,
        -1, -1, -1, -1, -1,  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14,
        15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, -1, -1, -1, -1, -1, -1, 26, 27, 28,
        29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
        49, 50, 51, -1, -1, -1, 63, -1
    });
    constexpr static const bool pad{true};
};

template<typename Params>
struct codec {
constexpr static std::string encode(const std::span<const std::byte>& bytes) {
    std::string string;
    string.reserve((bytes.size() + 2) / 3 * 4);
    std::size_t offset = 0;
    std::size_t remain = bytes.size();
    while (remain) {
        if (remain >= 3) {
            std::size_t idx1 = static_cast<std::size_t>(
                bytes[offset] >> 2
            );
            string.push_back(Params::alphabet[idx1]);
            std::size_t idx2 = static_cast<std::size_t>(
                ((bytes[offset] & std::byte{0x03}) << 4) | (bytes[offset + 1] >> 4)
            );
            string.push_back(Params::alphabet[idx2]);
            std::size_t idx3 = static_cast<std::size_t>(
                ((bytes[offset + 1] & std::byte{0x0F}) << 2) | (bytes[offset + 2] >> 6)
            );
            string.push_back(Params::alphabet[idx3]);
            std::size_t idx4 = static_cast<std::size_t>(
                bytes[offset + 2] & std::byte{0x3F}
            );
            string.push_back(Params::alphabet[idx4]);
            remain -= 3;
            offset += 3;
        } else if (remain == 2) {
            std::size_t idx1 = static_cast<std::size_t>(
                bytes[offset] >> 2
            );
            string.push_back(Params::alphabet[idx1]);
            std::size_t idx2 = static_cast<std::size_t>(
                ((bytes[offset] & std::byte{0x03}) << 4) | (bytes[offset + 1] >> 4)
            );
            string.push_back(Params::alphabet[idx2]);
            std::size_t idx3 = static_cast<std::size_t>(
                ((bytes[offset + 1] & std::byte{0x0F}) << 2)
            );
            string.push_back(Params::alphabet[idx3]);
            if constexpr (Params::pad) {
                string.push_back('=');
            }
            remain -= 2;
            offset += 2;
        } else {
            [[assume(remain == 1)]];
            std::size_t idx1 = static_cast<std::size_t>(
                bytes[offset] >> 2
            );
            string.push_back(Params::alphabet[idx1]);
            std::size_t idx2 = static_cast<std::size_t>(
                ((bytes[offset] & std::byte{0x03}) << 4)
            );
            string.push_back(Params::alphabet[idx2]);
            if constexpr (Params::pad) {
                string.push_back('=');
                string.push_back('=');
            }
            remain -= 1;
            offset += 1;
        }
    }
    return string;
}

constexpr static std::vector<std::byte> decode(const std::string_view& string) {
    std::vector<std::byte> bytes;
    bytes.reserve(string.length() / 4 * 3 + 2);
    std::size_t offset = 0;
    std::size_t remain = string.length();
    if constexpr (Params::pad) {
        if (remain && string[remain - 1] == '=') --remain;
        if (remain && string[remain - 1] == '=') --remain;
    }
    while (remain) {
        if (remain >= 4) {
            std::size_t idx1 = static_cast<std::size_t>(string[offset]);
            if (idx1 >= Params::decode_table.size())
                throw_invalid_symbol(string[offset]);
            std::byte sixtet1 = Params::decode_table[idx1];
            if (sixtet1 == std::byte{0xFF})
                throw_invalid_symbol(string[offset]);
            std::size_t idx2 = static_cast<std::size_t>(string[offset + 1]);
            if (idx2 >= Params::decode_table.size())
                throw_invalid_symbol(string[offset + 1]);
            std::byte sixtet2 = Params::decode_table[idx2];
            if (sixtet2 == std::byte{0xFF})
                throw_invalid_symbol(string[offset + 1]);
            std::size_t idx3 = static_cast<std::size_t>(string[offset + 2]);
            if (idx3 >= Params::decode_table.size())
                throw_invalid_symbol(string[offset + 2]);
            std::byte sixtet3 = Params::decode_table[idx3];
            if (sixtet3 == std::byte{0xFF})
                throw_invalid_symbol(string[offset + 2]);
            std::size_t idx4 = static_cast<std::size_t>(string[offset + 3]);
            if (idx4 >= Params::decode_table.size())
                throw_invalid_symbol(string[offset + 3]);
            std::byte sixtet4 = Params::decode_table[idx4];
            if (sixtet4 == std::byte{0xFF})
                throw_invalid_symbol(string[offset + 3]);
            bytes.emplace_back((sixtet1 << 2) | (sixtet2 >> 4));
            bytes.emplace_back((sixtet2 << 4) | (sixtet3 >> 2));
            bytes.emplace_back((sixtet3 << 6) | sixtet4);
            remain -= 4;
            offset += 4;
        } else if (remain == 3) {
            std::size_t idx1 = static_cast<std::size_t>(string[offset]);
            if (idx1 >= Params::decode_table.size())
                throw_invalid_symbol(string[offset]);
            std::byte sixtet1 = Params::decode_table[idx1];
            if (sixtet1 == std::byte{0xFF})
                throw_invalid_symbol(string[offset]);
            std::size_t idx2 = static_cast<std::size_t>(string[offset + 1]);
            if (idx2 >= Params::decode_table.size())
                throw_invalid_symbol(string[offset + 1]);
            std::byte sixtet2 = Params::decode_table[idx2];
            if (sixtet2 == std::byte{0xFF})
                throw_invalid_symbol(string[offset + 1]);
            std::size_t idx3 = static_cast<std::size_t>(string[offset + 2]);
            if (idx3 >= Params::decode_table.size())
                throw_invalid_symbol(string[offset + 2]);
            std::byte sixtet3 = Params::decode_table[idx3];
            if (sixtet3 == std::byte{0xFF})
                throw_invalid_symbol(string[offset + 2]);
            bytes.emplace_back((sixtet1 << 2) | (sixtet2 >> 4));
            bytes.emplace_back((sixtet2 << 4) | (sixtet3 >> 2));
            remain -= 3;
            offset += 3;
        } else if (remain == 2) {
            std::size_t idx1 = static_cast<std::size_t>(string[offset]);
            if (idx1 >= Params::decode_table.size())
                throw_invalid_symbol(string[offset]);
            std::byte sixtet1 = Params::decode_table[idx1];
            if (sixtet1 == std::byte{0xFF})
                throw_invalid_symbol(string[offset]);
            std::size_t idx2 = static_cast<std::size_t>(string[offset + 1]);
            if (idx2 >= Params::decode_table.size())
                throw_invalid_symbol(string[offset + 1]);
            std::byte sixtet2 = Params::decode_table[idx2];
            if (sixtet2 == std::byte{0xFF})
                throw_invalid_symbol(string[offset + 1]);
            bytes.emplace_back((sixtet1 << 2) | (sixtet2 >> 4));
            remain -= 2;
            offset += 2;
        } else {
            [[assume(remain == 1)]];
            throw Exception("Base64 decode failed");
        }
    }
    return bytes;
}
};

}

}

#endif

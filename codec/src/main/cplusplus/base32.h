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

#ifndef BLACKNET_CODEC_BASE32_H
#define BLACKNET_CODEC_BASE32_H

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

namespace base32 {

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
    throw Exception(fmt::format("Symbol {:?} not in base32 alphabet", symbol));
}
}

struct rfc4648 {
    constexpr static const std::string_view alphabet
    {"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567"};
    constexpr static const std::array<std::byte, 128> decode_table = compat::byte::arrayS<128>
    ({
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 26, 27, 28, 29, 30, 31, -1, -1, -1, -1,
        -1, -1, -1, -1, -1,  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14,
        15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1, -1
    });
    constexpr static const bool pad{true};
};

struct i2p {
    constexpr static const std::string_view alphabet
    {"abcdefghijklmnopqrstuvwxyz234567"};
    constexpr static const std::array<std::byte, 128> decode_table = compat::byte::arrayS<128>
    ({
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 26, 27, 28, 29, 30, 31, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  0,  1,  2,
         3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
        23, 24, 25, -1, -1, -1, -1, -1
    });
    constexpr static const bool pad{false};
};

using tor = i2p;

template<typename Params>
struct codec {
constexpr static std::string encode(const std::span<const std::byte>& bytes) {
    std::string string;
    string.reserve((bytes.size() + 4) / 5 * 8);
    std::size_t offset = 0;
    std::size_t remain = bytes.size();
    while (remain) {
        if (remain >= 5) {
            std::size_t idx1 = static_cast<std::size_t>(
                bytes[offset] >> 3
            );
            string.push_back(Params::alphabet[idx1]);
            std::size_t idx2 = static_cast<std::size_t>(
                ((bytes[offset] & std::byte{0x07}) << 2) | (bytes[offset + 1] >> 6)
            );
            string.push_back(Params::alphabet[idx2]);
            std::size_t idx3 = static_cast<std::size_t>(
                (bytes[offset + 1] & std::byte{0x3E}) >> 1
            );
            string.push_back(Params::alphabet[idx3]);
            std::size_t idx4 = static_cast<std::size_t>(
                ((bytes[offset + 1] & std::byte{0x01}) << 4) | (bytes[offset + 2] >> 4)
            );
            string.push_back(Params::alphabet[idx4]);
            std::size_t idx5 = static_cast<std::size_t>(
                ((bytes[offset + 2] & std::byte{0x0F}) << 1) | (bytes[offset + 3] >> 7)
            );
            string.push_back(Params::alphabet[idx5]);
            std::size_t idx6 = static_cast<std::size_t>(
                (bytes[offset + 3] & std::byte{0x7C}) >> 2
            );
            string.push_back(Params::alphabet[idx6]);
            std::size_t idx7 = static_cast<std::size_t>(
                ((bytes[offset + 3] & std::byte{0x03}) << 3) | (bytes[offset + 4] >> 5)
            );
            string.push_back(Params::alphabet[idx7]);
            std::size_t idx8 = static_cast<std::size_t>(
                (bytes[offset + 4] & std::byte{0x1F})
            );
            string.push_back(Params::alphabet[idx8]);
            remain -= 5;
            offset += 5;
        } else if (remain == 4) {
            std::size_t idx1 = static_cast<std::size_t>(
                bytes[offset] >> 3
            );
            string.push_back(Params::alphabet[idx1]);
            std::size_t idx2 = static_cast<std::size_t>(
                ((bytes[offset] & std::byte{0x07}) << 2) | (bytes[offset + 1] >> 6)
            );
            string.push_back(Params::alphabet[idx2]);
            std::size_t idx3 = static_cast<std::size_t>(
                (bytes[offset + 1] & std::byte{0x3E}) >> 1
            );
            string.push_back(Params::alphabet[idx3]);
            std::size_t idx4 = static_cast<std::size_t>(
                ((bytes[offset + 1] & std::byte{0x01}) << 4) | (bytes[offset + 2] >> 4)
            );
            string.push_back(Params::alphabet[idx4]);
            std::size_t idx5 = static_cast<std::size_t>(
                ((bytes[offset + 2] & std::byte{0x0F}) << 1) | (bytes[offset + 3] >> 7)
            );
            string.push_back(Params::alphabet[idx5]);
            std::size_t idx6 = static_cast<std::size_t>(
                (bytes[offset + 3] & std::byte{0x7C}) >> 2
            );
            string.push_back(Params::alphabet[idx6]);
            std::size_t idx7 = static_cast<std::size_t>(
                ((bytes[offset + 3] & std::byte{0x03}) << 3)
            );
            string.push_back(Params::alphabet[idx7]);
            if constexpr (Params::pad) {
                string.push_back('=');
            }
            remain -= 4;
            offset += 4;
        } else if (remain == 3) {
            std::size_t idx1 = static_cast<std::size_t>(
                bytes[offset] >> 3
            );
            string.push_back(Params::alphabet[idx1]);
            std::size_t idx2 = static_cast<std::size_t>(
                ((bytes[offset] & std::byte{0x07}) << 2) | (bytes[offset + 1] >> 6)
            );
            string.push_back(Params::alphabet[idx2]);
            std::size_t idx3 = static_cast<std::size_t>(
                (bytes[offset + 1] & std::byte{0x3E}) >> 1
            );
            string.push_back(Params::alphabet[idx3]);
            std::size_t idx4 = static_cast<std::size_t>(
                ((bytes[offset + 1] & std::byte{0x01}) << 4) | (bytes[offset + 2] >> 4)
            );
            string.push_back(Params::alphabet[idx4]);
            std::size_t idx5 = static_cast<std::size_t>(
                ((bytes[offset + 2] & std::byte{0x0F}) << 1)
            );
            string.push_back(Params::alphabet[idx5]);
            if constexpr (Params::pad) {
                string.push_back('=');
                string.push_back('=');
                string.push_back('=');
            }
            remain -= 3;
            offset += 3;
        } else if (remain == 2) {
            std::size_t idx1 = static_cast<std::size_t>(
                bytes[offset] >> 3
            );
            string.push_back(Params::alphabet[idx1]);
            std::size_t idx2 = static_cast<std::size_t>(
                ((bytes[offset] & std::byte{0x07}) << 2) | (bytes[offset + 1] >> 6)
            );
            string.push_back(Params::alphabet[idx2]);
            std::size_t idx3 = static_cast<std::size_t>(
                (bytes[offset + 1] & std::byte{0x3E}) >> 1
            );
            string.push_back(Params::alphabet[idx3]);
            std::size_t idx4 = static_cast<std::size_t>(
                ((bytes[offset + 1] & std::byte{0x01}) << 4)
            );
            string.push_back(Params::alphabet[idx4]);
            if constexpr (Params::pad) {
                string.push_back('=');
                string.push_back('=');
                string.push_back('=');
                string.push_back('=');
            }
            remain -= 2;
            offset += 2;
        } else {
            [[assume(remain == 1)]];
            std::size_t idx1 = static_cast<std::size_t>(
                bytes[offset] >> 3
            );
            string.push_back(Params::alphabet[idx1]);
            std::size_t idx2 = static_cast<std::size_t>(
                ((bytes[offset] & std::byte{0x07}) << 2)
            );
            string.push_back(Params::alphabet[idx2]);
            if constexpr (Params::pad) {
                string.push_back('=');
                string.push_back('=');
                string.push_back('=');
                string.push_back('=');
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
    bytes.reserve(string.length() / 8 * 5 + 4);
    std::size_t offset = 0;
    std::size_t remain = string.length();
    if constexpr (Params::pad) {
        if (string.ends_with("======"))
            remain -= 6;
        else if (string.ends_with("===="))
            remain -= 4;
        else if (string.ends_with("==="))
            remain -= 3;
        else if (string.ends_with('='))
            remain -= 1;
    }
    while (remain) {
        if (remain >= 8) {
            std::size_t idx1 = static_cast<std::size_t>(string[offset]);
            if (idx1 >= Params::decode_table.size())
                throw_invalid_symbol(string[offset]);
            std::byte quintette1 = Params::decode_table[idx1];
            if (quintette1 == std::byte{0xFF})
                throw_invalid_symbol(string[offset]);
            std::size_t idx2 = static_cast<std::size_t>(string[offset + 1]);
            if (idx2 >= Params::decode_table.size())
                throw_invalid_symbol(string[offset + 1]);
            std::byte quintette2 = Params::decode_table[idx2];
            if (quintette2 == std::byte{0xFF})
                throw_invalid_symbol(string[offset + 1]);
            std::size_t idx3 = static_cast<std::size_t>(string[offset + 2]);
            if (idx3 >= Params::decode_table.size())
                throw_invalid_symbol(string[offset + 2]);
            std::byte quintette3 = Params::decode_table[idx3];
            if (quintette3 == std::byte{0xFF})
                throw_invalid_symbol(string[offset + 2]);
            std::size_t idx4 = static_cast<std::size_t>(string[offset + 3]);
            if (idx4 >= Params::decode_table.size())
                throw_invalid_symbol(string[offset + 3]);
            std::byte quintette4 = Params::decode_table[idx4];
            if (quintette4 == std::byte{0xFF})
                throw_invalid_symbol(string[offset + 3]);
            std::size_t idx5 = static_cast<std::size_t>(string[offset + 4]);
            if (idx5 >= Params::decode_table.size())
                throw_invalid_symbol(string[offset + 4]);
            std::byte quintette5 = Params::decode_table[idx5];
            if (quintette5 == std::byte{0xFF})
                throw_invalid_symbol(string[offset + 4]);
            std::size_t idx6 = static_cast<std::size_t>(string[offset + 5]);
            if (idx6 >= Params::decode_table.size())
                throw_invalid_symbol(string[offset + 5]);
            std::byte quintette6 = Params::decode_table[idx6];
            if (quintette6 == std::byte{0xFF})
                throw_invalid_symbol(string[offset + 5]);
            std::size_t idx7 = static_cast<std::size_t>(string[offset + 6]);
            if (idx7 >= Params::decode_table.size())
                throw_invalid_symbol(string[offset + 6]);
            std::byte quintette7 = Params::decode_table[idx7];
            if (quintette7 == std::byte{0xFF})
                throw_invalid_symbol(string[offset + 6]);
            std::size_t idx8 = static_cast<std::size_t>(string[offset + 7]);
            if (idx8 >= Params::decode_table.size())
                throw_invalid_symbol(string[offset + 7]);
            std::byte quintette8 = Params::decode_table[idx8];
            if (quintette8 == std::byte{0xFF})
                throw_invalid_symbol(string[offset + 7]);
            bytes.emplace_back((quintette1 << 3) | (quintette2 >> 2));
            bytes.emplace_back((quintette2 << 6) | (quintette3 << 1) | (quintette4 >> 4));
            bytes.emplace_back((quintette4 << 4) | (quintette5 >> 1));
            bytes.emplace_back((quintette5 << 7) | (quintette6 << 2) | (quintette7 >> 3));
            bytes.emplace_back((quintette7 << 5) | quintette8);
            remain -= 8;
            offset += 8;
        } else if (remain == 7) {
            std::size_t idx1 = static_cast<std::size_t>(string[offset]);
            if (idx1 >= Params::decode_table.size())
                throw_invalid_symbol(string[offset]);
            std::byte quintette1 = Params::decode_table[idx1];
            if (quintette1 == std::byte{0xFF})
                throw_invalid_symbol(string[offset]);
            std::size_t idx2 = static_cast<std::size_t>(string[offset + 1]);
            if (idx2 >= Params::decode_table.size())
                throw_invalid_symbol(string[offset + 1]);
            std::byte quintette2 = Params::decode_table[idx2];
            if (quintette2 == std::byte{0xFF})
                throw_invalid_symbol(string[offset + 1]);
            std::size_t idx3 = static_cast<std::size_t>(string[offset + 2]);
            if (idx3 >= Params::decode_table.size())
                throw_invalid_symbol(string[offset + 2]);
            std::byte quintette3 = Params::decode_table[idx3];
            if (quintette3 == std::byte{0xFF})
                throw_invalid_symbol(string[offset + 2]);
            std::size_t idx4 = static_cast<std::size_t>(string[offset + 3]);
            if (idx4 >= Params::decode_table.size())
                throw_invalid_symbol(string[offset + 3]);
            std::byte quintette4 = Params::decode_table[idx4];
            if (quintette4 == std::byte{0xFF})
                throw_invalid_symbol(string[offset + 3]);
            std::size_t idx5 = static_cast<std::size_t>(string[offset + 4]);
            if (idx5 >= Params::decode_table.size())
                throw_invalid_symbol(string[offset + 4]);
            std::byte quintette5 = Params::decode_table[idx5];
            if (quintette5 == std::byte{0xFF})
                throw_invalid_symbol(string[offset + 4]);
            std::size_t idx6 = static_cast<std::size_t>(string[offset + 5]);
            if (idx6 >= Params::decode_table.size())
                throw_invalid_symbol(string[offset + 5]);
            std::byte quintette6 = Params::decode_table[idx6];
            if (quintette6 == std::byte{0xFF})
                throw_invalid_symbol(string[offset + 5]);
            std::size_t idx7 = static_cast<std::size_t>(string[offset + 6]);
            if (idx7 >= Params::decode_table.size())
                throw_invalid_symbol(string[offset + 6]);
            std::byte quintette7 = Params::decode_table[idx7];
            if (quintette7 == std::byte{0xFF})
                throw_invalid_symbol(string[offset + 6]);
            bytes.emplace_back((quintette1 << 3) | (quintette2 >> 2));
            bytes.emplace_back((quintette2 << 6) | (quintette3 << 1) | (quintette4 >> 4));
            bytes.emplace_back((quintette4 << 4) | (quintette5 >> 1));
            bytes.emplace_back((quintette5 << 7) | (quintette6 << 2) | (quintette7 >> 3));
            remain -= 7;
            offset += 7;
        } else if (remain == 6) {
            throw Exception("Base32 decode failed");
        } else if (remain == 5) {
            std::size_t idx1 = static_cast<std::size_t>(string[offset]);
            if (idx1 >= Params::decode_table.size())
                throw_invalid_symbol(string[offset]);
            std::byte quintette1 = Params::decode_table[idx1];
            if (quintette1 == std::byte{0xFF})
                throw_invalid_symbol(string[offset]);
            std::size_t idx2 = static_cast<std::size_t>(string[offset + 1]);
            if (idx2 >= Params::decode_table.size())
                throw_invalid_symbol(string[offset + 1]);
            std::byte quintette2 = Params::decode_table[idx2];
            if (quintette2 == std::byte{0xFF})
                throw_invalid_symbol(string[offset + 1]);
            std::size_t idx3 = static_cast<std::size_t>(string[offset + 2]);
            if (idx3 >= Params::decode_table.size())
                throw_invalid_symbol(string[offset + 2]);
            std::byte quintette3 = Params::decode_table[idx3];
            if (quintette3 == std::byte{0xFF})
                throw_invalid_symbol(string[offset + 2]);
            std::size_t idx4 = static_cast<std::size_t>(string[offset + 3]);
            if (idx4 >= Params::decode_table.size())
                throw_invalid_symbol(string[offset + 3]);
            std::byte quintette4 = Params::decode_table[idx4];
            if (quintette4 == std::byte{0xFF})
                throw_invalid_symbol(string[offset + 3]);
            std::size_t idx5 = static_cast<std::size_t>(string[offset + 4]);
            if (idx5 >= Params::decode_table.size())
                throw_invalid_symbol(string[offset + 4]);
            std::byte quintette5 = Params::decode_table[idx5];
            if (quintette5 == std::byte{0xFF})
                throw_invalid_symbol(string[offset + 4]);
            bytes.emplace_back((quintette1 << 3) | (quintette2 >> 2));
            bytes.emplace_back((quintette2 << 6) | (quintette3 << 1) | (quintette4 >> 4));
            bytes.emplace_back((quintette4 << 4) | (quintette5 >> 1));
            remain -= 5;
            offset += 5;
        } else if (remain == 4) {
            std::size_t idx1 = static_cast<std::size_t>(string[offset]);
            if (idx1 >= Params::decode_table.size())
                throw_invalid_symbol(string[offset]);
            std::byte quintette1 = Params::decode_table[idx1];
            if (quintette1 == std::byte{0xFF})
                throw_invalid_symbol(string[offset]);
            std::size_t idx2 = static_cast<std::size_t>(string[offset + 1]);
            if (idx2 >= Params::decode_table.size())
                throw_invalid_symbol(string[offset + 1]);
            std::byte quintette2 = Params::decode_table[idx2];
            if (quintette2 == std::byte{0xFF})
                throw_invalid_symbol(string[offset + 1]);
            std::size_t idx3 = static_cast<std::size_t>(string[offset + 2]);
            if (idx3 >= Params::decode_table.size())
                throw_invalid_symbol(string[offset + 2]);
            std::byte quintette3 = Params::decode_table[idx3];
            if (quintette3 == std::byte{0xFF})
                throw_invalid_symbol(string[offset + 2]);
            std::size_t idx4 = static_cast<std::size_t>(string[offset + 3]);
            if (idx4 >= Params::decode_table.size())
                throw_invalid_symbol(string[offset + 3]);
            std::byte quintette4 = Params::decode_table[idx4];
            if (quintette4 == std::byte{0xFF})
                throw_invalid_symbol(string[offset + 3]);
            bytes.emplace_back((quintette1 << 3) | (quintette2 >> 2));
            bytes.emplace_back((quintette2 << 6) | (quintette3 << 1) | (quintette4 >> 4));
            remain -= 4;
            offset += 4;
        } else if (remain == 3) {
            throw Exception("Base32 decode failed");
        } else if (remain == 2) {
            std::size_t idx1 = static_cast<std::size_t>(string[offset]);
            if (idx1 >= Params::decode_table.size())
                throw_invalid_symbol(string[offset]);
            std::byte quintette1 = Params::decode_table[idx1];
            if (quintette1 == std::byte{0xFF})
                throw_invalid_symbol(string[offset]);
            std::size_t idx2 = static_cast<std::size_t>(string[offset + 1]);
            if (idx2 >= Params::decode_table.size())
                throw_invalid_symbol(string[offset + 1]);
            std::byte quintette2 = Params::decode_table[idx2];
            if (quintette2 == std::byte{0xFF})
                throw_invalid_symbol(string[offset + 1]);
            bytes.emplace_back((quintette1 << 3) | (quintette2 >> 2));
            remain -= 2;
            offset += 2;
        } else {
            [[assume(remain == 1)]];
            throw Exception("Base32 decode failed");
        }
    }
    return bytes;
}
};

}

}

#endif

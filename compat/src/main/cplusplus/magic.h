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

#ifndef BLACKNET_COMPAT_MAGIC_H
#define BLACKNET_COMPAT_MAGIC_H

#include <cstdint>
#include <filesystem>
#include <string_view>

namespace blacknet::compat {

// Various magic and personalization values that must be changed if you are a fork.

/**
 * This name is used in network protocols and logs.
 */
consteval std::string_view agent_name() noexcept {
    return "Blacknet";
}

/**
 * The name of subdirectory, where data is stored.
 */
inline std::filesystem::path xdg_subdirectory() noexcept {
    return "Blacknet";
}

/**
 * This name is used to prevent replay of message signatures.
 */
consteval std::string_view message_sign_name() noexcept {
    return "Blacknet";
}

/**
 * The prefix for addresses.
 */
consteval std::string_view address_readable_part() noexcept {
    return "blacknet";
}

/**
 * The default port for peer-to-peer networking.
 */
consteval std::uint16_t default_p2p_port() noexcept {
    return 28453;
}

/**
 * The nonce that is used for quickly distinguishing networks.
 */
consteval std::uint32_t network_magic() noexcept {
    return 0x17895E7D;
}

}

#endif

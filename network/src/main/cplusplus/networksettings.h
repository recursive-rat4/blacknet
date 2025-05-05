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

#ifndef BLACKNET_NETWORK_NETWORKSETTINGS_H
#define BLACKNET_NETWORK_NETWORKSETTINGS_H

namespace blacknet::network {

//TODO load
struct NetworkSettings {
    bool ipv6{true};
    bool ipv4{true};
    uint16_t port{28453};
    int max_incoming_connections{128};
    bool logips{false};
    std::string i2psamhost{"127.0.0.1"};
    uint16_t i2psamport{7656};
    std::string torcontrolhost{"127.0.0.1"};
    uint16_t torcontrolport{9051};
    bool i2p{true};
    bool tor{true};
};

}

#endif

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

#ifndef BLACKNET_NETWORK_PEERTABLE_H
#define BLACKNET_NETWORK_PEERTABLE_H

#include <atomic>
#include <memory>
#include <boost/asio/io_context.hpp>
#include <boost/unordered/concurrent_flat_map.hpp>

#include "endpoint.h"
#include "logger.h"

namespace blacknet::network {

class PeerTable {
    constexpr static const std::size_t max_size{8192};

    class Entry {
        std::atomic<bool> in_contact{false};
    };
    using entry_ptr = std::unique_ptr<Entry>;

    log::Logger logger{"PeerTable"};
    boost::concurrent_flat_map<
        endpoint_ptr, entry_ptr,
        endpoint::hasher, endpoint::comparator
    > peers;
public:
    PeerTable() {
        peers.reserve(max_size);
    }

    void co_spawn(boost::asio::io_context& io_context) {
    }
};

}

#endif

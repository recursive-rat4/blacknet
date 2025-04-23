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

#ifndef BLACKNET_NETWORK_CONNECTION_H
#define BLACKNET_NETWORK_CONNECTION_H

#include <atomic>
#include <memory>
#include <boost/asio/thread_pool.hpp>

#include "endpoint.h"

namespace blacknet::network {

using connection_id = uint64_t;
class Connection {
    using endpoint_ptr = endpoint::endpoint_ptr;
public:
    enum State {
        Spawning,
        Helloing,
        Communicating,
        Closing,
    };

    const connection_id id;
    const endpoint_ptr remote_endpoint;
    const endpoint_ptr local_endpoint;
private:
    std::atomic<State> state{Spawning};
public:

    void co_spawn(boost::asio::thread_pool& thread_pool) {
    }
};
using connection_ptr = std::shared_ptr<Connection>;

}

#endif

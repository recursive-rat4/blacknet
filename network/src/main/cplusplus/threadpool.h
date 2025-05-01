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

#ifndef BLACKNET_NETWORK_THREADPOOL_H
#define BLACKNET_NETWORK_THREADPOOL_H

#include "blacknet-config.h"

#include <functional>
#include <thread>
#include <vector>
#include <boost/asio/io_context.hpp>

#include "logger.h"

namespace blacknet::network {

// Thread pool for i/o coroutines.
class ThreadPool : public boost::asio::io_context {
    std::vector<std::thread> threads;

    void thread_function() {
        for (;;) {
            try {
                boost::asio::io_context::run();
                break;
            } catch (const std::exception& e) {
                log::Logger logger{"ThreadPool"};
#if FMT_VERSION >= 100000
                logger->error("{:t}", e);
#else
                logger->error("{}", e.what());
#endif
            } catch (...) {
                log::Logger logger{"ThreadPool"};
                logger->error("Unknown exception");
            }
        }
    }
public:
    ThreadPool() : boost::asio::io_context() {}
    ~ThreadPool() {
        boost::asio::io_context::stop();
        join();
    }

    void spawn() {
        unsigned concurrency = std::thread::hardware_concurrency();
        concurrency = concurrency != 0 ? concurrency * 2 : 2;
        threads.reserve(concurrency);
        for (std::size_t i = 0; i < concurrency; ++i)
            threads.emplace_back(std::bind(&ThreadPool::thread_function, this));
    }

    void join() {
        for (std::thread& thread : threads)
            thread.join();
        threads.clear();
    }
};

}

#endif

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

#include <cstdint>
#include <array>
#include <atomic>
#include <bit>
#include <memory>
#include <set>
#include <string>
#include <string_view>
#include <boost/asio/io_context.hpp>
#include <boost/unordered/concurrent_flat_map.hpp>

#include "endpoint.h"
#include "input_stream.h"
#include "logger.h"
#include "milliseconds.h"
#include "output_stream.h"

namespace blacknet::network {

class PeerTable {
public:
    constexpr static const std::size_t max_size{8192};
private:
    constexpr static const std::uint32_t file_version{5};
    constexpr static const std::string_view file_name{"peers.dat"};

    struct localhost_t {
        bool in_contact;
    };

    struct entry_t {
        std::atomic<bool> in_contact{false};

        std::uint64_t attempts{0};
        time::Milliseconds last_try{0};
        time::Milliseconds last_connected{0};
        std::string user_agent{};
        std::set<std::array<std::byte, 32>> subnetworks{};
        time::Milliseconds added{0};

        constexpr entry_t(const localhost_t& localhost)
            : in_contact(localhost.in_contact) {}

        void serialize(io::output_stream& os) const {
            os.write_u64(attempts);
            os.write_u64(last_try.number());
            os.write_u64(last_connected.number());
            os.write_u32(user_agent.size());
            os.write_str(user_agent);
            os.write_u32(subnetworks.size());
            for (const auto& i : subnetworks)
                os.write(i);
            os.write_u64(added.number());
        }
    };
    class entry_ptr {
        std::shared_ptr<entry_t> ptr;
    public:
        entry_ptr(const localhost_t& localhost) :
            ptr(std::make_shared<entry_t>(localhost)) {}

        constexpr entry_t* operator -> () noexcept {
            return ptr.operator -> ();
        }

        constexpr const entry_t* operator -> () const noexcept {
            return ptr.operator -> ();
        }

        static entry_ptr deserialize(io::input_stream& is) {
            entry_ptr entry(localhost_t{});
            entry->attempts = is.read_u64();
            entry->last_try = is.read_u64();
            entry->last_connected = is.read_u64();
            std::size_t user_agent_size = is.read_u32();
            entry->user_agent.resize(user_agent_size);
            is.read_str(entry->user_agent);
            std::size_t subnetworks_size = is.read_u32();
            for (std::size_t i = 0; i < subnetworks_size; ++i) {
                std::array<std::byte, 32> id;
                is.read(id);
                entry->subnetworks.emplace(id);
            }
            entry->added = is.read_u64();
            return entry;
        }
    };

    log::Logger logger{"PeerTable"};
    const NetworkSettings& settings;
    boost::concurrent_flat_map<
        endpoint_ptr, entry_ptr,
        endpoint::hasher, endpoint::comparator
    > peers;
public:
    PeerTable(const NetworkSettings& settings) : settings(settings) {
        peers.reserve(max_size);
    }

    bool empty() const noexcept {
        return peers.empty();
    }

    std::size_t size() const noexcept {
        return peers.size();
    }

    bool contains(const endpoint_ptr& endpoint) const {
        return peers.contains(endpoint);
    }

    bool try_contact(const endpoint_ptr& endpoint) {
        if (endpoint->is_local() || endpoint->is_private()) return false;
        bool contacted = false;
        // ignore max size
        bool inserted = peers.try_emplace_or_visit(endpoint, localhost_t{true}, [&contacted](auto& x) {
            auto& [_, entry] = x;
            bool expected = false;
            contacted = entry->in_contact.compare_exchange_strong(expected, true, std::memory_order_acq_rel);
        });
        return contacted || inserted;
    }

    void contacted(const endpoint_ptr& endpoint) {
        if (endpoint->is_local() || endpoint->is_private()) return;
        bool contacted = false;
        // ignore max size
        bool inserted = peers.try_emplace_or_visit(endpoint, localhost_t{true}, [&contacted](auto& x) {
            auto& [_, entry] = x;
            bool expected = false;
            contacted = entry->in_contact.compare_exchange_strong(expected, true, std::memory_order_acq_rel);
        });
        if (contacted || inserted)
            return;
        logger->error("Inconsistent contact to {}", endpoint->to_log(settings.logips));
    }

    void discontacted(const endpoint_ptr& endpoint) {
        if (endpoint->is_local() || endpoint->is_private()) return;
        bool discontacted = false;
        bool visited = peers.visit(endpoint, [&discontacted](auto& x) {
            auto& [_, entry] = x;
            bool expected = true;
            discontacted = entry->in_contact.compare_exchange_strong(expected, false, std::memory_order_acq_rel);
        });
        if (discontacted)
            return;
        else if (!visited)
            logger->error("Not found entry of {}", endpoint->to_log(settings.logips));
        else
            logger->error("Inconsistent discontact from {}", endpoint->to_log(settings.logips));
    }

    void co_spawn(boost::asio::io_context& io_context) {
    }

    void serialize(io::output_stream& os) const {
        auto copy = peers;
        os.write_u32(copy.size());
        copy.visit_all([&os](auto& x) {
            const auto& [endpoint, entry] = x;
            endpoint->serialize(os);
            entry->serialize(os);
        });
    }

    void deserialize(io::input_stream& is) {
        std::size_t size = is.read_u32();
        for (std::size_t i = 0; i < size; ++i) {
            auto endpoint = endpoint::deserialize(is);
            auto entry = entry_ptr::deserialize(is);
            peers.emplace(std::move(endpoint), std::move(entry));
        }
    }
};

}

#endif

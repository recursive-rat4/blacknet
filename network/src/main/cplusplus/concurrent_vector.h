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

#ifndef BLACKNET_NETWORK_CONCURRENT_VECTOR_H
#define BLACKNET_NETWORK_CONCURRENT_VECTOR_H

#include <algorithm>
#include <atomic>
#include <functional>
#include <iterator>
#include <memory>
#include <mutex>
#include <utility>
#include <vector>

//TODO __cpp_lib_constexpr_atomic >= 202411L

namespace blacknet::network {

template<typename T, typename Allocator = std::allocator<T>>
class concurrent_vector {
    using vector_t = std::vector<T, Allocator>;
    using vector_ptr = std::shared_ptr<vector_t>;

    std::atomic<vector_ptr> vector;
    std::mutex mutex;
public:
    concurrent_vector() : vector(std::make_shared<vector_t>()) {}

    bool empty() const noexcept {
        vector_ptr weak = vector.load(std::memory_order_relaxed);
        return weak->empty();
    }

    std::size_t size() const noexcept {
        vector_ptr weak = vector.load(std::memory_order_relaxed);
        return weak->size();
    }

    void clear() noexcept {
        auto scope = std::lock_guard(mutex);
        vector.exchange(std::make_shared<vector_t>(), std::memory_order_acq_rel);
    }

    void push_back_if(
        const T& value,
        const std::function<bool(const vector_t&)>& condition =
            [](const vector_t&) { return true; }
    ) {
        auto scope = std::lock_guard(mutex);
        const vector_ptr& strong = vector.load(std::memory_order_acquire);
        if (condition(*strong)) {
            vector_ptr copy = std::make_shared<vector_t>();
            copy->reserve(strong->size() + 1);
            std::ranges::copy(*strong, std::back_inserter(*copy));
            copy->push_back(value);
            vector.store(copy, std::memory_order_release);
        }
    }

    void push_back_if(
        T&& value,
        const std::function<bool(const vector_t&)>& condition =
            [](const vector_t&) { return true; }
    ) {
        auto scope = std::lock_guard(mutex);
        const vector_ptr& strong = vector.load(std::memory_order_acquire);
        if (condition(*strong)) {
            vector_ptr copy = std::make_shared<vector_t>();
            copy->reserve(strong->size() + 1);
            std::ranges::copy(*strong, std::back_inserter(*copy));
            copy->push_back(std::move(value));
            vector.store(copy, std::memory_order_release);
        }
    }

    void pop_back() {
        auto scope = std::lock_guard(mutex);
        const vector_ptr& strong = vector.load(std::memory_order_acquire);
        std::size_t size = strong->size() - 1;
        vector_ptr copy = std::make_shared<vector_t>();
        copy->reserve(size);
        std::ranges::copy(strong->cbegin(), strong->cbegin() + size, std::back_inserter(*copy));
        vector.store(copy, std::memory_order_release);
    }

    //TODO __cpp_lib_algorithm_default_value_type >= 202403L

    template<
        typename Projection = std::identity,
#if 0
        typename Projected = std::projected_value_t<T, Projection>
#else
        typename Projected
#endif
    >
    void erase(const Projected& value, Projection projection = {}) {
        auto scope = std::lock_guard(mutex);
        const vector_ptr& strong = vector.load(std::memory_order_acquire);
        auto begin = strong->cbegin();
        auto end = strong->cend();
        auto it = std::ranges::find(begin, end, value, projection);
        if (it != end) {
            vector_ptr copy = std::make_shared<vector_t>();
            copy->reserve(strong->size() - 1);
            auto inserter = std::back_inserter(*copy);
            std::ranges::copy(begin, it, inserter);
            std::ranges::copy(it + 1, end, inserter);
            vector.store(copy, std::memory_order_release);
        }
    }

    class const_iterator {
        friend concurrent_vector;
        vector_ptr weak;
        std::size_t index;
        const_iterator(vector_ptr&& weak) : weak(std::move(weak)), index(0) {}
        const_iterator(const vector_ptr& weak, std::size_t index) : weak(weak), index(index) {}
    public:
        using difference_type = std::ptrdiff_t;
        using value_type = T;
        using iterator_category = std::contiguous_iterator_tag;
        consteval const_iterator() noexcept = default;
        const_iterator(const const_iterator&) = default;
        const_iterator(const_iterator&&) noexcept = default;
        const_iterator& operator = (const const_iterator&) = default;
        const_iterator& operator = (const_iterator&&) noexcept = default;
        constexpr bool operator == (const const_iterator& other) const {
            return index == other.index;
        }
        constexpr std::strong_ordering operator <=> (const const_iterator& other) const {
            return index <=> other.index;
        }
        bool operator == (std::default_sentinel_t) const {
            return !weak || index == weak->size();
        }
        const value_type& operator * () const noexcept {
            return (*weak)[index];
        }
        const value_type* operator -> () const noexcept {
            return weak->data() + index;
        }
        constexpr const_iterator& operator ++ () {
            ++index;
            return *this;
        }
        const_iterator operator ++ (int) {
            const_iterator old(*this);
            ++*this;
            return old;
        }
        constexpr const_iterator& operator -- () {
            --index;
            return *this;
        }
        const_iterator operator -- (int) {
            const_iterator old(*this);
            --*this;
            return old;
        }
        constexpr difference_type operator - (const const_iterator& other) const {
            return index - other.index;
        }
        constexpr const_iterator& operator += (difference_type diff) {
            index += diff;
            return *this;
        }
        const_iterator operator + (difference_type diff) const {
            return { weak, index + diff };
        }
        friend const_iterator operator + (difference_type lps, const const_iterator& rps) {
            return { rps.weak, lps + rps.index };
        }
        constexpr const_iterator& operator -= (difference_type diff) {
            index -= diff;
            return *this;
        }
        const_iterator operator - (difference_type diff) const {
            return { weak, index - diff };
        }
        const value_type& operator [] (difference_type diff) const {
            return (*weak)[index + diff];
        }
    };
    static_assert(std::contiguous_iterator<const_iterator>);
    const_iterator begin() const noexcept {
        vector_ptr weak = vector.load(std::memory_order_relaxed);
        return const_iterator(std::move(weak));
    }
    consteval std::default_sentinel_t end() const noexcept {
        return std::default_sentinel;
    }
};

}

#endif

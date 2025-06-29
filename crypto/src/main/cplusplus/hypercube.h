/*
 * Copyright (c) 2024-2025 Pavel Vasin
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

#ifndef BLACKNET_CRYPTO_HYPERCUBE_H
#define BLACKNET_CRYPTO_HYPERCUBE_H

#include <iterator>
#include <vector>
#include <utility>

#include "point.h"

namespace blacknet::crypto {

template<
    typename E
>
class Hypercube {
public:
    std::size_t n;
    std::size_t v;

    constexpr Hypercube(std::size_t n) : n(n), v(1 << n) {}

    class ComposedIterator {
        friend Hypercube;
        std::size_t last;
        std::size_t index;
        constexpr ComposedIterator(const Hypercube& e) : last(e.v), index(0) {}
    public:
        using difference_type = std::ptrdiff_t;
        using value_type = std::size_t;
        consteval ComposedIterator() noexcept = default;
        constexpr ComposedIterator(const ComposedIterator&) = default;
        constexpr ComposedIterator& operator = (const ComposedIterator&) = default;
        constexpr bool operator == (const ComposedIterator& other) const {
            return index == other.index;
        }
        constexpr bool operator == (std::default_sentinel_t) const {
            return index == last;
        }
        constexpr const std::size_t& operator * () const {
            return index;
        }
        constexpr ComposedIterator& operator ++ () {
            ++index;
            return *this;
        }
        constexpr ComposedIterator operator ++ (int) {
            ComposedIterator old(*this);
            ++*this;
            return old;
        }
    };
    static_assert(std::forward_iterator<ComposedIterator>);
    constexpr ComposedIterator composedBegin() const noexcept {
        return ComposedIterator(*this);
    }
    consteval std::default_sentinel_t composedEnd() const noexcept {
        return std::default_sentinel;
    }

    class DecomposedIterator {
        friend Hypercube;
        Point<E> point;
        std::size_t last;
        std::size_t index;
        constexpr DecomposedIterator(const Hypercube& e) : point(e.n), last(e.v), index(0) {}
    public:
        using difference_type = std::ptrdiff_t;
        using value_type = Point<E>;
        consteval DecomposedIterator() noexcept = default;
        constexpr DecomposedIterator(const DecomposedIterator&) = default;
        constexpr DecomposedIterator(DecomposedIterator&&) noexcept = default;
        constexpr ~DecomposedIterator() noexcept = default;
        constexpr DecomposedIterator& operator = (const DecomposedIterator&) = default;
        constexpr DecomposedIterator& operator = (DecomposedIterator&&) noexcept = default;
        constexpr bool operator == (const DecomposedIterator& other) const {
            return index == other.index;
        }
        constexpr bool operator == (std::default_sentinel_t) const {
            return index == last;
        }
        constexpr const Point<E>& operator * () const {
            return point;
        }
        constexpr DecomposedIterator& operator ++ () {
            ++index;
            std::size_t s = last;
            for (std::size_t i = 0; i < point.size(); ++i) {
                s >>= 1;
                if ((index & s) == s)
                    point[i] = E(1);
                else
                    point[i] = E(0);
            }
            return *this;
        }
        constexpr DecomposedIterator operator ++ (int) {
            DecomposedIterator old(*this);
            ++*this;
            return old;
        }
    };
    static_assert(std::forward_iterator<DecomposedIterator>);
    constexpr DecomposedIterator decomposedBegin() const noexcept {
        return DecomposedIterator(*this);
    }
    consteval std::default_sentinel_t decomposedEnd() const noexcept {
        return std::default_sentinel;
    }

    class SplittedIterator {
        friend Hypercube;
        std::pair<std::size_t, std::size_t> data;
        std::size_t last;
        std::size_t rows;
        std::size_t columns;
        std::size_t index;
        constexpr SplittedIterator(const Hypercube& e, std::size_t rows, std::size_t columns)
            : data(std::make_pair(0, 0)), last(e.v), rows(rows), columns(columns), index(0) {}
    public:
        using difference_type = std::ptrdiff_t;
        using value_type = std::pair<std::size_t, std::size_t>;
        consteval SplittedIterator() noexcept = default;
        constexpr SplittedIterator(const SplittedIterator&) = default;
        constexpr SplittedIterator& operator = (const SplittedIterator&) = default;
        constexpr bool operator == (const SplittedIterator& other) const {
            return index == other.index;
        }
        constexpr bool operator == (std::default_sentinel_t) const {
            return index == last;
        }
        constexpr const std::pair<std::size_t, std::size_t>& operator * () const {
            return data;
        }
        constexpr SplittedIterator& operator ++ () {
            ++index;
            data = std::make_pair(index / columns, index % columns);
            return *this;
        }
        constexpr SplittedIterator operator ++ (int) {
            SplittedIterator old(*this);
            ++*this;
            return old;
        }
    };
    static_assert(std::forward_iterator<SplittedIterator>);
    constexpr SplittedIterator splittedBegin(std::size_t rows, std::size_t columns) const noexcept {
        return SplittedIterator(*this, rows, columns);
    }
    consteval std::default_sentinel_t splittedEnd() const noexcept {
        return std::default_sentinel;
    }

    template<typename P>
    constexpr static E sum(const P& p) {
        E sigma(E::additive_identity());
        Hypercube hc(p.variables());
        for (auto i = hc.decomposedBegin(); i != hc.decomposedEnd(); ++i) {
            sigma += p(*i);
        }
        return sigma;
    }
};

}

#endif

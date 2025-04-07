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

#ifndef BLACKNET_CRYPTO_BITINT_H
#define BLACKNET_CRYPTO_BITINT_H

#include <algorithm>
#include <charconv>
#include <cmath>
#include <concepts>
#include <iterator>

namespace blacknet::crypto {

template<std::size_t N>requires(N > 0)class BigInt;

template<std::size_t BITS>
requires(BITS > 0)
struct BitInt {
    typedef uint64_t L;

    constexpr static const std::size_t N = std::ceil(double(BITS)/double(sizeof(L)*8));

    L limbs[N];

    consteval BitInt() : limbs{} {}
    consteval BitInt(const std::string& hex) {
        for (std::size_t i = 0; i < N; ++i)
            std::from_chars(hex.data() + i * sizeof(L) * 2, hex.data() + (i + 1) * sizeof(L) * 2, limbs[N - i - 1], 16);
    }
    constexpr BitInt(L n) : limbs{n} {}
    template<std::size_t M>
    requires(std::same_as<decltype(limbs), decltype(BigInt<M>::limbs)>)
    constexpr BitInt(BigInt<M> n) {
        std::ranges::copy(n.limbs, limbs);
    }

    constexpr bool operator [] (std::size_t index) const {
        constexpr std::size_t w = std::log2(sizeof(L) * 8);
        constexpr std::size_t b = sizeof(L) * 8 - 1;
        return (limbs[index >> w] >> (index & b)) & 1;
    }

    consteval static std::size_t bits() { return BITS; }

    class BitIterator {
        friend BitInt;
        const BitInt* data;
        std::size_t index;
        constexpr BitIterator(const BitInt& e) : data(&e), index(0) {}
    public:
        using difference_type = std::ptrdiff_t;
        using value_type = bool;
        constexpr BitIterator(const BitIterator& other)
            : data(other.data), index(other.index) {}
        constexpr BitIterator& operator = (const BitIterator& other) {
            data = other.data;
            index = other.index;
            return *this;
        }
        constexpr bool operator == (std::default_sentinel_t) const {
            return index == BITS;
        }
        constexpr bool operator * () const {
            return (*data)[index];
        }
        constexpr BitIterator& operator ++ () {
            ++index;
            return *this;
        }
        constexpr BitIterator operator ++ (int) {
            BitIterator old(*this);
            ++*this;
            return old;
        }
    };
    static_assert(std::input_iterator<BitIterator>);
    constexpr BitIterator bitsBegin() const noexcept {
        return BitIterator(*this);
    }
    consteval std::default_sentinel_t bitsEnd() const noexcept {
        return std::default_sentinel;
    }
};

}

#endif

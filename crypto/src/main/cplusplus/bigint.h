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

#ifndef BLACKNET_CRYPTO_BIGINT_H
#define BLACKNET_CRYPTO_BIGINT_H

#include "blacknet-config.h"

#include <algorithm>
#include <bit>
#include <charconv>
#include <exception>
#include <ostream>
#include <random>
#include <fmt/format.h>
#include <fmt/ostream.h>

namespace blacknet::crypto {

class ArithmeticException : public std::exception {
    std::string message;
public:
    ArithmeticException(const std::string& message) : message(message) {}
    virtual const char* what() const noexcept override {
        return message.c_str();
    }
};

template<std::size_t N>
requires(N > 0)
struct BigInt {
    typedef __uint64_t L;
    typedef __uint128_t LL;
    typedef __int128_t SLL;
    static_assert(sizeof(L) > 1 && sizeof(L) * 2 == sizeof(LL) && sizeof(LL) == sizeof(SLL));

    L limbs[N];

    consteval BigInt() : limbs{} {}
    consteval BigInt(const std::string& hex) {
        for (std::size_t i = 0; i < N; ++i)
            std::from_chars(hex.data() + i * sizeof(L) * 2, hex.data() + (i + 1) * sizeof(L) * 2, limbs[N - i - 1], 16);
    }
    constexpr BigInt(uint8_t n) : limbs{n} {}
    constexpr BigInt(L l0, L l1, L l2, L l3) : limbs{l3, l2, l1, l0} {
        static_assert(N == 4);
    }
    constexpr BigInt(L l0, L l1, L l2, L l3, L l4, L l5, L l6, L l7) : limbs{l7, l6, l5, l4, l3, l2, l1, l0} {
        static_assert(N == 8);
    }

    constexpr bool operator == (const BigInt& other) const {
        for (std::size_t i = N; i --> 0;)
            if (limbs[i] != other.limbs[i])
                return false;
        return true;
    }
    constexpr std::strong_ordering operator <=> (const BigInt& other) const {
        for (std::size_t i = N; i --> 0;)
            if (limbs[i] < other.limbs[i])
                return std::strong_ordering::less;
            else if (limbs[i] > other.limbs[i])
                return std::strong_ordering::greater;
        return std::strong_ordering::equal;
    }

    constexpr BigInt& operator += (const BigInt& other) {
        LL c = 0;
        for (std::size_t i = 0; i < N; ++i) {
            c += LL(limbs[i]) + LL(other.limbs[i]);
            limbs[i] = c;
            c >>= sizeof(L) * 8;
        }
        return *this;
    }

    constexpr BigInt operator + (const BigInt& other) const {
        LL c = 0;
        BigInt r;
        for (std::size_t i = 0; i < N; ++i) {
            c += LL(limbs[i]) + LL(other.limbs[i]);
            r.limbs[i] = c;
            c >>= sizeof(L) * 8;
        }
        return r;
    }

    template<std::size_t M>
    constexpr BigInt<N+M> operator * (const BigInt<M>& other) const {
        LL c = 0;
        BigInt<N+M> r;
        for (std::size_t i = 0; i < N; ++i) {
            for (std::size_t j = 0; j < M; ++j) {
                c += LL(limbs[i]) * LL(other.limbs[j]) + LL(r.limbs[i + j]);
                r.limbs[i + j] = c;
                c >>= sizeof(L) * 8;
            }
            r.limbs[i + M] = c;
            c = 0;
        }
        return r;
    }

    constexpr BigInt& operator -= (const BigInt& other) {
        SLL c = 0;
        for (std::size_t i = 0; i < N; ++i) {
            c += LL(limbs[i]) - LL(other.limbs[i]);
            limbs[i] = c;
            c >>= sizeof(L) * 8;
        }
        return *this;
    }

    constexpr BigInt operator - (const BigInt& other) const {
        SLL c = 0;
        BigInt r;
        for (std::size_t i = 0; i < N; ++i) {
            c += LL(limbs[i]) - LL(other.limbs[i]);
            r.limbs[i] = c;
            c >>= sizeof(L) * 8;
        }
        return r;
    }

    constexpr BigInt<N*2> square() const {
#ifdef BLACKNET_OPTIMIZE
        return *this * *this;
#else
        L c = 0;
        BigInt<N*2> r;
        std::size_t j = N * 2;
        for (std::size_t i = N; i --> 0;) {
            LL p = LL(limbs[i]) * LL(limbs[i]);
            r.limbs[--j] = c << (sizeof(L) * 8 - 1) | p >> (sizeof(L) * 8 + 1);
            r.limbs[--j] = p >> 1;
            c = p;
        }

        j = 2;
        LL b = 0;
        for (std::size_t i = 1; i < N; ++i) {
            LL d = 0;
            for (std::size_t k = 0; k < i; ++k) {
                d += LL(limbs[i]) * LL(limbs[k]) + LL(r.limbs[i + k]);
                r.limbs[i + k] = d;
                d >>= sizeof(L) * 8;
            }
            b += d;
            b += r.limbs[j];
            r.limbs[j++] = b;
            b >>= sizeof(L) * 8;
            b += r.limbs[j];
            r.limbs[j++] = b;
            b >>= sizeof(L) * 8;
        }

        c = limbs[0] << (sizeof(L) * 8 - 1);
        for (std::size_t i = 0; i < N * 2; ++i) {
            L d = r.limbs[i];
            r.limbs[i] = d << 1 | c >> (sizeof(L) * 8 - 1);
            c = d;
        }

        return r;
#endif
    }

    constexpr BigInt douple() const {
        L c = 0;
        BigInt r;
        for (std::size_t i = 0; i < N; ++i) {
            r.limbs[i] = (limbs[i] << 1) | c;
            c = limbs[i] >> (sizeof(L) * 8 - 1);
        }
        return r;
    }

    constexpr BigInt halve() const {
        L c = 0;
        BigInt r;
        for (std::size_t i = N; i --> 0;) {
            r.limbs[i] = (limbs[i] >> 1) | (c << (sizeof(L) * 8 - 1));
            c = limbs[i] & 1;
        }
        return r;
    }

    constexpr bool isEven() const {
        return !(*this)[0];
    }

    constexpr bool operator [] (std::size_t index) const {
        static_assert(std::has_single_bit(sizeof(L)), "Not implemented");
        constexpr std::size_t w = std::countr_zero(sizeof(L) * 8);
        constexpr std::size_t b = sizeof(L) * 8 - 1;
        return (limbs[index >> w] >> (index & b)) & 1;
    }

    consteval static std::size_t BITS() { return N * sizeof(L) * 8; }

    consteval static std::size_t LIMBS() { return N; }

    template<typename RNG>
    static BigInt random(RNG& rng) {
        std::uniform_int_distribution<L> ud;
        BigInt r;
        std::ranges::generate(r.limbs, [&] { return ud(rng); });
        return r;
    }

    friend std::ostream& operator << (std::ostream& out, const BigInt& val)
    {
        for (std::size_t i = N; i --> 0;)
            fmt::print(out, limb_format(), val.limbs[i]);
        return out;
    }
private:
    consteval static const char* limb_format() noexcept {
        if constexpr (sizeof(L) == 8)
            return "{:016X}";
        else if constexpr (sizeof(L) == 4)
            return "{:08X}";
        else if constexpr (sizeof(L) == 2)
            return "{:04X}";
        else if constexpr (sizeof(L) == 1)
            return "{:02X}";
        else
            static_assert(false, "Not implemented");
    }
};

typedef BigInt<4> UInt256;
typedef BigInt<8> UInt512;
typedef BigInt<16> UInt1024;

}

#endif

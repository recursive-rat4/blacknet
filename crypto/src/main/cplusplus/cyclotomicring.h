/*
 * Copyright (c) 2024 Pavel Vasin
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

#ifndef BLACKNET_CRYPTO_CYCLOTOMICRING_H
#define BLACKNET_CRYPTO_CYCLOTOMICRING_H

#include <array>
#include <iostream>
#include <boost/io/ostream_joiner.hpp>

template<
    typename Z,
    std::size_t N,
    void(*CONVOLUTE)(std::array<Z,N>&, const std::array<Z,N>&, const std::array<Z,N>&)
>
class CyclotomicRing {
public:
    typedef Z Scalar;
    consteval static CyclotomicRing LEFT_ADDITIVE_IDENTITY() {
        CyclotomicRing t;
        for (std::size_t i = 0; i < N; ++i)
            t.coefficients[i] = Z::LEFT_ADDITIVE_IDENTITY();
        return t;
    }
    consteval static CyclotomicRing LEFT_MULTIPLICATIVE_IDENTITY() {
        CyclotomicRing t;
        t.coefficients[0] = Z::LEFT_MULTIPLICATIVE_IDENTITY();
        for (std::size_t i = 1; i < N; ++i)
            t.coefficients[i] = Z(0);
        return t;
    }

    std::array<Z, N> coefficients;

    consteval CyclotomicRing() : coefficients() {}
    template<typename ...E>
    constexpr CyclotomicRing(E&&...e) : coefficients{std::forward<E>(e)...} {}

    constexpr bool operator == (const CyclotomicRing&) const = default;

    constexpr CyclotomicRing& operator += (const CyclotomicRing& other) {
        for (std::size_t i = 0; i < N; ++i)
            coefficients[i] += other.coefficients[i];
        return *this;
    }

    constexpr CyclotomicRing operator + (const CyclotomicRing& other) const {
        CyclotomicRing t;
        for (std::size_t i = 0; i < N; ++i)
            t.coefficients[i] = coefficients[i] + other.coefficients[i];
        return t;
    }

    constexpr CyclotomicRing& operator *= (const CyclotomicRing& other) {
        return *this = *this * other;
    }

    constexpr CyclotomicRing operator * (const CyclotomicRing& other) const {
        CyclotomicRing t(CyclotomicRing::LEFT_ADDITIVE_IDENTITY());
        CONVOLUTE(t.coefficients, this->coefficients, other.coefficients);
        return t;
    }

    constexpr CyclotomicRing& operator *= (const Scalar& other) {
        for (std::size_t i = 0; i < N; ++i)
            coefficients[i] *= other;
        return *this;
    }

    constexpr CyclotomicRing operator * (const Scalar& other) const {
        CyclotomicRing t;
        for (std::size_t i = 0; i < N; ++i)
            t.coefficients[i] = coefficients[i] * other;
        return t;
    }

    constexpr CyclotomicRing& operator -= (const CyclotomicRing& other) {
        for (std::size_t i = 0; i < N; ++i)
            coefficients[i] -= other.coefficients[i];
        return *this;
    }

    constexpr CyclotomicRing operator - (const CyclotomicRing& other) const {
        CyclotomicRing t;
        for (std::size_t i = 0; i < N; ++i)
            t.coefficients[i] = coefficients[i] - other.coefficients[i];
        return t;
    }

    constexpr bool checkInfiniteNorm(typename Z::value_type bound) const {
        for (std::size_t i = 0; i < N; ++i) {
            if (coefficients[i].checkInfiniteNorm(bound))
                continue;
            else
                return false;
        }
        return true;
    }

    friend std::ostream& operator << (std::ostream& out, const CyclotomicRing& val)
    {
        out << '[';
        std::copy(std::begin(val.coefficients), std::end(val.coefficients), boost::io::make_ostream_joiner(out, ", "));
        return out << ']';
    }

    template<typename RNG>
    static CyclotomicRing random(RNG& rng) {
        CyclotomicRing t;
        for (std::size_t i = 0; i < N; ++i)
            t.coefficients[i] = Z::random(rng);
        return t;
    }
};

#endif

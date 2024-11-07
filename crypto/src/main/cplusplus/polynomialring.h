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

#ifndef BLACKNET_CRYPTO_POLYNOMIALRING_H
#define BLACKNET_CRYPTO_POLYNOMIALRING_H

#include <array>
#include <iostream>
#include <boost/io/ostream_joiner.hpp>

#include "convolution.h"

template<
    typename Z,
    std::size_t N,
    void(*CONVOLUTE)(std::array<Z,N>&, const std::array<Z,N>&, const std::array<Z,N>&)
>
class PolynomialRing {
public:
    typedef Z Scalar;
    consteval static PolynomialRing LEFT_ADDITIVE_IDENTITY() {
        PolynomialRing t;
        for (std::size_t i = 0; i < N; ++i)
            t.coefficients[i] = Z::LEFT_ADDITIVE_IDENTITY();
        return t;
    }
    consteval static PolynomialRing LEFT_MULTIPLICATIVE_IDENTITY() {
        PolynomialRing t;
        t.coefficients[0] = Z::LEFT_MULTIPLICATIVE_IDENTITY();
        for (std::size_t i = 1; i < N; ++i)
            t.coefficients[i] = Z(0);
        return t;
    }

    using NormType = Z::NormType;

    std::array<Z, N> coefficients;

    consteval PolynomialRing() : coefficients() {}
    constexpr PolynomialRing(const Z& e) {
        coefficients[0] = e;
        std::fill_n(coefficients.begin() + 1, N - 1, Z(0));
    }
    constexpr PolynomialRing(std::initializer_list<Z> init) {
        std::copy(init.begin(), init.end(), coefficients.begin());
        std::fill_n(coefficients.begin() + init.size(), N - init.size(), Z(0));
    }

    constexpr bool operator == (const PolynomialRing&) const = default;

    constexpr PolynomialRing& operator += (const PolynomialRing& other) {
        for (std::size_t i = 0; i < N; ++i)
            coefficients[i] += other.coefficients[i];
        return *this;
    }

    constexpr PolynomialRing operator + (const PolynomialRing& other) const {
        PolynomialRing t;
        for (std::size_t i = 0; i < N; ++i)
            t.coefficients[i] = coefficients[i] + other.coefficients[i];
        return t;
    }

    constexpr PolynomialRing& operator *= (const PolynomialRing& other) {
        return *this = *this * other;
    }

    constexpr PolynomialRing operator * (const PolynomialRing& other) const {
        PolynomialRing t(PolynomialRing::LEFT_ADDITIVE_IDENTITY());
        CONVOLUTE(t.coefficients, this->coefficients, other.coefficients);
        return t;
    }

    constexpr PolynomialRing& operator *= (const Scalar& other) {
        for (std::size_t i = 0; i < N; ++i)
            coefficients[i] *= other;
        return *this;
    }

    constexpr PolynomialRing operator * (const Scalar& other) const {
        PolynomialRing t;
        for (std::size_t i = 0; i < N; ++i)
            t.coefficients[i] = coefficients[i] * other;
        return t;
    }

    friend constexpr PolynomialRing operator * (const Scalar& lps, const PolynomialRing& rps) {
        PolynomialRing t;
        for (std::size_t i = 0; i < N; ++i)
            t.coefficients[i] = lps * rps.coefficients[i];
        return t;
    }

    constexpr PolynomialRing& operator -= (const PolynomialRing& other) {
        for (std::size_t i = 0; i < N; ++i)
            coefficients[i] -= other.coefficients[i];
        return *this;
    }

    constexpr PolynomialRing operator - (const PolynomialRing& other) const {
        PolynomialRing t;
        for (std::size_t i = 0; i < N; ++i)
            t.coefficients[i] = coefficients[i] - other.coefficients[i];
        return t;
    }

    constexpr PolynomialRing operator - () const {
        PolynomialRing t;
        for (std::size_t i = 0; i < N; ++i)
            t.coefficients[i] = - coefficients[i];
        return t;
    }

    constexpr bool checkInfiniteNorm(const NormType& bound) const {
        for (std::size_t i = 0; i < N; ++i) {
            if (coefficients[i].checkInfiniteNorm(bound))
                continue;
            else
                return false;
        }
        return true;
    }

    constexpr PolynomialRing douple() const {
        PolynomialRing t;
        for (std::size_t i = 0; i < N; ++i)
            t.coefficients[i] = coefficients[i].douple();
        return t;
    }

    friend std::ostream& operator << (std::ostream& out, const PolynomialRing& val)
    {
        out << '[';
        std::copy(std::begin(val.coefficients), std::end(val.coefficients), boost::io::make_ostream_joiner(out, ", "));
        return out << ']';
    }

    template<typename DRG>
    constexpr void absorb(DRG& drg) const {
        for (std::size_t i = 0; i < N; ++i)
            drg.absorb(coefficients[i]);
    }

    template<typename DRG>
    constexpr static PolynomialRing squeeze(DRG& drg) {
        PolynomialRing t;
        for (std::size_t i = 0; i < N; ++i)
            t.coefficients[i] = Z::squeeze(drg);
        return t;
    }

    template<typename RNG>
    static PolynomialRing random(RNG& rng) {
        PolynomialRing t;
        for (std::size_t i = 0; i < N; ++i)
            t.coefficients[i] = Z::random(rng);
        return t;
    }

    template<typename RNG, typename DST>
    static PolynomialRing random(RNG& rng, const DST& dst) {
        PolynomialRing t;
        for (std::size_t i = 0; i < N; ++i)
            t.coefficients[i] = Z::random(rng, dst);
        return t;
    }
};

template<
    typename Z,
    std::size_t N
>
using CyclotomicRing = PolynomialRing<
    Z,
    N,
    convolution::negacyclic<Z, N>
>;

template<
    typename Z,
    std::size_t N,
    std::array<Z, N + 1> M
>
using ExtensionRing = PolynomialRing<
    Z,
    N,
    convolution::quotient<Z, N, M>
>;

#endif

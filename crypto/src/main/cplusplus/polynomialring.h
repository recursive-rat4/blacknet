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

#ifndef BLACKNET_CRYPTO_POLYNOMIALRING_H
#define BLACKNET_CRYPTO_POLYNOMIALRING_H

#include <algorithm>
#include <array>
#include <ostream>
#include <random>
#include <fmt/format.h>
#include <fmt/ostream.h>
#include <fmt/ranges.h>

#include "semigroup.h"

namespace blacknet::crypto {

template<typename Params>
class PolynomialRing {
public:
    using Z = Params::Z;
    constexpr static const std::size_t N = Params::N;

    consteval static PolynomialRing LEFT_ADDITIVE_IDENTITY() {
        PolynomialRing t;
        std::ranges::fill(t.coefficients, Z::LEFT_ADDITIVE_IDENTITY());
        Params::toForm(t.coefficients);
        return t;
    }
    consteval static PolynomialRing LEFT_MULTIPLICATIVE_IDENTITY() {
        PolynomialRing t;
        t.coefficients[0] = Z::LEFT_MULTIPLICATIVE_IDENTITY();
        std::fill_n(t.coefficients.begin() + 1, N - 1, Z(0));
        Params::toForm(t.coefficients);
        return t;
    }

    using NumericType = Z::NumericType;

    std::array<Z, N> coefficients;

    consteval PolynomialRing() : coefficients() {}
    constexpr PolynomialRing(const Z& e) {
        coefficients[0] = e;
        std::fill_n(coefficients.begin() + 1, N - 1, Z(0));
        Params::toForm(coefficients);
    }
    constexpr PolynomialRing(std::initializer_list<Z> init) {
        std::copy(init.begin(), init.end(), coefficients.begin());
        std::fill_n(coefficients.begin() + init.size(), N - init.size(), Z(0));
        Params::toForm(coefficients);
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
        Params::convolute(t.coefficients, this->coefficients, other.coefficients);
        return t;
    }

    constexpr PolynomialRing& operator *= (const Z& other) {
        for (std::size_t i = 0; i < N; ++i)
            coefficients[i] *= other;
        return *this;
    }

    constexpr PolynomialRing operator * (const Z& other) const {
        PolynomialRing t;
        for (std::size_t i = 0; i < N; ++i)
            t.coefficients[i] = coefficients[i] * other;
        return t;
    }

    friend constexpr PolynomialRing operator * (const Z& lps, const PolynomialRing& rps) {
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

    constexpr PolynomialRing douple() const {
        PolynomialRing t;
        for (std::size_t i = 0; i < N; ++i)
            t.coefficients[i] = coefficients[i].douple();
        return t;
    }

    constexpr PolynomialRing square() const {
        return *this * *this;
    }

    constexpr std::optional<PolynomialRing> invert() const {
        if (*this != PolynomialRing(0)) {
            return semigroup::power(*this, Params::PSY_MINUS_1);
        } else {
            return std::nullopt;
        }
    }

    constexpr bool checkInfinityNorm(const NumericType& bound) const {
        std::array<Z, N> t(coefficients);
        Params::fromForm(t);
        return std::ranges::all_of(t, [&bound](const Z& i) {
            return i.checkInfinityNorm(bound);
        });
    }

    friend std::ostream& operator << (std::ostream& out, const PolynomialRing& val)
    {
        std::array<Z, N> coefficients(val.coefficients);
        Params::fromForm(coefficients);
        fmt::print(out, "{}", coefficients);
        return out;
    }

    template<typename DRG>
    constexpr void absorb(DRG& drg) const {
        drg.absorb(coefficients);
    }

    template<typename DRG>
    constexpr static PolynomialRing squeeze(DRG& drg) {
        PolynomialRing t;
        drg.squeeze(t.coefficients);
        return t;
    }

    template<typename RNG>
    static PolynomialRing random(RNG& rng) {
        PolynomialRing t;
        std::ranges::generate(t.coefficients, [&] { return Z::random(rng); });
        return t;
    }

    template<typename RNG, typename DST>
    static PolynomialRing random(RNG& rng, DST& dst) {
        PolynomialRing t;
        std::ranges::generate(t.coefficients, [&] { return Z::random(rng, dst); });
        Params::toForm(t.coefficients);
        return t;
    }

    template<typename RNG, typename DST>
    static PolynomialRing random(RNG& rng, DST& dst, std::size_t hamming) {
        std::uniform_int_distribution<std::size_t> uid(0, N - 1);
        PolynomialRing t;
        std::ranges::fill(t.coefficients, Z(0));
        while (hamming) {
            std::size_t i = uid(rng);
            if (t.coefficients[i] == Z(0)) {
                if ((t.coefficients[i] = dst(rng)) != Z(0))
                    --hamming;
            }
        }
        Params::toForm(t.coefficients);
        return t;
    }
};

}

template<>
template<typename Params>
struct fmt::formatter<blacknet::crypto::PolynomialRing<Params>> : ostream_formatter {};

#endif

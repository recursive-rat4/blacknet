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
struct PolynomialRing {
private:
    using Z = Params::Z;
    constexpr static const std::size_t N = Params::N;
public:

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

    using BaseRing = Z;
    using NumericType = Z::NumericType;

    std::array<Z, N> coefficients;

    consteval PolynomialRing() noexcept = default;
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

    consteval static std::size_t size() noexcept {
        return dimension();
    }

    consteval static std::size_t dimension() noexcept {
        return Params::N;
    }

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
        static_assert(Params::is_division_ring, "Not implemented");
        if (*this != PolynomialRing(0)) {
            // Feng and Itoh-Tsujii algorithm
            PolynomialRing r1 = semigroup::power(*this, Params::INVERSION_R1);
            Z r0 = (r1 * (*this)).coefficients[0];
            Z z1 = *r0.invert();
            return z1 * r1;
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

    constexpr double euclideanNorm() const {
        std::array<Z, N> t(coefficients);
        Params::fromForm(t);
        double r = 0;
        for (std::size_t i = 0; i < N; ++i) {
            double e = t[i].euclideanNorm();
            r += e * e;
        }
        return std::sqrt(r);
    }

    constexpr decltype(auto) begin() noexcept {
        return coefficients.begin();
    }

    constexpr decltype(auto) begin() const noexcept {
        return coefficients.begin();
    }

    constexpr decltype(auto) end() noexcept {
        return coefficients.end();
    }

    constexpr decltype(auto) end() const noexcept {
        return coefficients.end();
    }

    friend std::ostream& operator << (std::ostream& out, const PolynomialRing& val)
    {
        std::array<Z, N> coefficients(val.coefficients);
        Params::fromForm(coefficients);
        fmt::print(out, "{}", coefficients);
        return out;
    }

    template<typename Sponge>
    constexpr void absorb(Sponge& sponge) const {
        sponge.absorb(coefficients);
    }

    template<typename Sponge>
    constexpr static PolynomialRing squeeze(Sponge& sponge) {
        PolynomialRing t;
        sponge.squeeze(t.coefficients);
        return t;
    }

    template<std::uniform_random_bit_generator RNG>
    static PolynomialRing random(RNG& rng) {
        PolynomialRing t;
        std::ranges::generate(t.coefficients, [&] { return Z::random(rng); });
        return t;
    }

    template<std::uniform_random_bit_generator RNG, typename DST>
    static PolynomialRing random(RNG& rng, DST& dst) {
        PolynomialRing t;
        std::ranges::generate(t.coefficients, [&] { return Z::random(rng, dst); });
        Params::toForm(t.coefficients);
        return t;
    }

    template<std::uniform_random_bit_generator RNG, typename DST>
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

#endif

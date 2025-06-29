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

#ifndef BLACKNET_CRYPTO_MATRIXRING_H
#define BLACKNET_CRYPTO_MATRIXRING_H

#include <algorithm>
#include <array>
#include <concepts>
#include <initializer_list>
#include <ostream>
#include <random>
#include <fmt/format.h>
#include <fmt/ostream.h>
#include <fmt/ranges.h>

namespace blacknet::crypto {

template<typename R,std::size_t N>struct Module;

template<typename R, std::size_t N>
struct MatrixRing {
    consteval static MatrixRing additive_identity() {
        MatrixRing t;
        t.elements.fill(R::additive_identity());
        return t;
    }
    consteval static MatrixRing multiplicative_identity() {
        return R::multiplicative_identity();
    }

    using BaseRing = R;
    using NumericType = R::NumericType;

    std::array<R, N * N> elements;

    constexpr MatrixRing() noexcept = default;
    constexpr MatrixRing(const R& e) {
        for (std::size_t i = 0; i < rows(); ++i)
            for (std::size_t j = 0; j < columns(); ++j)
                if (i != j)
                    (*this)[i, j] = R::additive_identity();
                else
                    (*this)[i, j] = e;
    }
    constexpr MatrixRing(std::initializer_list<R> init) {
        std::ranges::copy(init, elements.begin());
    }

    constexpr bool operator == (const MatrixRing&) const = default;

    consteval static std::size_t rows() noexcept {
        return N;
    }
    consteval static std::size_t columns() noexcept {
        return N;
    }

    constexpr R& operator [] (std::size_t i, std::size_t j) {
        return elements[i * columns() + j];
    }

    constexpr const R& operator [] (std::size_t i, std::size_t j) const {
        return elements[i * columns() + j];
    }

    constexpr MatrixRing& operator += (const MatrixRing& other) {
        for (std::size_t i = 0; i < rows(); ++i)
            for (std::size_t j = 0; j < columns(); ++j)
                (*this)[i, j] += other[i, j];
        return *this;
    }

    constexpr MatrixRing operator + (const MatrixRing& other) const {
        MatrixRing r;
        for (std::size_t i = 0; i < rows(); ++i)
            for (std::size_t j = 0; j < columns(); ++j)
                r[i, j] = (*this)[i, j] + other[i, j];
        return r;
    }

    constexpr MatrixRing& operator *= (const MatrixRing& other) {
        return *this = *this * other;
    }

    constexpr MatrixRing operator * (const MatrixRing& other) const {
        // Iterative algorithm
        MatrixRing r;
        r.elements.fill(R::additive_identity());
        for (std::size_t i = 0; i < rows(); ++i)
            for (std::size_t j = 0; j < other.columns(); ++j)
                for (std::size_t k = 0; k < columns(); ++k)
                    r[i, j] += (*this)[i, k] * other[k, j];
        return r;
    }

    constexpr Module<R, N> operator * (const Module<R, N>& other) const {
        auto r = Module<R, N>::additive_identity();
        for (std::size_t i = 0; i < rows(); ++i)
            for (std::size_t j = 0; j < columns(); ++j)
                r[i] += (*this)[i, j] * other[j];
        return r;
    }

    friend constexpr Module<R, N> operator * (const Module<R, N>& lps, const MatrixRing& rps) {
        auto r = Module<R, N>::additive_identity();
        for (std::size_t i = 0; i < rps.rows(); ++i)
            for (std::size_t j = 0; j < rps.columns(); ++j)
                r[j] += lps[i] * rps[i, j];
        return r;
    }

    constexpr MatrixRing& operator *= (const R& other) {
        for (std::size_t i = 0; i < rows(); ++i)
            for (std::size_t j = 0; j < columns(); ++j)
                (*this)[i, j] *= other;
        return *this;
    }

    constexpr MatrixRing operator * (const R& other) const {
        MatrixRing r;
        for (std::size_t i = 0; i < rows(); ++i)
            for (std::size_t j = 0; j < columns(); ++j)
                r[i, j] = (*this)[i, j] * other;
        return r;
    }

    friend constexpr MatrixRing operator * (const R& lps, const MatrixRing& rps) {
        MatrixRing r;
        for (std::size_t i = 0; i < rps.rows(); ++i)
            for (std::size_t j = 0; j < rps.columns(); ++j)
                r[i, j] = lps * rps[i, j];
        return r;
    }

    constexpr MatrixRing& operator -= (const MatrixRing& other) {
        for (std::size_t i = 0; i < rows(); ++i)
            for (std::size_t j = 0; j < columns(); ++j)
                (*this)[i, j] -= other[i, j];
        return *this;
    }

    constexpr MatrixRing operator - (const MatrixRing& other) const {
        MatrixRing r;
        for (std::size_t i = 0; i < rows(); ++i)
            for (std::size_t j = 0; j < columns(); ++j)
                r[i, j] = (*this)[i, j] - other[i, j];
        return r;
    }

    constexpr MatrixRing operator - () const {
        MatrixRing t;
        for (std::size_t i = 0; i < rows(); ++i)
            for (std::size_t j = 0; j < columns(); ++j)
                t[i, j] = - (*this)[i, j];
        return t;
    }

    constexpr MatrixRing douple() const {
        if constexpr (R::characteristic() != 2) {
            MatrixRing t;
            for (std::size_t i = 0; i < rows(); ++i)
                for (std::size_t j = 0; j < columns(); ++j)
                    t[i, j] = (*this)[i, j].douple();
            return t;
        } else {
            return additive_identity();
        }
    }

    constexpr MatrixRing square() const {
        return *this * *this;
    }

    constexpr MatrixRing transpose() const {
        MatrixRing r;
        for (std::size_t i = 0; i < rows(); ++i)
            for (std::size_t j = 0; j < columns(); ++j)
                r[j, i] = (*this)[i, j];
        return r;
    }

    template<typename NormType>
    requires(std::same_as<NormType, typename R::NumericType>)
    constexpr bool checkInfinityNorm(const NormType& bound) const {
        return std::ranges::all_of(elements, [&bound](const R& e) {
            return e.checkInfinityNorm(bound);
        });
    }

    friend std::ostream& operator << (std::ostream& out, const MatrixRing& val)
    {
        fmt::print(out, "{}", val.elements);
        return out;
    }

    consteval static auto characteristic() {
        return R::characteristic();
    }

    template<typename Sponge>
    constexpr static MatrixRing squeeze(Sponge& sponge) {
        MatrixRing t;
        std::ranges::generate(t.elements, [&] { return R::squeeze(sponge); });
        return t;
    }

    template<typename Sponge, typename DST>
    constexpr static MatrixRing squeeze(Sponge& sponge, DST& dst) {
        MatrixRing t;
        std::ranges::generate(t.elements, [&] { return R::squeeze(sponge, dst); });
        return t;
    }

    template<std::uniform_random_bit_generator RNG>
    static MatrixRing random(RNG& rng) {
        MatrixRing t;
        std::ranges::generate(t.elements, [&] { return R::random(rng); });
        return t;
    }

    template<std::uniform_random_bit_generator RNG, typename DST>
    static MatrixRing random(RNG& rng, DST& dst) {
        MatrixRing t;
        std::ranges::generate(t.elements, [&] { return R::random(rng, dst); });
        return t;
    }
};

}

#endif

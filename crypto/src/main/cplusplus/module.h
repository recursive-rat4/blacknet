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

#ifndef BLACKNET_CRYPTO_MODULE_H
#define BLACKNET_CRYPTO_MODULE_H

#include <algorithm>
#include <array>
#include <initializer_list>
#include <ostream>
#include <random>
#include <fmt/format.h>
#include <fmt/ostream.h>
#include <fmt/ranges.h>

namespace blacknet::crypto {

template<
    typename R,
    std::size_t N
>
struct Module {
    consteval static Module additive_identity() {
        Module t;
        t.components.fill(R(0));
        return t;
    }

    std::array<R, N> components;

    consteval Module() noexcept = default;
    constexpr Module(std::initializer_list<R> init) {
        std::ranges::copy(init, components.begin());
        std::ranges::fill_n(components.begin() + init.size(), N - init.size(), R(0));
    }
    constexpr Module(const std::array<R, N>& components) : components(components) {}
    constexpr Module(const Module&) = default;
    constexpr Module(Module&&) noexcept = default;
    constexpr ~Module() noexcept = default;

    constexpr Module& operator = (const Module&) = default;
    constexpr Module& operator = (Module&&) noexcept = default;

    constexpr bool operator == (const Module&) const = default;

    consteval static std::size_t size() noexcept {
        return rank();
    }

    constexpr R& operator [] (std::size_t i) {
        return components[i];
    }

    constexpr const R& operator [] (std::size_t i) const {
        return components[i];
    }

    consteval static std::size_t rank() noexcept {
        return N;
    }

    constexpr Module& operator += (const Module& other) {
        for (std::size_t i = 0; i < N; ++i)
            components[i] += other.components[i];
        return *this;
    }

    constexpr Module operator + (const Module& other) const {
        Module r;
        for (std::size_t i = 0; i < N; ++i)
            r.components[i] = components[i] + other.components[i];
        return r;
    }

    constexpr Module& operator *= (const R& other) {
        for (std::size_t i = 0; i < N; ++i)
            components[i] *= other;
        return *this;
    }

    constexpr Module operator * (const R& other) const {
        Module r;
        for (std::size_t i = 0; i < N; ++i)
            r.components[i] = components[i] * other;
        return r;
    }

    friend constexpr Module operator * (const R& lps, const Module& rps) {
        Module r;
        for (std::size_t i = 0; i < N; ++i)
            r.components[i] = lps * rps.components[i];
        return r;
    }

    constexpr Module& operator -= (const Module& other) {
        for (std::size_t i = 0; i < N; ++i)
            components[i] -= other.components[i];
        return *this;
    }

    constexpr Module operator - (const Module& other) const {
        Module r;
        for (std::size_t i = 0; i < N; ++i)
            r.components[i] = components[i] - other.components[i];
        return r;
    }

    constexpr Module operator - () const {
        Module r;
        for (std::size_t i = 0; i < N; ++i)
            r.components[i] = -components[i];
        return r;
    }

    constexpr Module douple() const {
        if constexpr (R::characteristic() != 2) {
            Module r;
            for (std::size_t i = 0; i < N; ++i)
                r.components[i] = components[i].douple();
            return r;
        } else {
            return additive_identity();
        }
    }

    constexpr decltype(auto) begin() noexcept {
        return components.begin();
    }

    constexpr decltype(auto) begin() const noexcept {
        return components.begin();
    }

    constexpr decltype(auto) end() noexcept {
        return components.end();
    }

    constexpr decltype(auto) end() const noexcept {
        return components.end();
    }

    friend std::ostream& operator << (std::ostream& out, const Module& val)
    {
        fmt::print(out, "{}", val.components);
        return out;
    }

    template<typename Sponge>
    constexpr void absorb(Sponge& sponge) const {
        sponge.absorb(components);
    }

    template<typename Sponge>
    constexpr static Module squeeze(Sponge& sponge) {
        Module t;
        sponge.squeeze(t.components);
        return t;
    }

    template<std::uniform_random_bit_generator RNG>
    static Module random(RNG& rng) {
        Module t;
        std::ranges::generate(t.components, [&] { return R::random(rng); });
        return t;
    }

    template<std::uniform_random_bit_generator RNG, typename DST>
    static Module random(RNG& rng, DST& dst) {
        Module t;
        std::ranges::generate(t.components, [&] { return R::random(rng, dst); });
        return t;
    }

    template<std::uniform_random_bit_generator RNG, typename DST>
    static Module random(RNG& rng, DST& dst, std::size_t hamming) {
        std::uniform_int_distribution<std::size_t> uid(0, N - 1);
        auto t = Module::additive_identity();
        while (hamming) {
            std::size_t i = uid(rng);
            if (t[i] == R(0)) {
                if ((t[i] = dst(rng)) != R(0))
                    --hamming;
            }
        }
        return t;
    }
};

}

#endif

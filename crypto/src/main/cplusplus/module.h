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
    consteval static Module identity() {
        Module t;
        t.components.fill(R(0));
        return t;
    }

    std::array<R, N> components;

    consteval Module() = default;
    constexpr Module(const std::array<R, N>& components) : components(components) {}
    constexpr Module(const Module&) = default;

    constexpr Module& operator = (const Module&) = default;

    constexpr bool operator == (const Module&) const = default;

    consteval static std::size_t rank() {
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
};

}

#endif

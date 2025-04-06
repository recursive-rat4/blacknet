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

#include "util.h"

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

    consteval Module() : components() {}
    constexpr Module(const Module& other) : components(other.components) {}
    constexpr Module(const std::array<R, N>& components) : components(components) {}

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
        return out << val.components;
    }

    template<typename DRG>
    constexpr void absorb(DRG& drg) const {
        drg.absorb(components);
    }

    template<typename DRG>
    constexpr static Module squeeze(DRG& drg) {
        Module t;
        drg.squeeze(t.components);
        return t;
    }

    template<typename RNG>
    static Module random(RNG& rng) {
        Module t;
        std::ranges::generate(t.components, [&] { return R::random(rng); });
        return t;
    }

    template<typename RNG, typename DST>
    static Module random(RNG& rng, const DST& dst) {
        Module t;
        std::ranges::generate(t.components, [&] { return R::random(rng, dst); });
        return t;
    }
};

#endif

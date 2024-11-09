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

#ifndef BLACKNET_CRYPTO_RINGPRODUCT_H
#define BLACKNET_CRYPTO_RINGPRODUCT_H

#include <iostream>
#include <tuple>
#include <type_traits>

template<typename... Rs>
struct RingProduct {
    typedef RingProduct Scalar;
    consteval static RingProduct LEFT_ADDITIVE_IDENTITY() {
        RingProduct t;
        for_each(t.ideals, [](auto&& r) { r = std::remove_reference_t<decltype(r)>::LEFT_ADDITIVE_IDENTITY(); });
        return t;
    }
    consteval static RingProduct LEFT_MULTIPLICATIVE_IDENTITY() {
        RingProduct t;
        for_each(t.ideals, [](auto&& r) { r = std::remove_reference_t<decltype(r)>::LEFT_MULTIPLICATIVE_IDENTITY(); });
        return t;
    }

    std::tuple<Rs...> ideals;

    constexpr RingProduct() {}
    constexpr RingProduct(Rs&&... rs) : ideals(std::forward<Rs>(rs)...) {}

    constexpr bool operator == (const RingProduct&) const = default;

    constexpr RingProduct& operator += (const RingProduct& other) {
        for_each(this->ideals, other.ideals, [](auto&& r, auto&& a) { r += a; });
        return *this;
    }

    constexpr RingProduct operator + (const RingProduct& other) const {
        RingProduct t;
        for_each(t.ideals, this->ideals, other.ideals, [](auto&& r, auto&& a, auto&& b) { r = a + b; });
        return t;
    }

    constexpr RingProduct& operator *= (const RingProduct& other) {
        for_each(this->ideals, other.ideals, [](auto&& r, auto&& a) { r *= a; });
        return *this;
    }

    constexpr RingProduct operator * (const RingProduct& other) const {
        RingProduct t;
        for_each(t.ideals, this->ideals, other.ideals, [](auto&& r, auto&& a, auto&& b) { r = a * b; });
        return t;
    }

    constexpr RingProduct& operator -= (const RingProduct& other) {
        for_each(this->ideals, other.ideals, [](auto&& r, auto&& a) { r -= a; });
        return *this;
    }

    constexpr RingProduct operator - (const RingProduct& other) const {
        RingProduct t;
        for_each(t.ideals, this->ideals, other.ideals, [](auto&& r, auto&& a, auto&& b) { r = a - b; });
        return t;
    }

    constexpr RingProduct operator - () const {
        RingProduct t;
        for_each(t.ideals, ideals, [](auto&& r, auto&& a) { r = - a; });
        return t;
    }

    constexpr RingProduct douple() const {
        RingProduct t;
        for_each(t.ideals, ideals, [](auto&& r, auto&& a) { r = a.douple(); });
        return t;
    }

    constexpr RingProduct square() const {
        RingProduct t;
        for_each(t.ideals, ideals, [](auto&& r, auto&& a) { r = a.square(); });
        return t;
    }

    friend std::ostream& operator << (std::ostream& out, const RingProduct& val)
    {
        std::size_t joiner{0};
        out << '[';
        std::apply([&](auto&&... i) {
            ((out << i << (++joiner != sizeof...(i) ? ", " : "")), ...);
        }, val.ideals);
        return out << ']';
    }

    template<typename RNG>
    static RingProduct random(RNG& rng) {
        RingProduct t;
        for_each(t.ideals, [&](auto&& r) { r = std::remove_reference_t<decltype(r)>::random(rng); });
        return t;
    }

    template<typename RNG, typename DST>
    static RingProduct random(RNG& rng, const DST& dst) {
        RingProduct t;
        for_each(t.ideals, [&](auto&& r) { r = std::remove_reference_t<decltype(r)>::random(rng, dst); });
        return t;
    }
private:
    constexpr static void for_each(std::tuple<Rs...>& r, const auto& f) {
        [&]<std::size_t... Is>(std::index_sequence<Is...>) {
            (f(std::get<Is>(r)), ...);
        }(std::make_index_sequence<sizeof...(Rs)>{});
    }
    constexpr static void for_each(std::tuple<Rs...>& r, const std::tuple<Rs...>& a, const auto& f) {
        [&]<std::size_t... Is>(std::index_sequence<Is...>) {
            (f(std::get<Is>(r), std::get<Is>(a)), ...);
        }(std::make_index_sequence<sizeof...(Rs)>{});
    }
    constexpr static void for_each(std::tuple<Rs...>& r, const std::tuple<Rs...>& a, const std::tuple<Rs...>& b, const auto& f) {
        [&]<std::size_t... Is>(std::index_sequence<Is...>) {
            (f(std::get<Is>(r), std::get<Is>(a), std::get<Is>(b)), ...);
        }(std::make_index_sequence<sizeof...(Rs)>{});
    }
};

#endif

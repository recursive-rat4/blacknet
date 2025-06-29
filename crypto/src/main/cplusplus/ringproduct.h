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

#ifndef BLACKNET_CRYPTO_RINGPRODUCT_H
#define BLACKNET_CRYPTO_RINGPRODUCT_H

#include <ostream>
#include <random>
#include <tuple>
#include <type_traits>

namespace blacknet::crypto {

template<typename... Rs>
struct RingProduct {
    consteval static RingProduct additive_identity() {
        RingProduct t;
        for_each(t.factors, [](auto&& r) { r = std::remove_reference_t<decltype(r)>::additive_identity(); });
        return t;
    }
    consteval static RingProduct multiplicative_identity() {
        RingProduct t;
        for_each(t.factors, [](auto&& r) { r = std::remove_reference_t<decltype(r)>::multiplicative_identity(); });
        return t;
    }

    std::tuple<Rs...> factors;

    constexpr RingProduct() = default;
    constexpr RingProduct(Rs&&... rs) : factors(std::forward<Rs>(rs)...) {}

    constexpr bool operator == (const RingProduct&) const = default;

    constexpr RingProduct& operator += (const RingProduct& other) {
        for_each(this->factors, other.factors, [](auto&& r, auto&& a) { r += a; });
        return *this;
    }

    constexpr RingProduct operator + (const RingProduct& other) const {
        RingProduct t;
        for_each(t.factors, this->factors, other.factors, [](auto&& r, auto&& a, auto&& b) { r = a + b; });
        return t;
    }

    constexpr RingProduct& operator *= (const RingProduct& other) {
        for_each(this->factors, other.factors, [](auto&& r, auto&& a) { r *= a; });
        return *this;
    }

    constexpr RingProduct operator * (const RingProduct& other) const {
        RingProduct t;
        for_each(t.factors, this->factors, other.factors, [](auto&& r, auto&& a, auto&& b) { r = a * b; });
        return t;
    }

    constexpr RingProduct& operator -= (const RingProduct& other) {
        for_each(this->factors, other.factors, [](auto&& r, auto&& a) { r -= a; });
        return *this;
    }

    constexpr RingProduct operator - (const RingProduct& other) const {
        RingProduct t;
        for_each(t.factors, this->factors, other.factors, [](auto&& r, auto&& a, auto&& b) { r = a - b; });
        return t;
    }

    constexpr RingProduct operator - () const {
        RingProduct t;
        for_each(t.factors, factors, [](auto&& r, auto&& a) { r = - a; });
        return t;
    }

    constexpr RingProduct douple() const {
        RingProduct t;
        for_each(t.factors, factors, [](auto&& r, auto&& a) { r = a.douple(); });
        return t;
    }

    constexpr RingProduct square() const {
        RingProduct t;
        for_each(t.factors, factors, [](auto&& r, auto&& a) { r = a.square(); });
        return t;
    }

    friend std::ostream& operator << (std::ostream& out, const RingProduct& val)
    {
        std::size_t joiner{0};
        out << '[';
        std::apply([&](auto&&... i) {
            ((out << i << (++joiner != sizeof...(i) ? ", " : "")), ...);
        }, val.factors);
        return out << ']';
    }

    template<std::uniform_random_bit_generator RNG>
    static RingProduct random(RNG& rng) {
        RingProduct t;
        for_each(t.factors, [&](auto&& r) { r = std::remove_reference_t<decltype(r)>::random(rng); });
        return t;
    }

    template<std::uniform_random_bit_generator RNG, typename DST>
    static RingProduct random(RNG& rng, DST& dst) {
        RingProduct t;
        for_each(t.factors, [&](auto&& r) { r = std::remove_reference_t<decltype(r)>::random(rng, dst); });
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

}

#endif

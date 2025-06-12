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

#ifndef BLACKNET_CRYPTO_POINT_H
#define BLACKNET_CRYPTO_POINT_H

#include <concepts>
#include <initializer_list>
#include <ostream>
#include <vector>
#include <fmt/format.h>
#include <fmt/ostream.h>
#include <fmt/ranges.h>

namespace blacknet::crypto {

template<typename S>
struct Point {
    std::vector<S> coordinates;

    consteval Point() noexcept = default;
    constexpr Point(std::size_t size) : coordinates(size) {}
    constexpr Point(std::initializer_list<S> init) : coordinates(init) {}
    constexpr Point(std::vector<S>&& coordinates) : coordinates(std::move(coordinates)) {}
    constexpr Point(const Point&) = default;
    constexpr Point(Point&&) noexcept = default;
    constexpr ~Point() noexcept = default;

    constexpr Point& operator = (const Point&) = default;
    constexpr Point& operator = (Point&&) noexcept = default;

    constexpr bool operator == (const Point&) const = default;

    constexpr std::size_t size() const noexcept {
        return coordinates.size();
    }

    constexpr S& operator [] (std::size_t i) {
        return coordinates[i];
    }

    constexpr const S& operator [] (std::size_t i) const {
        return coordinates[i];
    }

    constexpr decltype(auto) begin() noexcept {
        return coordinates.begin();
    }

    constexpr decltype(auto) begin() const noexcept {
        return coordinates.begin();
    }

    constexpr decltype(auto) end() noexcept {
        return coordinates.end();
    }

    constexpr decltype(auto) end() const noexcept {
        return coordinates.end();
    }

    friend std::ostream& operator << (std::ostream& out, const Point& val)
    {
        fmt::print(out, "{}", val.coordinates);
        return out;
    }

    template<typename Sponge>
    constexpr void absorb(Sponge& sponge) const {
        for (std::size_t i = 0; i < coordinates.size(); ++i)
            coordinates[i].absorb(sponge);
    }

template<typename Builder>
requires(std::same_as<S, typename Builder::R>)
struct Circuit {
    using Variable = Builder::Variable;
    using LinearCombination = Builder::LinearCombination;

    std::vector<LinearCombination> coordinates;

    constexpr Circuit(std::size_t size) : coordinates(size) {}

    constexpr Circuit(Builder& circuit, Variable::Type type, std::size_t size) : coordinates(size) {
        std::ranges::generate(coordinates, [&]{ return circuit.variable(type); });
    }

    constexpr LinearCombination& operator [] (std::size_t i) {
        return coordinates[i];
    }

    constexpr const LinearCombination& operator [] (std::size_t i) const {
        return coordinates[i];
    }
};

};

}

#endif

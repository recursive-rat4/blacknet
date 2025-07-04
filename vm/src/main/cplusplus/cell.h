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

#ifndef BLACKNET_VM_CELL_H
#define BLACKNET_VM_CELL_H

#include <concepts>
#include <ranges>
#include <stdexcept>
#include <vector>
#include <fmt/format.h>
#include <fmt/ostream.h>
#include <fmt/ranges.h>

namespace blacknet::vm {

template<typename Jive>
struct Cell {
    using E = Jive::Hash::value_type;
    using Hash = Jive::Hash;

    Hash value;

    constexpr Cell(const Hash& value) : value(value) {}

    constexpr bool operator == (const Cell&) const = default;

    consteval static Cell null() {
        Hash value;
        value.fill(0);
        return { value };
    }

    constexpr static Cell cons(const Cell& left, const Cell& right) {
        return { Jive::compress(left.value, right.value) };
    }

    constexpr Cell car(const Cell& left, const Cell& right) const {
        if (Jive::compress(left.value, right.value) == value)
            return left;
        else
            throw std::runtime_error("Invalid KAR");
    }

    constexpr Cell cdr(const Cell& left, const Cell& right) const {
        if (Jive::compress(left.value, right.value) == value)
            return right;
        else
            throw std::runtime_error("Invalid KUDER");
    }

    friend std::ostream& operator << (std::ostream& out, const Cell& val)
    {
        fmt::print(out, "{}", val.value);
        return out;
    }

template<typename Builder>
requires(std::same_as<E, typename Builder::R>)
struct Circuit {
    using LinearCombination = Builder::LinearCombination;
    using Hash = typename Jive::HashCircuit<Builder>;

    Builder* circuit;
    Hash value;

    constexpr Circuit(Builder* circuit, const Hash& value)
        : circuit(circuit), value(value) {}

    constexpr static Circuit null(Builder* circuit) {
        return { circuit, {} };
    }

    constexpr static Circuit cons(Builder* circuit, const Circuit& left, const Circuit& right) {
        auto scope = circuit->scope("Cell::cons");
        auto hash = Jive::template Circuit<Builder>::compress(circuit, left.value, right.value);
        return { circuit, hash };
    }

    constexpr Circuit car(const Circuit& left, const Circuit& right) const {
        auto scope = circuit->scope("Cell::car");
        auto hash = Jive::template Circuit<Builder>::compress(circuit, left.value, right.value);
        for (const auto& [x, y] : std::views::zip(hash, value))
            scope(x == y);
        return left;
    }

    constexpr Circuit cdr(const Circuit& left, const Circuit& right) const {
        auto scope = circuit->scope("Cell::cdr");
        auto hash = Jive::template Circuit<Builder>::compress(circuit, left.value, right.value);
        for (const auto& [x, y] : std::views::zip(hash, value))
            scope(x == y);
        return right;
    }
};

template<std::size_t Degree>
struct Assigner {
    Cell cell;
    std::vector<E>* assigment;

    constexpr static Cell cons(const Cell& left, const Cell& right, std::vector<E>* assigment) {
        return { Jive::template Assigner<Degree>::compress(left.value, right.value, assigment) };
    }

    constexpr Cell car(const Cell& left, const Cell& right) const {
        if (Jive::template Assigner<Degree>::compress(left.value, right.value, assigment) == cell.value)
            return left;
        else
            throw std::runtime_error("Invalid KAR");
    }

    constexpr Cell cdr(const Cell& left, const Cell& right) const {
        if (Jive::template Assigner<Degree>::compress(left.value, right.value, assigment) == cell.value)
            return right;
        else
            throw std::runtime_error("Invalid KUDER");
    }
};
};

}

#endif

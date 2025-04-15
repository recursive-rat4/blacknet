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

template<typename Circuit>
requires(std::same_as<E, typename Circuit::R>)
struct Gadget {
    using LinearCombination = Circuit::LinearCombination;
    using Hash = typename Jive::HashGadget<Circuit>;

    Circuit& circuit;
    Hash value;

    constexpr Gadget(Circuit& circuit, const Hash& value)
        : circuit(circuit), value(value) {}

    constexpr static Gadget null(Circuit& circuit) {
        return { circuit, {} };
    }

    constexpr static Gadget cons(Circuit& circuit, const Gadget& left, const Gadget& right) {
        auto scope = circuit.scope("Cell::cons");
        auto hash = Jive::template circuit<Circuit>::compress(circuit, left.value, right.value);
        return { circuit, hash };
    }

    constexpr Gadget car(const Gadget& left, const Gadget& right) const {
        auto scope = circuit.scope("Cell::car");
        auto hash = Jive::template circuit<Circuit>::compress(circuit, left.value, right.value);
        for (const auto& [x, y] : std::views::zip(hash, value))
            circuit(x == y);
        return left;
    }

    constexpr Gadget cdr(const Gadget& left, const Gadget& right) const {
        auto scope = circuit.scope("Cell::cdr");
        auto hash = Jive::template circuit<Circuit>::compress(circuit, left.value, right.value);
        for (const auto& [x, y] : std::views::zip(hash, value))
            circuit(x == y);
        return right;
    }
};

template<std::size_t circuit>
struct Tracer {
    Cell& cell;
    std::vector<E>& trace;

    constexpr static Cell cons(const Cell& left, const Cell& right, std::vector<E>& trace) {
        return { Jive::template trace<circuit>::compress(left.value, right.value, trace) };
    }

    constexpr Cell car(const Cell& left, const Cell& right) const {
        if (Jive::template trace<circuit>::compress(left.value, right.value, trace) == cell.value)
            return left;
        else
            throw std::runtime_error("Invalid KAR");
    }

    constexpr Cell cdr(const Cell& left, const Cell& right) const {
        if (Jive::template trace<circuit>::compress(left.value, right.value, trace) == cell.value)
            return right;
        else
            throw std::runtime_error("Invalid KUDER");
    }
};
};

}

#endif

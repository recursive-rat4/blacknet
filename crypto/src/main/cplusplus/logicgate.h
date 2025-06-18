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

#ifndef BLACKNET_CRYPTO_LOGICGATE_H
#define BLACKNET_CRYPTO_LOGICGATE_H

#include "vector.h"

namespace blacknet::crypto {

template<typename R>
struct LogicGate {
    constexpr R Xor(const R& a, const R& b) {
        return a + b - (a * b).douple();
    }
    constexpr R And(const R& a, const R& b) {
        return a * b;
    }
    constexpr R Or(const R& a, const R& b) {
        return a + b - a * b;
    }
    constexpr R Not(const R& a) {
        return R(1) - a;
    }

template<typename Builder>
requires(std::same_as<R, typename Builder::R>)
struct Circuit {
    using Variable = Builder::Variable;
    using LinearCombination = Builder::LinearCombination;
    using Vector = Vector<R>::template Circuit<Builder>;

    Builder& circuit;

    constexpr Circuit(Builder& circuit) : circuit(circuit) {}

    constexpr void RangeCheck(const LinearCombination& a) {
        auto scope = circuit.scope("LogicGate::RangeCheck");
        circuit(R(0) == a * (a - R(1)));
    }
    constexpr void RangeCheck(const Vector& a) {
        auto scope = circuit.scope("LogicGate::RangeCheck");
        for (const auto& i : a) {
            circuit(R(0) == i * (i - R(1)));
        }
    }

    constexpr LinearCombination Xor(const LinearCombination& a, const LinearCombination& b) {
        auto scope = circuit.scope("LogicGate::Xor");
        LinearCombination ab = circuit.auxiliary();
        circuit(ab == a * b);
        return a + b - ab * R(2);
    }

    constexpr LinearCombination And(const LinearCombination& a, const LinearCombination& b) {
        auto scope = circuit.scope("LogicGate::And");
        LinearCombination ab = circuit.auxiliary();
        circuit(ab == a * b);
        return ab;
    }

    constexpr LinearCombination Or(const LinearCombination& a, const LinearCombination& b) {
        auto scope = circuit.scope("LogicGate::Or");
        LinearCombination ab = circuit.auxiliary();
        circuit(ab == a * b);
        return a + b - ab;
    }

    constexpr LinearCombination Not(const LinearCombination& a) {
        return R(1) - a;
    }
};

struct Tracer {
    std::vector<R>& trace;

    constexpr Tracer(std::vector<R>& trace)
        : trace(trace) {}

    constexpr R Xor(const R& a, const R& b) {
        return a + b - trace.emplace_back(
            a * b
        ).douple();
    }
    constexpr R And(const R& a, const R& b) {
        return trace.emplace_back(
            a * b
        );
    }
    constexpr R Or(const R& a, const R& b) {
        return a + b - trace.emplace_back(
            a * b
        );
    }
    constexpr R Not(const R& a) {
        return R(1) - a;
    }
};

};

}

#endif

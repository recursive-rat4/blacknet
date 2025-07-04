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

#include <optional>

#include "vectordense.h"

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
    using VectorDenseCircuit = VectorDense<R>::template Circuit<Builder>;

    Builder* circuit;

    constexpr Circuit(Builder* circuit) : circuit(circuit) {}

    constexpr void RangeCheck(const LinearCombination& a) {
        auto scope = circuit->scope("LogicGate::RangeCheck");
        scope(R(0) == a * (a - R(1)));
    }
    constexpr void RangeCheck(const VectorDenseCircuit& a) {
        auto scope = circuit->scope("LogicGate::RangeCheck");
        for (const auto& i : a) {
            scope(R(0) == i * (i - R(1)));
        }
    }

    constexpr void LessOrEqualCheck(const VectorDenseCircuit& a, const VectorDense<R>& b) {
        auto scope = circuit->scope("LogicGate::LessOrEqualCheck");
        VectorDenseCircuit current_run(circuit);
        std::optional<LinearCombination> last_run;
        for (std::size_t i = b.size(); i --> 0;) {
            const auto& digit = a[i];
            if (b[i] == R(1)) {
                scope(R(0) == digit * (digit - R(1)));
                current_run.elements.push_back(digit);
            } else {
                if (!current_run.elements.empty()) {
                    if (last_run.has_value()) {
                        current_run.elements.push_back(std::move(*last_run));
                    }
                    last_run = And(current_run);
                    current_run.elements.clear();
                }
                if (last_run.has_value()) {
                    scope(R(0) == digit * (digit - R(1) + *last_run));
                } else {
                    scope(R(0) == digit);
                }
            }
        }
    }

    constexpr LinearCombination Xor(const LinearCombination& a, const LinearCombination& b) {
        auto scope = circuit->scope("LogicGate::Xor");
        LinearCombination ab = circuit->auxiliary();
        scope(ab == a * b);
        return a + b - ab * R(2);
    }

    constexpr LinearCombination And(const LinearCombination& a, const LinearCombination& b) {
        auto scope = circuit->scope("LogicGate::And");
        LinearCombination ab = circuit->auxiliary();
        scope(ab == a * b);
        return ab;
    }
    constexpr LinearCombination And(const VectorDenseCircuit& a) {
        if (a.size() == 1) return a[0];
        auto scope = circuit->scope("LogicGate::And");
        LinearCombination pi = R::multiplicative_identity();
        for (const auto& i : a) {
            LinearCombination p = circuit->auxiliary();
            scope(p == pi * i);
            pi = p;
        }
        return pi;
    }

    constexpr LinearCombination Or(const LinearCombination& a, const LinearCombination& b) {
        auto scope = circuit->scope("LogicGate::Or");
        LinearCombination ab = circuit->auxiliary();
        scope(ab == a * b);
        return a + b - ab;
    }

    constexpr LinearCombination Not(const LinearCombination& a) {
        return R(1) - a;
    }
};

template<std::size_t Degree>
struct Assigner {
    std::vector<R>* assigment;

    constexpr void LessOrEqualCheck(const VectorDense<R>& a, const VectorDense<R>& b) {
        VectorDense<R> current_run;
        std::optional<R> last_run;
        for (std::size_t i = b.size(); i --> 0;) {
            const auto& digit = a[i];
            if (b[i] == R(1)) {
                current_run.elements.push_back(digit);
            } else {
                if (!current_run.elements.empty()) {
                    if (last_run.has_value()) {
                        current_run.elements.push_back(std::move(*last_run));
                    }
                    last_run = And(current_run);
                    current_run.elements.clear();
                }
            }
        }
    }

    constexpr R Xor(const R& a, const R& b) {
        return a + b - assigment->emplace_back(
            a * b
        ).douple();
    }

    constexpr R And(const R& a, const R& b) {
        return assigment->emplace_back(
            a * b
        );
    }
    constexpr R And(const VectorDense<R>& a) {
        if (a.size() == 1) return a[0];
        R pi = R::multiplicative_identity();
        for (const auto& i : a) {
            pi = And(pi, i);
        }
        return pi;
    }

    constexpr R Or(const R& a, const R& b) {
        return a + b - assigment->emplace_back(
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

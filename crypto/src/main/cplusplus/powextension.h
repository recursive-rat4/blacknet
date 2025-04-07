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

#ifndef BLACKNET_CRYPTO_POWEXTENSION_H
#define BLACKNET_CRYPTO_POWEXTENSION_H

#include <ostream>

#include "eqextension.h"

namespace blacknet::crypto {

template<typename E>
class PowExtension {
    EqExtension<E> eq;
public:
    constexpr PowExtension(const E& tau, std::size_t ell) : eq(powers(tau, ell)) {}
    constexpr PowExtension(const EqExtension<E>& eq) : eq(eq) {}
    constexpr PowExtension(EqExtension<E>&& eq) : eq(std::move(eq)) {}

    constexpr static std::vector<E> powers(const E& tau, std::size_t ell) {
        std::vector<E> coefficients(ell);
        coefficients[0] = tau;
        for (std::size_t i = 1; i < ell; ++i)
            coefficients[i] = coefficients[i - 1].square();
        return coefficients;
    }

    constexpr std::vector<E> operator () () const {
        return eq();
    }

    constexpr E operator () (const std::vector<E>& point) const {
        return eq(point);
    }

    constexpr PowExtension& operator *= (const E& other) {
        eq *= other;
        return *this;
    }

    constexpr PowExtension operator * (const E& other) const {
        return PowExtension(eq * other);
    }

    template<E e, typename Fuse>
    constexpr void bind(std::vector<E>& hypercube) const {
        return eq.template bind<e, Fuse>(hypercube);
    }

    constexpr void bind(const E& e) {
        eq.bind(e);
    }

    consteval std::size_t degree() const {
        return eq.degree();
    }

    constexpr std::size_t variables() const {
        return eq.variables();
    }

    template<typename S>
    constexpr PowExtension<S> homomorph() const {
        return PowExtension<S>(eq.template homomorph<S>());
    }

    friend std::ostream& operator << (std::ostream& out, const PowExtension& val)
    {
        return out << '(' << val.eq << ')';
    }

template<typename Circuit>
requires(std::same_as<E, typename Circuit::R>)
struct circuit {
    using Variable = Circuit::Variable;
    using LinearCombination = Circuit::LinearCombination;

    template<std::size_t ell>
    constexpr static std::array<LinearCombination, ell> powers(
        Circuit& circuit,
        const LinearCombination& tau
    ) {
        auto scope = circuit.scope("PowExtension::powers");
        std::array<LinearCombination, ell> coefficients;
        coefficients[0] = tau;
        for (std::size_t i = 1; i < ell; ++i) {
            auto cs = circuit.auxiliary();
            circuit(cs == coefficients[i - 1] * coefficients[i - 1]);
            coefficients[i] = cs;
        }
        return coefficients;
    }
};

struct trace {
    constexpr static std::vector<E> powers(const E& tau, std::size_t ell, std::vector<E>& trace) {
        std::vector<E> coefficients(ell);
        coefficients[0] = tau;
        for (std::size_t i = 1; i < ell; ++i)
            coefficients[i] = trace.emplace_back(
                coefficients[i - 1].square()
            );
        return coefficients;
    }
};

};

}

#endif

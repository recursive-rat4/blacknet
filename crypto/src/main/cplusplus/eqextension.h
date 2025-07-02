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

#ifndef BLACKNET_CRYPTO_EQEXTENSION_H
#define BLACKNET_CRYPTO_EQEXTENSION_H

#include <vector>

#include "point.h"

namespace blacknet::crypto {

template<typename E>
struct EqExtension {
    std::vector<E> coefficients;
    E z;

    constexpr EqExtension(const std::vector<E>& coefficients)
        : coefficients(coefficients), z(E::multiplicative_identity()) {}
    constexpr EqExtension(const std::vector<E>& coefficients, const E& z)
        : coefficients(coefficients), z(z) {}
    constexpr EqExtension(std::vector<E>&& coefficients)
        : coefficients(std::move(coefficients)), z(E::multiplicative_identity()) {}
    constexpr EqExtension(std::vector<E>&& coefficients, E&& z)
        : coefficients(std::move(coefficients)), z(std::move(z)) {}

    constexpr static std::vector<E> evaluate(
        const std::vector<E>& coefficients,
        const E& z = E::multiplicative_identity(),
        const std::size_t offset = 0
    ) {
        std::vector<E> r(1 << (coefficients.size() - offset), E::additive_identity());
        r[0] = z;
        for (std::size_t i = coefficients.size() - offset, j = 1; i --> 0; j <<= 1) {
            for (std::size_t k = 0, l = j; k < j && l < j << 1; ++k, ++l) {
                r[l] = r[k] * coefficients[i + offset];
                r[k] -= r[l];
            }
        }
        return r;
    }

    constexpr std::vector<E> operator () () const {
        return evaluate(coefficients, z);
    }

    constexpr E operator () (const Point<E>& point) const {
        E pi(z);
        for (std::size_t i = 0; i < coefficients.size(); ++i)
            pi *= (coefficients[i] * point[i]).douple() - coefficients[i] - point[i] + E(1);
        return pi;
    }

    constexpr EqExtension& operator *= (const E& other) {
        z *= other;
        return *this;
    }

    constexpr EqExtension operator * (const E& other) const {
        return EqExtension(coefficients, z * other);
    }

    template<E e, typename Fuse>
    constexpr void bind(std::vector<E>& hypercube) const {
        E ze;
        if constexpr (e == E(-2)) {
            ze = z * (E(3) - coefficients[0] - coefficients[0].douple().douple());
        } else if constexpr (e == E(-1)) {
            ze = z * (E(2) - coefficients[0] - coefficients[0].douple());
        } else if constexpr (e == E(0)) {
            ze = z * (E(1) - coefficients[0]);
        } else if constexpr (e == E(1)) {
            ze = z * coefficients[0];
        } else if constexpr (e == E(2)) {
            ze = z * (coefficients[0].douple() + coefficients[0] - E(1));
        } else if constexpr (e == E(3)) {
            ze = z * (coefficients[0].douple().douple() + coefficients[0] - E(2));
        } else if constexpr (e == E(4)) {
            ze = z * (coefficients[0].douple().douple().douple() - coefficients[0] - E(3));
        } else {
            static_assert(false);
        }
        Fuse::call(hypercube, evaluate(coefficients, ze, 1));
    }

    constexpr void bind(const E& e) {
        z *= (coefficients[0] * e).douple() - coefficients[0] - e + E(1);
        coefficients.erase(coefficients.begin());
    }

    consteval std::size_t degree() const {
        return 1;
    }

    constexpr std::size_t variables() const {
        return coefficients.size();
    }

    friend std::ostream& operator << (std::ostream& out, const EqExtension& val)
    {
        return out << '(' << val.coefficients << ", " << val.z << ')';
    }

template<typename Builder>
requires(std::same_as<E, typename Builder::R>)
struct Circuit {
    using Variable = Builder::Variable;
    using LinearCombination = Builder::LinearCombination;
    using Point = Point<E>::template Circuit<Builder>;

    Builder& circuit;
    std::vector<LinearCombination> coefficients;

    constexpr Circuit(Builder& circuit, Variable::Type type, std::size_t variables)
        : circuit(circuit), coefficients(variables)
    {
        std::ranges::generate(coefficients, [&]{ return circuit.variable(type); });
    }

    constexpr LinearCombination operator () (const Point& point) const {
        auto scope = circuit.scope("EqExtension::point");
        LinearCombination pi = E::multiplicative_identity();
        for (std::size_t i = 0; i < coefficients.size(); ++i) {
            LinearCombination cp(circuit.auxiliary());
            circuit(cp == coefficients[i] * point[i]);
            auto t = circuit.auxiliary();
            circuit(t == pi * (cp * E(2) - coefficients[i] - point[i] + E(1)));
            pi = t;
        }
        return pi;
    }

    constexpr static std::vector<LinearCombination> hypercube(
        Builder& circuit,
        const std::vector<LinearCombination>& coefficients
    ) {
        auto scope = circuit.scope("EqExtension::hypercube");
        std::vector<LinearCombination> r;
        r.resize(1 << coefficients.size());
        r[0] = E::multiplicative_identity();
        for (std::size_t i = coefficients.size(), j = 1; i --> 0; j <<= 1) {
            for (std::size_t k = 0, l = j; k < j && l < j << 1; ++k, ++l) {
                auto t = circuit.auxiliary();
                circuit(t == r[k] * coefficients[i]);
                r[l] = t;
                r[k] -= r[l];
            }
        }
        return r;
    }

    consteval std::size_t degree() const {
        return 1;
    }

    constexpr std::size_t variables() const {
        return coefficients.size();
    }
};

template<std::size_t Degree>
struct Assigner {
    std::vector<E>& assigment;

    constexpr E point(const EqExtension& eq, const Point<E>& point) {
        E pi = E::multiplicative_identity();
        for (std::size_t i = 0; i < eq.coefficients.size(); ++i)
            assigment.push_back(
                pi *= assigment.emplace_back(
                    eq.coefficients[i] * point[i]
                ).douple() - eq.coefficients[i] - point[i] + E(1)
            );
        return pi;
    }

    constexpr std::vector<E> hypercube(const std::vector<E>& coefficients) {
        std::vector<E> r(1 << coefficients.size(), E::additive_identity());
        r[0] = E::multiplicative_identity();
        for (std::size_t i = coefficients.size(), j = 1; i --> 0; j <<= 1) {
            for (std::size_t k = 0, l = j; k < j && l < j << 1; ++k, ++l) {
                r[l] = assigment.emplace_back(
                    r[k] * coefficients[i]
                );
                r[k] -= r[l];
            }
        }
        return r;
    }
};

};

}

#endif

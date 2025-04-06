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

#ifndef BLACKNET_CRYPTO_UNIVARIATEPOLYNOMIAL_H
#define BLACKNET_CRYPTO_UNIVARIATEPOLYNOMIAL_H

#include <concepts>
#include <initializer_list>
#include <ostream>
#include <vector>

#include "util.h"

template<typename E>
class UnivariatePolynomial {
public:
    std::vector<E> coefficients;

    consteval UnivariatePolynomial() : coefficients() {}
    constexpr UnivariatePolynomial(std::size_t size) : coefficients(size) {}
    constexpr UnivariatePolynomial(std::initializer_list<E> init) : coefficients(init) {}
    constexpr UnivariatePolynomial(std::vector<E>&& coefficients) : coefficients(std::move(coefficients)) {}
    constexpr UnivariatePolynomial(const UnivariatePolynomial& other)
        : coefficients(other.coefficients) {}
    constexpr UnivariatePolynomial(UnivariatePolynomial&& other) noexcept
        : coefficients(std::move(other.coefficients)) {}

    constexpr UnivariatePolynomial& operator = (UnivariatePolynomial&& other) noexcept {
        coefficients = std::move(other.coefficients);
        return *this;
    }

    constexpr bool operator == (const UnivariatePolynomial&) const = default;

    constexpr E operator () (const E& point) const {
        E sigma(coefficients[0]);
        E pi(point);
        for (std::size_t i = 1; i < coefficients.size() - 1; ++i) {
            sigma += pi * coefficients[i];
            pi *= point;
        }
        if (coefficients.size() > 1) {
            sigma += pi * coefficients.back();
        }
        return sigma;
    }

    constexpr std::size_t degree() const {
        return coefficients.size() - 1;
    }

    consteval std::size_t variables() const {
        return 1;
    }

    template<typename S>
    constexpr UnivariatePolynomial<S> homomorph() const {
        std::vector<S> t;
        t.reserve(coefficients.size());
        for (const auto& i : coefficients)
            t.emplace_back(i);
        return UnivariatePolynomial<S>(std::move(t));
    }

    friend std::ostream& operator << (std::ostream& out, const UnivariatePolynomial& val)
    {
        return out << val.coefficients;
    }

    template<typename DRG>
    constexpr void absorb(DRG& drg) const {
        for (std::size_t i = 0; i < coefficients.size(); ++i)
            coefficients[i].absorb(drg);
    }

template<typename Circuit>
requires(std::same_as<E, typename Circuit::R>)
struct Gadget {
    using Variable = Circuit::Variable;
    using LinearCombination = Circuit::LinearCombination;

    Circuit& circuit;
    std::vector<LinearCombination> coefficients;

    constexpr Gadget(Circuit& circuit, const std::vector<LinearCombination>& coefficients)
        : circuit(circuit), coefficients(coefficients) {}
    constexpr Gadget(Circuit& circuit, std::vector<LinearCombination>&& coefficients)
        : circuit(circuit), coefficients(std::move(coefficients)) {}

    constexpr LinearCombination operator () (const LinearCombination& point) const {
        auto scope = circuit.scope("UnivariatePolynomial::point");
        LinearCombination pi(point);
        std::vector<Variable> cppm(coefficients.size() - 1);
        for (std::size_t i = 1; i < coefficients.size() - 1; ++i) {
            cppm[i - 1] = circuit.auxiliary();
            circuit(cppm[i - 1] == pi * coefficients[i]);
            Variable t(circuit.auxiliary());
            circuit(t == pi * point);
            pi = t;
        }
        if (coefficients.size() > 1) {
            cppm.back() = circuit.auxiliary();
            circuit(cppm.back() == pi * coefficients.back());
        }
        LinearCombination lc(coefficients[0]);
        for (std::size_t i = 0; i < cppm.size(); ++i)
            lc += cppm[i];
        return lc;
    }
};

struct Tracer {
    UnivariatePolynomial& polynomial;
    std::vector<E>& trace;

    constexpr Tracer(UnivariatePolynomial& polynomial, std::vector<E>& trace)
        : polynomial(polynomial), trace(trace) {}

    constexpr E operator () (const E& point) const {
        E sigma(polynomial.coefficients[0]);
        E pi(point);
        for (std::size_t i = 1; i < polynomial.coefficients.size() - 1; ++i) {
            sigma += trace.emplace_back(
                pi * polynomial.coefficients[i]
            );
            trace.push_back(
                pi *= point
            );
        }
        if (polynomial.coefficients.size() > 1) {
            sigma += trace.emplace_back(
                pi * polynomial.coefficients.back()
            );
        }
        return sigma;
    }
};

};

#endif

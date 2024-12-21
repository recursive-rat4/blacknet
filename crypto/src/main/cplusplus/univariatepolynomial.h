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

#ifndef BLACKNET_CRYPTO_UNIVARIATEPOLYNOMIAL_H
#define BLACKNET_CRYPTO_UNIVARIATEPOLYNOMIAL_H

#include <array>
#include <concepts>
#include <initializer_list>
#include <iostream>
#include <ranges>
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
            t.emplace_back(S(i));
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
struct circuit {
    using Variable = Circuit::Variable;
    using LinearCombination = Circuit::LinearCombination;

    template<std::size_t N>
    constexpr static void evaluate(
        Circuit& circuit,
        const std::array<Variable, N>& coefficients,
        Variable& x
    ) {
        auto scope = circuit.scope("UnivariatePolynomial::evaluate");
        Variable pi(x);
        std::array<Variable, coefficients.size() - 1> cxpm;
        for (std::size_t i = 1; i < coefficients.size() - 1; ++i) {
            cxpm[i - 1] = circuit.auxiliary();
            circuit(cxpm[i - 1] == pi * coefficients[i]);
            Variable t(circuit.auxiliary());
            circuit(t == pi * x);
            pi = t;
        }
        if (coefficients.size() > 1) {
            cxpm.back() = circuit.auxiliary();
            circuit(cxpm.back() == pi * coefficients.back());
        }
        LinearCombination lc;
        lc += coefficients[0];
        for (std::size_t i = 0; i < cxpm.size(); ++i)
            lc += cxpm[i];
        Variable y(circuit.auxiliary());
        circuit(y == lc);
        x = y;
    }
};

struct trace {
    constexpr static void evaluate(const UnivariatePolynomial& p, const E& x, std::vector<E>& trace) {
        E sigma(p.coefficients[0]);
        E pi(x);
        for (std::size_t i = 1; i < p.coefficients.size() - 1; ++i) {
            sigma += trace.emplace_back(
                pi * p.coefficients[i]
            );
            trace.push_back(
                pi *= x
            );
        }
        if (p.coefficients.size() > 1) {
            sigma += trace.emplace_back(
                pi * p.coefficients.back()
            );
        }
        trace.emplace_back(std::move(sigma));
    }
};

};

#endif

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

#include <algorithm>
#include <concepts>
#include <functional>
#include <initializer_list>
#include <ostream>
#include <vector>
#include <fmt/format.h>
#include <fmt/ostream.h>
#include <fmt/ranges.h>

namespace blacknet::crypto {

template<typename E>
class UnivariatePolynomial {
public:
    std::vector<E> coefficients;

    consteval UnivariatePolynomial() = default;
    constexpr UnivariatePolynomial(std::size_t size) : coefficients(size) {}
    constexpr UnivariatePolynomial(std::initializer_list<E> init) : coefficients(init) {}
    constexpr UnivariatePolynomial(std::vector<E>&& coefficients) : coefficients(std::move(coefficients)) {}
    constexpr UnivariatePolynomial(const UnivariatePolynomial&) = default;
    constexpr UnivariatePolynomial(UnivariatePolynomial&&) noexcept = default;
    constexpr ~UnivariatePolynomial() noexcept = default;

    constexpr UnivariatePolynomial& operator = (const UnivariatePolynomial&) = default;
    constexpr UnivariatePolynomial& operator = (UnivariatePolynomial&&) noexcept = default;

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

    constexpr E at_0_plus_1() const {
        return std::ranges::fold_left(coefficients, coefficients[0], std::plus<E>{});
    }

    constexpr std::size_t degree() const {
        return coefficients.size() - 1;
    }

    consteval std::size_t variables() const {
        return 1;
    }

    friend std::ostream& operator << (std::ostream& out, const UnivariatePolynomial& val)
    {
        fmt::print(out, "{}", val.coefficients);
        return out;
    }

    template<typename Sponge>
    constexpr void absorb(Sponge& sponge) const {
        for (std::size_t i = 0; i < coefficients.size(); ++i)
            coefficients[i].absorb(sponge);
    }

template<typename Builder>
requires(std::same_as<E, typename Builder::R>)
struct Circuit {
    using Variable = Builder::Variable;
    using LinearCombination = Builder::LinearCombination;

    Builder& circuit;
    std::vector<LinearCombination> coefficients;

    constexpr Circuit(Builder& circuit, Variable::Type type, std::size_t degree)
        : circuit(circuit), coefficients(degree + 1)
    {
        std::ranges::generate(coefficients, [&]{ return circuit.variable(type); });
    }
    constexpr Circuit(Builder& circuit, const std::vector<LinearCombination>& coefficients)
        : circuit(circuit), coefficients(coefficients) {}
    constexpr Circuit(Builder& circuit, std::vector<LinearCombination>&& coefficients)
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

    constexpr LinearCombination at_0_plus_1() const {
        return std::ranges::fold_left(coefficients, coefficients[0], std::plus<LinearCombination>{});
    }

    template<typename Sponge>
    constexpr void absorb(Sponge& sponge) const {
        for (std::size_t i = 0; i < coefficients.size(); ++i)
            coefficients[i].absorb(sponge);
    }
};

struct Tracer {
    UnivariatePolynomial polynomial;
    std::vector<E>& trace;

    constexpr Tracer(const UnivariatePolynomial& polynomial, std::vector<E>& trace)
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

    constexpr E at_0_plus_1() const {
        return polynomial.at_0_plus_1();
    }

    constexpr std::size_t degree() const {
        return polynomial.degree();
    }

    consteval std::size_t variables() const {
        return polynomial.variables();
    }

    template<typename Sponge>
    constexpr void absorb(Sponge& sponge) const {
        polynomial.absorb(sponge);
    }
};

};

}

#endif

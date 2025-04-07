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

#ifndef BLACKNET_CRYPTO_MULTILINEAREXTENSION_H
#define BLACKNET_CRYPTO_MULTILINEAREXTENSION_H

#include <cmath>
#include <concepts>
#include <initializer_list>
#include <ostream>
#include <vector>

#include "eqextension.h"
#include "matrix.h"
#include "polynomialring.h"
#include "util.h"
#include "vector.h"

namespace blacknet::crypto {

template<typename E>
struct MultilinearExtension {
    std::vector<E> coefficients;

    consteval MultilinearExtension() : coefficients() {}
    constexpr MultilinearExtension(std::size_t size) : coefficients(size) {}
    constexpr MultilinearExtension(std::initializer_list<E> init) : coefficients(init) {}
    constexpr MultilinearExtension(std::vector<E>&& coefficients) : coefficients(std::move(coefficients)) {}
    constexpr MultilinearExtension(const Matrix<E>& matrix) : coefficients(matrix.elements) {}
    template<typename Params>
    requires(std::same_as<E, typename Params::Z>)
    constexpr MultilinearExtension(const PolynomialRing<Params>& polynomial) {
        coefficients.assign(polynomial.coefficients.cbegin(), polynomial.coefficients.cend());
    }
    constexpr MultilinearExtension(const Vector<E>& vector) : coefficients(vector.elements) {}
    template<typename Params>
    requires(std::same_as<E, typename Params::Z>)
    constexpr MultilinearExtension(const Vector<PolynomialRing<Params>>& vector) {
        coefficients.reserve(vector.elements.size() * Params::N);
        for (std::size_t i = 0; i < vector.elements.size(); ++i)
            std::copy(
                vector.elements[i].coefficients.begin(),
                vector.elements[i].coefficients.end(),
                std::back_inserter(coefficients)
            );
    }
    constexpr MultilinearExtension(const MultilinearExtension&) = default;
    constexpr MultilinearExtension(MultilinearExtension&&) noexcept = default;
    constexpr ~MultilinearExtension() noexcept = default;

    constexpr MultilinearExtension& operator = (const MultilinearExtension&) = default;
    constexpr MultilinearExtension& operator = (MultilinearExtension&&) noexcept = default;

    constexpr bool operator == (const MultilinearExtension&) const = default;

    constexpr const std::vector<E>& operator () () const {
        return coefficients;
    }

    constexpr E operator () (const std::vector<E>& point) const {
        const std::vector<E>& pis = EqExtension<E>::evaluate(point);
        E sigma(E::LEFT_ADDITIVE_IDENTITY());
        for (std::size_t i = 0; i < coefficients.size(); ++i)
            sigma += pis[i] * coefficients[i];
        return sigma;
    }

    constexpr MultilinearExtension& operator += (const MultilinearExtension& other) {
        for (std::size_t i = 0; i < coefficients.size(); ++i)
            coefficients[i] += other.coefficients[i];
        return *this;
    }

    constexpr MultilinearExtension operator + (const MultilinearExtension& other) const {
        MultilinearExtension t(coefficients.size());
        for (std::size_t i = 0; i < coefficients.size(); ++i)
            t.coefficients[i] = coefficients[i] + other.coefficients[i];
        return t;
    }

    constexpr MultilinearExtension& operator *= (const E& other) {
        for (std::size_t i = 0; i < coefficients.size(); ++i)
            coefficients[i] *= other;
        return *this;
    }

    constexpr MultilinearExtension operator * (const E& other) const {
        MultilinearExtension t(coefficients.size());
        for (std::size_t i = 0; i < coefficients.size(); ++i)
            t.coefficients[i] = coefficients[i] * other;
        return t;
    }

    constexpr MultilinearExtension& operator -= (const E& other) {
        for (std::size_t i = 0; i < coefficients.size(); ++i)
            coefficients[i] -= other;
        return *this;
    }

    constexpr MultilinearExtension operator - (const E& other) const {
        MultilinearExtension t(coefficients.size());
        for (std::size_t i = 0; i < coefficients.size(); ++i)
            t.coefficients[i] = coefficients[i] - other;
        return t;
    }

    template<E e, typename Fuse>
    constexpr void bind(std::vector<E>& hypercube) const {
        if constexpr (e == E(-2)) {
            for (std::size_t i = 0, j = hypercube.size(); i < hypercube.size(); ++i, ++j)
                Fuse::call(hypercube[i], coefficients[i] + coefficients[i].douple() - coefficients[j].douple());
        } else if constexpr (e == E(-1)) {
            for (std::size_t i = 0, j = hypercube.size(); i < hypercube.size(); ++i, ++j)
                Fuse::call(hypercube[i], coefficients[i].douple() - coefficients[j]);
        } else if constexpr (e == E(0)) {
            for (std::size_t i = 0; i < hypercube.size(); ++i)
                Fuse::call(hypercube[i], coefficients[i]);
        } else if constexpr (e == E(1)) {
            for (std::size_t i = 0, j = hypercube.size(); i < hypercube.size(); ++i, ++j)
                Fuse::call(hypercube[i], coefficients[j]);
        } else if constexpr (e == E(2)) {
            for (std::size_t i = 0, j = hypercube.size(); i < hypercube.size(); ++i, ++j)
                Fuse::call(hypercube[i], coefficients[j].douple() - coefficients[i]);
        } else if constexpr (e == E(3)) {
            for (std::size_t i = 0, j = hypercube.size(); i < hypercube.size(); ++i, ++j)
                Fuse::call(hypercube[i], coefficients[j] + coefficients[j].douple() - coefficients[i].douple());
        } else if constexpr (e == E(4)) {
            for (std::size_t i = 0, j = hypercube.size(); i < hypercube.size(); ++i, ++j)
                Fuse::call(hypercube[i], coefficients[j].douple().douple() - coefficients[i].douple() - coefficients[i]);
        } else {
            static_assert(false);
        }
    }

    constexpr void bind(const E& e) {
        std::size_t ns = coefficients.size() >> 1;
        for (std::size_t i = 0, j = ns; i < ns; ++i, ++j) {
            coefficients[i] = coefficients[i] + e * (coefficients[j] - coefficients[i]);
        }
        coefficients.resize(ns);
    }

    consteval std::size_t degree() const {
        return 1;
    }

    constexpr std::size_t variables() const {
        return std::log2(coefficients.size());
    }

    template<typename S>
    constexpr MultilinearExtension<S> homomorph() const {
        std::vector<S> t;
        t.reserve(coefficients.size());
        for (const auto& i : coefficients)
            t.emplace_back(i);
        return MultilinearExtension<S>(std::move(t));
    }

    friend std::ostream& operator << (std::ostream& out, const MultilinearExtension& val)
    {
        return out << val.coefficients;
    }

template<typename Circuit>
requires(std::same_as<E, typename Circuit::R>)
struct circuit {
    using Variable = Circuit::Variable;
    using LinearCombination = Circuit::LinearCombination;

    template<std::size_t ell>
    constexpr static LinearCombination point(
        Circuit& circuit,
        const std::array<LinearCombination, 1 << ell>& coefficients,
        const std::array<LinearCombination, ell>& point
    ) {
        auto scope = circuit.scope("MultilinearExtension::point");
        auto pis = EqExtension<E>::template circuit<Circuit>::hypercube(circuit, point);
        LinearCombination sigma;
        for (std::size_t i = 0; i < coefficients.size(); ++i) {
            auto pc = circuit.auxiliary();
            circuit(pc == pis[i] * coefficients[i]);
            sigma += pc;
        }
        return sigma;
    }
};

struct trace {
    constexpr static E point(const MultilinearExtension& mle, const std::vector<E>& point, std::vector<E>& trace) {
        const std::vector<E>& pis = EqExtension<E>::trace::hypercube(point, trace);
        E sigma(E::LEFT_ADDITIVE_IDENTITY());
        for (std::size_t i = 0; i < mle.coefficients.size(); ++i)
            sigma += trace.emplace_back(
                pis[i] * mle.coefficients[i]
            );
        return sigma;
    }
};

};

}

#endif

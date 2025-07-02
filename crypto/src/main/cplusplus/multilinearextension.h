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

#include <algorithm>
#include <bit>
#include <concepts>
#include <initializer_list>
#include <ostream>
#include <vector>
#include <fmt/format.h>
#include <fmt/ostream.h>
#include <fmt/ranges.h>

#include "eqextension.h"
#include "matrixdense.h"
#include "point.h"
#include "vector.h"

namespace blacknet::crypto {

template<typename E>
struct MultilinearExtension {
    std::vector<E> coefficients;

    consteval MultilinearExtension() = default;
    constexpr MultilinearExtension(std::size_t size) : coefficients(size) {}
    constexpr MultilinearExtension(std::initializer_list<E> init) : coefficients(init) {}
    constexpr MultilinearExtension(std::vector<E>&& coefficients) : coefficients(std::move(coefficients)) {}
    constexpr MultilinearExtension(const MatrixDense<E>& matrix) : coefficients(matrix.elements) {}
    template<typename S>
    requires(
        std::same_as<E, typename S::BaseRing> ||
        std::same_as<typename E::BaseRing, typename S::BaseRing>
    )
    constexpr MultilinearExtension(const S& structure) {
        //TODO __cpp_lib_containers_ranges >= 202202L
        coefficients.assign(structure.begin(), structure.end());
    }
    constexpr MultilinearExtension(const Vector<E>& vector) : coefficients(vector.elements) {}
    template<typename S>
    requires(
        std::same_as<E, typename S::BaseRing> ||
        std::same_as<typename E::BaseRing, typename S::BaseRing>
    )
    constexpr MultilinearExtension(const Vector<S>& vector) {
        coefficients.reserve(vector.elements.size() * S::size());
        auto inserter = std::back_inserter(coefficients);
        for (std::size_t i = 0; i < vector.elements.size(); ++i)
            std::ranges::copy(
                vector.elements[i],
                inserter
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

    constexpr E operator () (const Point<E>& point) const {
        const std::vector<E>& pis = EqExtension<E>::evaluate(point.coordinates);
        E sigma(E::additive_identity());
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
        [[assume(std::has_single_bit(coefficients.size()))]];
        return std::countr_zero(coefficients.size());
    }

    friend std::ostream& operator << (std::ostream& out, const MultilinearExtension& val)
    {
        fmt::print(out, "{}", val.coefficients);
        return out;
    }

template<typename Builder>
requires(std::same_as<E, typename Builder::R>)
struct Circuit {
    using Variable = Builder::Variable;
    using LinearCombination = Builder::LinearCombination;
    using EqExtension = EqExtension<E>::template Circuit<Builder>;
    using Point = Point<E>::template Circuit<Builder>;

    Builder& circuit;
    std::vector<LinearCombination> coefficients;

    constexpr Circuit(Builder& circuit, Variable::Type type, std::size_t variables)
        : circuit(circuit), coefficients(1 << variables)
    {
        std::ranges::generate(coefficients, [&]{ return circuit.variable(type); });
    }

    constexpr LinearCombination operator () (const Point& point) const {
        auto scope = circuit.scope("MultilinearExtension::point");
        auto pis = EqExtension::hypercube(circuit, point.coordinates);
        LinearCombination sigma;
        for (std::size_t i = 0; i < coefficients.size(); ++i) {
            auto pc = circuit.auxiliary();
            circuit(pc == pis[i] * coefficients[i]);
            sigma += pc;
        }
        return sigma;
    }

    consteval std::size_t degree() const {
        return 1;
    }

    constexpr std::size_t variables() const {
        [[assume(std::has_single_bit(coefficients.size()))]];
        return std::countr_zero(coefficients.size());
    }
};

template<std::size_t Degree>
struct Assigner {
    using EqExtension = EqExtension<E>::template Assigner<Degree>;

    MultilinearExtension mle;
    std::vector<E>& assigment;

    constexpr Assigner(const MultilinearExtension& mle, std::vector<E>& assigment)
        : mle(mle), assigment(assigment) {}

    constexpr E operator () (const Point<E>& point) const {
        const std::vector<E>& pis = EqExtension(assigment).hypercube(point.coordinates);
        E sigma(E::additive_identity());
        for (std::size_t i = 0; i < mle.coefficients.size(); ++i)
            sigma += assigment.emplace_back(
                pis[i] * mle.coefficients[i]
            );
        return sigma;
    }

    consteval std::size_t degree() const {
        return mle.degree();
    }

    constexpr std::size_t variables() const {
        return mle.variables();
    }
};

};

}

#endif

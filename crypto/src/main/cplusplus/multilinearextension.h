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

#ifndef BLACKNET_CRYPTO_MULTILINEAREXTENSION_H
#define BLACKNET_CRYPTO_MULTILINEAREXTENSION_H

#include <cmath>
#include <initializer_list>
#include <iostream>
#include <vector>
#include <boost/io/ostream_joiner.hpp>

#include "eqextension.h"
#include "matrix.h"
#include "polynomialring.h"
#include "vector.h"

template<typename E>
class MultilinearExtension {
    std::vector<E> coefficients;
public:
    consteval MultilinearExtension() : coefficients() {}
    constexpr MultilinearExtension(std::size_t size) : coefficients(size) {}
    constexpr MultilinearExtension(std::initializer_list<E> init) : coefficients(init) {}
    constexpr MultilinearExtension(std::vector<E>&& coefficients) : coefficients(std::move(coefficients)) {}
    constexpr MultilinearExtension(const Matrix<E>& matrix) : coefficients(matrix.elements) {}
    template<auto... A>
    constexpr MultilinearExtension(const PolynomialRing<E, A...>& polynomial) {
        coefficients.assign(polynomial.coefficients.cbegin(), polynomial.coefficients.cend());
    }
    constexpr MultilinearExtension(const Vector<E>& vector) : coefficients(vector.elements) {}
    template<std::size_t N, auto... A>
    constexpr MultilinearExtension(const Vector<PolynomialRing<E, N, A...>>& vector) {
        coefficients.reserve(vector.elements.size() * N);
        for (std::size_t i = 0; i < vector.elements.size(); ++i)
            std::copy(
                vector.elements[i].coefficients.begin(),
                vector.elements[i].coefficients.end(),
                std::back_inserter(coefficients)
            );
    }
    constexpr MultilinearExtension(const MultilinearExtension& other) : coefficients(other.coefficients) {}
    constexpr MultilinearExtension(MultilinearExtension&& other) noexcept
        : coefficients(std::move(other.coefficients)) {}

    constexpr MultilinearExtension& operator = (const MultilinearExtension& other) {
        coefficients = other.coefficients;
        return *this;
    }
    constexpr MultilinearExtension& operator = (MultilinearExtension&& other) {
        coefficients = std::move(other.coefficients);
        return *this;
    }

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
        if constexpr (e == E(0)) {
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
            t.emplace_back(S(i));
        return MultilinearExtension<S>(std::move(t));
    }

    friend std::ostream& operator << (std::ostream& out, const MultilinearExtension& val)
    {
        out << '[';
        std::copy(val.coefficients.begin(), val.coefficients.end(), boost::io::make_ostream_joiner(out, ", "));
        return out << ']';
    }
};

#endif

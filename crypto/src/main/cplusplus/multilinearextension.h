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
#include "vector.h"

template<typename E>
class MultilinearExtension {
public:
    std::vector<E> coefficients;

    constexpr MultilinearExtension(std::size_t size) : coefficients(size) {}
    constexpr MultilinearExtension(std::initializer_list<E> init) : coefficients(init) {}
    constexpr MultilinearExtension(const Matrix<E>& matrix) : coefficients(matrix.elements) {}
    constexpr MultilinearExtension(const Vector<E>& vector) : coefficients(vector.elements) {}
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

    constexpr E operator () (const std::vector<E>& point) const {
        EqExtension eq(point);
        std::vector<E> pis(eq());
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

    template<E e>
    constexpr MultilinearExtension bind() const {
        std::size_t ns = coefficients.size() >> 1;
        MultilinearExtension r(ns);
        if constexpr (e == E(0)) {
            std::copy(coefficients.begin(), coefficients.begin() + ns, r.coefficients.begin());
        } else if constexpr (e == E(1)) {
            std::copy(coefficients.begin() + ns, coefficients.end(), r.coefficients.begin());
        } else if constexpr (e == E(2)) {
            for (std::size_t i = 0, j = ns; i < ns; ++i, ++j) {
                r.coefficients[i] = coefficients[i] + (coefficients[j] - coefficients[i]).douple();
            }
        } else {
            static_assert(false);
        }
        return r;
    }

    constexpr MultilinearExtension bind(const E& e) const {
        std::size_t ns = coefficients.size() >> 1;
        MultilinearExtension r(ns);
        for (std::size_t i = 0, j = ns; i < ns; ++i, ++j) {
            r.coefficients[i] = coefficients[i] + e * (coefficients[j] - coefficients[i]);
        }
        return r;
    }

    constexpr std::size_t degree() const {
        return 1;
    }

    constexpr std::size_t variables() const {
        return std::log2(coefficients.size());
    }

    friend std::ostream& operator << (std::ostream& out, const MultilinearExtension& val)
    {
        out << '[';
        std::copy(val.coefficients.begin(), val.coefficients.end(), boost::io::make_ostream_joiner(out, ", "));
        return out << ']';
    }
};

#endif

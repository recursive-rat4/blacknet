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

#ifndef BLACKNET_CRYPTO_MATRIX_H
#define BLACKNET_CRYPTO_MATRIX_H

#include <initializer_list>
#include <iostream>
#include <vector>
#include <boost/io/ostream_joiner.hpp>

#include "vector.h"

template<typename E>
class Matrix {
    std::size_t rows;
    std::size_t columns;
    std::vector<E> elements;
public:
    constexpr Matrix(std::size_t rows, std::size_t columns)
        : rows(rows), columns(columns), elements(rows * columns) {}
    constexpr Matrix(std::size_t rows, std::size_t columns, std::initializer_list<E> init)
        : rows(rows), columns(columns), elements(init) {}
    constexpr Matrix(Matrix&& other) noexcept
        : rows(other.rows), columns(other.columns), elements(std::move(other.elements)) {}

    constexpr bool operator == (const Matrix&) const = default;

    constexpr E& operator [] (std::size_t i, std::size_t j) {
        return elements[i * columns + j];
    }

    constexpr const E& operator [] (std::size_t i, std::size_t j) const {
        return elements[i * columns + j];
    }

    constexpr Vector<E> operator * (const Vector<E>& other) const {
        Vector<E> r(rows, E::LEFT_ADDITIVE_IDENTITY());
        for (std::size_t i = 0; i < rows; ++i)
            for (std::size_t j = 0; j < columns; ++j)
                r[i] += (*this)[i, j] * other[j];
        return r;
    }

    friend std::ostream& operator << (std::ostream& out, const Matrix& val)
    {
        out << '[';
        std::copy(val.elements.begin(), val.elements.end(), boost::io::make_ostream_joiner(out, ", "));
        return out << ']';
    }
};

#endif

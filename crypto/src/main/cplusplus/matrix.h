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

#ifndef BLACKNET_CRYPTO_MATRIX_H
#define BLACKNET_CRYPTO_MATRIX_H

#include <algorithm>
#include <initializer_list>
#include <ostream>
#include <random>
#include <vector>
#include <fmt/format.h>
#include <fmt/ostream.h>
#include <fmt/ranges.h>

namespace blacknet::crypto {

template<typename E>class Vector;

template<typename E>
class Matrix {
public:
    using ElementType = E;

    std::size_t rows;
    std::size_t columns;
    std::vector<E> elements;

    constexpr Matrix() noexcept = default;
    constexpr Matrix(std::size_t rows, std::size_t columns)
        : rows(rows), columns(columns), elements(rows * columns) {}
    constexpr Matrix(std::size_t rows, std::size_t columns, const E& fill)
        : rows(rows), columns(columns), elements(rows * columns, fill) {}
    constexpr Matrix(std::size_t rows, std::size_t columns, std::initializer_list<E> init)
        : rows(rows), columns(columns), elements(init) {}
    constexpr Matrix(std::size_t rows, std::size_t columns, std::vector<E>&& elements)
        : rows(rows), columns(columns), elements(std::move(elements)) {}
    constexpr Matrix(const Matrix&) = default;
    constexpr Matrix(Matrix&&) noexcept = default;
    constexpr ~Matrix() noexcept = default;

    constexpr Matrix& operator = (const Matrix&) = default;
    constexpr Matrix& operator = (Matrix&&) noexcept = default;

    constexpr bool operator == (const Matrix&) const = default;

    constexpr E& operator [] (std::size_t i, std::size_t j) {
        return elements[i * columns + j];
    }

    constexpr const E& operator [] (std::size_t i, std::size_t j) const {
        return elements[i * columns + j];
    }

    constexpr Matrix operator + (const Matrix& other) const {
        Matrix r(rows, columns);
        for (std::size_t i = 0; i < rows; ++i)
            for (std::size_t j = 0; j < columns; ++j)
                r[i, j] = (*this)[i, j] + other[i, j];
        return r;
    }

    constexpr Matrix operator * (const Matrix& other) const {
        // Iterative algorithm
        Matrix r(rows, other.columns, E::LEFT_ADDITIVE_IDENTITY());
        for (std::size_t i = 0; i < rows; ++i)
            for (std::size_t j = 0; j < other.columns; ++j)
                for (std::size_t k = 0; k < columns; ++k)
                    r[i, j] += (*this)[i, k] * other[k, j];
        return r;
    }

    constexpr Vector<E> operator * (const Vector<E>& other) const {
        Vector<E> r(rows, E::LEFT_ADDITIVE_IDENTITY());
        for (std::size_t i = 0; i < rows; ++i)
            for (std::size_t j = 0; j < columns; ++j)
                r[i] += (*this)[i, j] * other[j];
        return r;
    }

    friend constexpr Vector<E> operator * (const Vector<E>& lps, const Matrix& rps) {
        Vector<E> r(rps.columns, E::LEFT_ADDITIVE_IDENTITY());
        for (std::size_t i = 0; i < rps.rows; ++i)
            for (std::size_t j = 0; j < rps.columns; ++j)
                r[j] += lps[i] * rps[i, j];
        return r;
    }

    constexpr Matrix operator || (const Matrix& other) const {
        Matrix r(rows, columns + other.columns);
        for (std::size_t i = 0; i < rows; ++i) {
            for (std::size_t j = 0; j < columns; ++j)
                r[i, j] = (*this)[i, j];
            for (std::size_t j = 0; j < other.columns; ++j)
                r[i, j + columns] = other[i, j];
        }
        return r;
    }

    constexpr Matrix transpose() const {
        Matrix r(columns, rows);
        for (std::size_t i = 0; i < rows; ++i)
            for (std::size_t j = 0; j < columns; ++j)
                r[j, i] = (*this)[i, j];
        return r;
    }

    constexpr bool checkInfinityNorm(const E::NumericType& bound) const {
        return std::ranges::all_of(elements, [&bound](const E& e) {
            return e.checkInfinityNorm(bound);
        });
    }

    friend std::ostream& operator << (std::ostream& out, const Matrix& val)
    {
        fmt::print(out, "{}", val.elements);
        return out;
    }

    template<std::uniform_random_bit_generator RNG>
    static Matrix random(RNG& rng, std::size_t rows, std::size_t columns) {
        Matrix t(rows, columns);
        std::ranges::generate(t.elements, [&] { return E::random(rng); });
        return t;
    }

    template<std::uniform_random_bit_generator RNG, typename DST>
    static Matrix random(RNG& rng, DST& dst, std::size_t rows, std::size_t columns) {
        Matrix t(rows, columns);
        std::ranges::generate(t.elements, [&] { return E::random(rng, dst); });
        return t;
    }
};

}

#endif

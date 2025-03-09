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

#ifndef BLACKNET_CRYPTO_MATRIXSPARSE_H
#define BLACKNET_CRYPTO_MATRIXSPARSE_H

#include <iostream>
#include <vector>

#include "util.h"

template<typename E>class Matrix;
template<typename E>class Vector;

// https://arxiv.org/abs/2404.06047
// CSR format

template<typename E>
struct MatrixSparse {
    using ElementType = E;

    std::size_t columns;
    std::vector<std::size_t> rIndex;
    std::vector<std::size_t> cIndex;
    std::vector<E> elements;

    constexpr MatrixSparse(std::size_t rows, std::size_t columns) : columns(columns) {
        rIndex.reserve(rows + 1);
        rIndex.push_back(0);
    }
    constexpr MatrixSparse(const Matrix<E>& dense) : columns(dense.columns) {
        rIndex.reserve(dense.rows + 1);
        for (std::size_t i = 0; i < dense.rows; ++i) {
            rIndex.push_back(elements.size());
            for (std::size_t j = 0; j < dense.columns; ++j) {
                if (dense[i, j] != E(0)) {
                    cIndex.push_back(j);
                    elements.push_back(dense[i, j]);
                }
            }
        }
        rIndex.push_back(elements.size());
    }
    constexpr MatrixSparse(
        std::size_t columns,
        const std::vector<std::size_t>& rIndex, const std::vector<std::size_t>& cIndex,
        std::vector<E>&& elements
    ) : columns(columns), rIndex(rIndex), cIndex(cIndex), elements(std::move(elements)) {}
    constexpr MatrixSparse(const MatrixSparse& other)
        : columns(other.columns), rIndex(other.rIndex), cIndex(other.cIndex), elements(other.elements) {}
    constexpr MatrixSparse(MatrixSparse&& other) noexcept
        : columns(other.columns),
          rIndex(std::move(other.rIndex)), cIndex(std::move(other.cIndex)),
          elements(std::move(other.elements)) {}

    constexpr bool operator == (const MatrixSparse&) const = default;

    constexpr std::size_t rows() const {
        return rIndex.size() - 1;
    }

    template<typename S = E>
    constexpr Vector<S> operator * (const Vector<S>& other) const {
        std::size_t rows = rIndex.size() - 1;
        Vector<S> r(rows, S::LEFT_ADDITIVE_IDENTITY());
        for (std::size_t i = 0; i < rows; ++i) {
            std::size_t row_start = rIndex[i];
            std::size_t row_end = rIndex[i + 1];
            for (std::size_t j = row_start; j < row_end; ++j) {
                std::size_t column = cIndex[j];
                r[i] += elements[j] * other[column];
            }
        }
        return r;
    }

    template<typename S>
    constexpr MatrixSparse<S> homomorph() const {
        std::vector<S> t;
        t.reserve(elements.size());
        for (const auto& i : elements)
            t.emplace_back(i);
        return MatrixSparse<S>(columns, rIndex, cIndex, std::move(t));
    }

    constexpr Matrix<E> dense() const {
        std::size_t rows = rIndex.size() - 1;
        Matrix<E> r(rows, columns, E(0));
        for (std::size_t i = 0; i < rows; ++i) {
            std::size_t row_start = rIndex[i];
            std::size_t row_end = rIndex[i + 1];
            for (std::size_t j = row_start; j < row_end; ++j) {
                std::size_t column = cIndex[j];
                r[i, column] = elements[j];
            }
        }
        return r;
    }

    friend std::ostream& operator << (std::ostream& out, const MatrixSparse& val)
    {
        return out << '(' << val.rIndex << ", " << val.cIndex << ", " << val.elements << ')';
    }
};

#endif

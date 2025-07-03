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

#ifndef BLACKNET_CRYPTO_MATRIXDENSE_H
#define BLACKNET_CRYPTO_MATRIXDENSE_H

#include <algorithm>
#include <concepts>
#include <initializer_list>
#include <ostream>
#include <random>
#include <vector>
#include <fmt/format.h>
#include <fmt/ostream.h>
#include <fmt/ranges.h>

namespace blacknet::crypto {

template<typename E>class VectorDense;

template<typename E>
class MatrixDense {
public:
    using ElementType = E;

    std::size_t rows;
    std::size_t columns;
    std::vector<E> elements;

    constexpr MatrixDense() noexcept = default;
    constexpr MatrixDense(std::size_t rows, std::size_t columns)
        : rows(rows), columns(columns), elements(rows * columns) {}
    constexpr MatrixDense(std::size_t rows, std::size_t columns, const E& fill)
        : rows(rows), columns(columns), elements(rows * columns, fill) {}
    constexpr MatrixDense(std::size_t rows, std::size_t columns, std::initializer_list<E> init)
        : rows(rows), columns(columns), elements(init) {}
    constexpr MatrixDense(std::size_t rows, std::size_t columns, std::vector<E>&& elements)
        : rows(rows), columns(columns), elements(std::move(elements)) {}
    constexpr MatrixDense(const MatrixDense&) = default;
    constexpr MatrixDense(MatrixDense&&) noexcept = default;
    constexpr ~MatrixDense() noexcept = default;

    constexpr MatrixDense& operator = (const MatrixDense&) = default;
    constexpr MatrixDense& operator = (MatrixDense&&) noexcept = default;

    constexpr bool operator == (const MatrixDense&) const = default;

    constexpr E& operator [] (std::size_t i, std::size_t j) {
        return elements[i * columns + j];
    }

    constexpr const E& operator [] (std::size_t i, std::size_t j) const {
        return elements[i * columns + j];
    }

    constexpr MatrixDense operator + (const MatrixDense& other) const {
        MatrixDense r(rows, columns);
        for (std::size_t i = 0; i < rows; ++i)
            for (std::size_t j = 0; j < columns; ++j)
                r[i, j] = (*this)[i, j] + other[i, j];
        return r;
    }

    constexpr MatrixDense operator * (const MatrixDense& other) const {
        // Iterative algorithm
        MatrixDense r(rows, other.columns, E::additive_identity());
        for (std::size_t i = 0; i < rows; ++i)
            for (std::size_t j = 0; j < other.columns; ++j)
                for (std::size_t k = 0; k < columns; ++k)
                    r[i, j] += (*this)[i, k] * other[k, j];
        return r;
    }

    constexpr VectorDense<E> operator * (const VectorDense<E>& other) const {
        VectorDense<E> r(rows, E::additive_identity());
        for (std::size_t i = 0; i < rows; ++i)
            for (std::size_t j = 0; j < columns; ++j)
                r[i] += (*this)[i, j] * other[j];
        return r;
    }

    friend constexpr VectorDense<E> operator * (const VectorDense<E>& lps, const MatrixDense& rps) {
        VectorDense<E> r(rps.columns, E::additive_identity());
        for (std::size_t i = 0; i < rps.rows; ++i)
            for (std::size_t j = 0; j < rps.columns; ++j)
                r[j] += lps[i] * rps[i, j];
        return r;
    }

    constexpr MatrixDense operator || (const MatrixDense& other) const {
        MatrixDense r(rows, columns + other.columns);
        for (std::size_t i = 0; i < rows; ++i) {
            for (std::size_t j = 0; j < columns; ++j)
                r[i, j] = (*this)[i, j];
            for (std::size_t j = 0; j < other.columns; ++j)
                r[i, j + columns] = other[i, j];
        }
        return r;
    }

    constexpr E trace() const {
        E sigma = E::additive_identity();
        for (std::size_t i = 0; i < rows; ++i)
            sigma += (*this)[i, i];
        return sigma;
    }

    constexpr MatrixDense transpose() const {
        MatrixDense r(columns, rows);
        for (std::size_t i = 0; i < rows; ++i)
            for (std::size_t j = 0; j < columns; ++j)
                r[j, i] = (*this)[i, j];
        return r;
    }

    template<typename NormType>
    requires(std::same_as<NormType, typename E::NumericType>)
    constexpr bool checkInfinityNorm(const NormType& bound) const {
        return std::ranges::all_of(elements, [&bound](const E& e) {
            return e.checkInfinityNorm(bound);
        });
    }

    friend std::ostream& operator << (std::ostream& out, const MatrixDense& val)
    {
        fmt::print(out, "{}", val.elements);
        return out;
    }

    template<typename Sponge>
    constexpr static MatrixDense squeeze(Sponge& sponge, std::size_t rows, std::size_t columns) {
        MatrixDense t(rows, columns);
        std::ranges::generate(t.elements, [&] { return E::squeeze(sponge); });
        return t;
    }

    template<typename Sponge, typename DST>
    constexpr static MatrixDense squeeze(Sponge& sponge, DST& dst, std::size_t rows, std::size_t columns) {
        MatrixDense t(rows, columns);
        std::ranges::generate(t.elements, [&] { return E::squeeze(sponge, dst); });
        return t;
    }

    template<std::uniform_random_bit_generator RNG>
    static MatrixDense random(RNG& rng, std::size_t rows, std::size_t columns) {
        MatrixDense t(rows, columns);
        std::ranges::generate(t.elements, [&] { return E::random(rng); });
        return t;
    }

    template<std::uniform_random_bit_generator RNG, typename DST>
    static MatrixDense random(RNG& rng, DST& dst, std::size_t rows, std::size_t columns) {
        MatrixDense t(rows, columns);
        std::ranges::generate(t.elements, [&] { return E::random(rng, dst); });
        return t;
    }

template<typename Builder>
requires(std::same_as<E, typename Builder::R>)
struct Circuit {
    using Variable = Builder::Variable;
    using LinearCombination = Builder::LinearCombination;
    using VectorDense = VectorDense<E>::template Circuit<Builder>;

    Builder& circuit;
    std::size_t rows;
    std::size_t columns;
    std::vector<LinearCombination> elements;

    constexpr Circuit(Builder& circuit, std::size_t rows, std::size_t columns)
        : circuit(circuit), rows(rows), columns(columns), elements(rows * columns) {}
    constexpr Circuit(Builder& circuit, Variable::Type type, std::size_t rows, std::size_t columns)
        : circuit(circuit), rows(rows), columns(columns), elements(rows * columns)
    {
        std::ranges::generate(elements, [&]{ return circuit.variable(type); });
    }

    constexpr LinearCombination& operator [] (std::size_t i, std::size_t j) {
        return elements[i * columns + j];
    }

    constexpr const LinearCombination& operator [] (std::size_t i, std::size_t j) const {
        return elements[i * columns + j];
    }

    constexpr VectorDense operator * (const VectorDense& other) const {
        auto scope = circuit.scope("Matrix::vector");
        VectorDense r(circuit, rows);
        for (std::size_t i = 0; i < rows; ++i) {
            for (std::size_t j = 0; j < columns; ++j) {
                auto t = circuit.auxiliary();
                circuit(t == (*this)[i, j] * other[j]);
                r[i] += t;
            }
        }
        return r;
    }
};

template<std::size_t Degree>
struct Assigner {
    using VectorDense = VectorDense<E>::template Assigner<Degree>;

    MatrixDense matrix;
    std::vector<E>& assigment;

    constexpr Assigner(const MatrixDense& matrix, std::vector<E>& assigment)
        : matrix(matrix), assigment(assigment) {}
    constexpr Assigner(MatrixDense&& matrix, std::vector<E>& assigment)
        : matrix(std::move(matrix)), assigment(assigment) {}
    constexpr Assigner(std::size_t rows, std::size_t columns, std::vector<E>& assigment)
        : matrix(rows, columns), assigment(assigment) {}

    constexpr E& operator [] (std::size_t i, std::size_t j) {
        return matrix[i, j];
    }

    constexpr const E& operator [] (std::size_t i, std::size_t j) const {
        return matrix[i, j];
    }

    constexpr VectorDense operator * (const VectorDense& other) const {
        VectorDense r(matrix.rows, E::additive_identity(), assigment);
        for (std::size_t i = 0; i < matrix.rows; ++i)
            for (std::size_t j = 0; j < matrix.columns; ++j)
                r[i] += assigment.emplace_back(
                    matrix[i, j] * other[j]
                );
        return r;
    }
};

};

}

#endif

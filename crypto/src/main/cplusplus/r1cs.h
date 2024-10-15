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

#ifndef BLACKNET_CRYPTO_R1CS_H
#define BLACKNET_CRYPTO_R1CS_H

#include <iostream>
#include <ranges>
#include <vector>

#include "matrix.h"
#include "matrixsparse.h"
#include "vector.h"

template<typename E>
class R1CS {
    MatrixSparse<E> a;
    MatrixSparse<E> b;
    MatrixSparse<E> c;
public:
    constexpr R1CS(const MatrixSparse<E>& a, const MatrixSparse<E>& b, const MatrixSparse<E>& c)
        : a(a), b(b), c(c) {}
    constexpr R1CS(MatrixSparse<E>&& a, MatrixSparse<E>&& b, MatrixSparse<E>&& c)
        : a(std::move(a)), b(std::move(b)), c(std::move(c)) {}
    constexpr R1CS(R1CS&& other) noexcept
        : a(std::move(other.a)), b(std::move(other.b)), c(std::move(other.c)) {}

    constexpr bool operator == (const R1CS&) const = default;

    template<typename S = E>
    constexpr bool isSatisfied(const Vector<S>& z) const {
        return (a * z) * (b * z) == c * z;
    }

    friend std::ostream& operator << (std::ostream& out, const R1CS& val)
    {
        return out << '[' << val.a << ", " << val.b << ", " << val.c << ']';
    }

    class Builder {
        std::vector<std::pair<Matrix<E>, Vector<E>>> as;
        std::vector<std::pair<Matrix<E>, Vector<E>>> bs;
        std::vector<std::pair<Matrix<E>, Vector<E>>> cs;
    public:
        constexpr void append(
            const Matrix<E>& am, const Vector<E>& av,
            const Matrix<E>& bm, const Vector<E>& bv,
            const Matrix<E>& cm, const Vector<E>& cv
        ) {
            as.emplace_back(std::make_pair(am, av));
            bs.emplace_back(std::make_pair(bm, bv));
            cs.emplace_back(std::make_pair(cm, cv));
        }
        constexpr void append(
            Matrix<E>&& am, Vector<E>&& av,
            Matrix<E>&& bm, Vector<E>&& bv,
            Matrix<E>&& cm, Vector<E>&& cv
        ) {
            as.emplace_back(std::make_pair(std::move(am), std::move(av)));
            bs.emplace_back(std::make_pair(std::move(bm), std::move(bv)));
            cs.emplace_back(std::make_pair(std::move(cm), std::move(cv)));
        }

        constexpr R1CS build() {
            return R1CS(matrix(as), matrix(bs), matrix(cs));
        }
    private:
        constexpr static MatrixSparse<E> matrix(const auto& ms) {
            std::size_t rows = std::ranges::fold_left(ms, std::size_t(0), [] (auto&& a, auto&& i) { return a + i.first.rows; });
            std::size_t columns = std::ranges::fold_left(ms, std::size_t(1), [] (auto&& a, auto&& i) { return a + i.first.columns; });
            Matrix<E> l(rows, columns, E(0));
            std::size_t m = 0, n = 1;
            for (const auto& [k, o] : ms) {
                for (std::size_t i = 0; i < k.rows; ++i) {
                    for (std::size_t j = 0; j < k.columns; ++j) {
                        l[m + i, n + j] = k[i, j];
                    }
                    l[m + i, 0] = o[i];
                }
                m += k.rows;
                n += k.columns;
            }
            return MatrixSparse(l);
        }
    };
};

#endif

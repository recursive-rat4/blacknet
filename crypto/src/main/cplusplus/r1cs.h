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
#include "vector.h"

template<typename E>
class R1CS {
    Matrix<E> a;
    Matrix<E> b;
    Matrix<E> c;
public:
    constexpr R1CS(const Matrix<E>& a, const Matrix<E>& b, const Matrix<E>& c)
        : a(a), b(b), c(c) {}
    constexpr R1CS(Matrix<E>&& a, Matrix<E>&& b, Matrix<E>&& c)
        : a(std::move(a)), b(std::move(b)), c(std::move(c)) {}
    constexpr R1CS(R1CS&& other) noexcept
        : a(std::move(other.a)), b(std::move(other.b)), c(std::move(other.c)) {}

    constexpr bool operator == (const R1CS&) const = default;

    constexpr bool isSatisfied(const Vector<E>& z) const {
        return (a * z) * (b * z) == c * z;
    }

    template<typename S>
    constexpr R1CS<S> homomorph() const {
        return R1CS<S>(a.template homomorph<S>(), b.template homomorph<S>(), c.template homomorph<S>());
    }

    friend std::ostream& operator << (std::ostream& out, const R1CS& val)
    {
        return out << '[' << val.a << ", " << val.b << ", " << val.c << ']';
    }

    class Builder {
        std::vector<Matrix<E>> as;
        std::vector<Matrix<E>> bs;
        std::vector<Matrix<E>> cs;
    public:
        constexpr void append(const Matrix<E>& a, const Matrix<E>& b, const Matrix<E>& c) {
            as.push_back(a);
            bs.push_back(b);
            cs.push_back(c);
        }
        constexpr void append(Matrix<E>&& a, Matrix<E>&& b, Matrix<E>&& c) {
            as.emplace_back(std::move(a));
            bs.emplace_back(std::move(b));
            cs.emplace_back(std::move(c));
        }

        constexpr R1CS build() {
            return R1CS(matrix(as), matrix(bs), matrix(cs));
        }
    private:
        constexpr static Matrix<E> matrix(const std::vector<Matrix<E>>& ms) {
            std::size_t rows = std::ranges::fold_left(ms, std::size_t(0), [] (auto&& a, auto&& i) { return a + i.rows; });
            std::size_t columns = std::ranges::fold_left(ms, std::size_t(1), [] (auto&& a, auto&& i) { return a + i.columns; });
            Matrix<E> l(rows, columns, E(0));
            std::size_t m = 0, n = 1;
            for (const auto& k : ms) {
                for (std::size_t i = 0; i < k.rows; ++i) {
                    for (std::size_t j = 0; j < k.columns; ++j) {
                        l[m + i, n + j] = k[i, j];
                    }
                }
                m += k.rows;
                n += k.columns;
            }
            return l;
        }
    };
};

#endif

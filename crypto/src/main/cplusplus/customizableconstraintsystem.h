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

#ifndef BLACKNET_CRYPTO_CUSTOMIZABLECONSTRAINTSYSTEM_H
#define BLACKNET_CRYPTO_CUSTOMIZABLECONSTRAINTSYSTEM_H

#include <vector>

#include "matrixsparse.h"
#include "vector.h"

/*
 * Customizable constraint systems for succinct arguments
 * Srinath Setty, Justin Thaler, Riad Wahby
 * May 3, 2023
 * https://eprint.iacr.org/2023/552
 */

template<typename E>
class CustomizableConstraintSystem {
    std::size_t rows;
    std::size_t columns;
    std::vector<MatrixSparse<E>> m;
    std::vector<std::vector<std::size_t>> s;
    std::vector<E> c;
public:
    constexpr CustomizableConstraintSystem(
        std::size_t rows,
        std::size_t columns,
        std::vector<MatrixSparse<E>>&& m,
        std::vector<std::vector<std::size_t>>&& s,
        std::vector<E>&& c
    ) : rows(rows), columns(columns), m(std::move(m)), s(std::move(s)), c(std::move(c)) {}
    constexpr CustomizableConstraintSystem(CustomizableConstraintSystem&& other) noexcept
        : rows(other.rows), columns(other.columns), m(std::move(other.m)), s(std::move(other.s)), c(std::move(other.c)) {}

    constexpr bool isSatisfied(const Vector<E>& z) const {
        Vector<E> sigma(rows, E::LEFT_ADDITIVE_IDENTITY());
        for (std::size_t i = 0; i < c.size(); ++i) {
            Vector<E> circle(rows, E::LEFT_MULTIPLICATIVE_IDENTITY());
            for (std::size_t j : s[i]) {
                circle *= m[j] * z;
            }
            sigma += circle * c[i];
        }
        return sigma == Vector<E>(rows, E(0));
    }
};

#endif

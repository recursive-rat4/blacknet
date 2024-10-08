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

#include <vector>

#include "matrix.h"
#include "vector.h"

template<typename E>
class R1CS {
    Matrix<E> a;
    Matrix<E> b;
    Matrix<E> c;
public:
    constexpr R1CS(
        Matrix<E>&& a,
        Matrix<E>&& b,
        Matrix<E>&& c
    ) : a(std::move(a)), b(std::move(b)), c(std::move(c)) {}
    constexpr R1CS(R1CS&& other) noexcept
        : a(std::move(other.a)), b(std::move(other.b)), c(std::move(other.c)) {}

    constexpr bool isSatisfied(const Vector<E>& z) const {
        return (a * z) * (b * z) == c * z;
    }

    template<typename S>
    constexpr R1CS<S> homomorph() const {
        return R1CS<S>(a.template homomorph<S>(), b.template homomorph<S>(), c.template homomorph<S>());
    }
};

#endif

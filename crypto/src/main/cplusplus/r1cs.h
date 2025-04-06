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

#ifndef BLACKNET_CRYPTO_R1CS_H
#define BLACKNET_CRYPTO_R1CS_H

#include <ostream>
#include <utility>

#include "matrixsparse.h"
#include "vector.h"

template<typename E>
class R1CS {
    MatrixSparse<E> a;
    MatrixSparse<E> b;
    MatrixSparse<E> c;
public:
    using ElementType = E;

    constexpr R1CS(const MatrixSparse<E>& a, const MatrixSparse<E>& b, const MatrixSparse<E>& c)
        : a(a), b(b), c(c) {}
    constexpr R1CS(MatrixSparse<E>&& a, MatrixSparse<E>&& b, MatrixSparse<E>&& c)
        : a(std::move(a)), b(std::move(b)), c(std::move(c)) {}
    constexpr R1CS(R1CS&& other) noexcept
        : a(std::move(other.a)), b(std::move(other.b)), c(std::move(other.c)) {}

    constexpr bool operator == (const R1CS&) const = default;

    constexpr std::size_t constraints() const {
        return a.rows();
    }

    constexpr std::size_t variables() const {
        return a.columns;
    }

    template<typename S = E>
    constexpr bool isSatisfied(const Vector<S>& z) const {
        return (a * z) * (b * z) == c * z;
    }

    template<typename S = E>
    constexpr bool isSatisfied(const Vector<S>& z, const Vector<S>& e) const {
        return error(z) == e;
    }

    template<typename S = E>
    constexpr void fold(
        const S& r,
        Vector<S>& z, Vector<S>& e,
        const Vector<S>& z1, const Vector<S>& e1,
        const Vector<S>& z2, const Vector<S>& e2
    ) const {
        const S& u1 = z1[0];
        const S& u2 = z2[0];
        Vector<S> z12{ z1 + z2 };
        Vector<S> t{ (a * z12) * (b * z12) - (u1 + u2) * (c * z12) - e1 - e2 };
        z = z1 + r * z2;
        e = e1 + r * t + r.square() * e2;
    }

    friend std::ostream& operator << (std::ostream& out, const R1CS& val)
    {
        return out << '[' << val.a << ", " << val.b << ", " << val.c << ']';
    }

    template<typename S = E, typename DRG>
    constexpr std::pair<Vector<S>, Vector<S>> squeeze(DRG& drg) const {
        auto z = Vector<S>::squeeze(drg, variables());
        return { z, error(z) };
    }

    template<typename S = E, typename RNG>
    std::pair<Vector<S>, Vector<S>> random(RNG& rng) const {
        auto z = Vector<S>::random(rng, variables());
        return { z, error(z) };
    }
private:
    template<typename S = E>
    constexpr Vector<S> error(const Vector<S>& z) const {
        const S& u = z[0];
        return (a * z) * (b * z) - u * (c * z);
    }
};

#endif

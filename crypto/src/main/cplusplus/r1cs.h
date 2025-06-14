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
#include <random>
#include <stdexcept>
#include <utility>
#include <fmt/format.h>

#include "matrixsparse.h"
#include "vector.h"

namespace blacknet::crypto {

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
    constexpr R1CS(R1CS&&) noexcept = default;
    constexpr ~R1CS() noexcept = default;

    constexpr R1CS& operator = (R1CS&&) noexcept = default;

    constexpr bool operator == (const R1CS&) const = default;

    constexpr std::size_t constraints() const {
        return a.rows();
    }

    constexpr std::size_t variables() const {
        return a.columns;
    }

    constexpr bool isSatisfied(const Vector<E>& z) const {
        if (variables() == z.size()) {
            return (a * z) * (b * z) == c * z;
        } else {
            throw std::runtime_error(fmt::format("Assigned only {} variables of {} required", variables(), z.size()));
        }
    }

    constexpr bool isSatisfied(const Vector<E>& z, const Vector<E>& e) const {
        if (variables() == z.size()) {
            return error(z) == e;
        } else {
            throw std::runtime_error(fmt::format("Assigned only {} variables of {} required", variables(), z.size()));
        }
    }

    constexpr void fold(
        const E& r,
        Vector<E>& z, Vector<E>& e,
        const Vector<E>& z1, const Vector<E>& e1,
        const Vector<E>& z2, const Vector<E>& e2
    ) const {
        const E& u1 = z1[0];
        const E& u2 = z2[0];
        Vector<E> z12{ z1 + z2 };
        Vector<E> t{ (a * z12) * (b * z12) - (u1 + u2) * (c * z12) - e1 - e2 };
        z = z1 + r * z2;
        e = e1 + r * t + r.square() * e2;
    }

    constexpr Vector<E> assigment(E&& constant = E(1)) const {
        Vector<E> z;
        z.elements.reserve(variables());
        z.elements.emplace_back(constant);
        return z;
    }

    friend std::ostream& operator << (std::ostream& out, const R1CS& val)
    {
        return out << '[' << val.a << ", " << val.b << ", " << val.c << ']';
    }

    template<typename Sponge>
    constexpr std::pair<Vector<E>, Vector<E>> squeeze(Sponge& sponge) const {
        auto z = Vector<E>::squeeze(sponge, variables());
        return { z, error(z) };
    }

    template<std::uniform_random_bit_generator RNG>
    std::pair<Vector<E>, Vector<E>> random(RNG& rng) const {
        auto z = Vector<E>::random(rng, variables());
        return { z, error(z) };
    }
private:
    constexpr Vector<E> error(const Vector<E>& z) const {
        const E& u = z[0];
        return (a * z) * (b * z) - u * (c * z);
    }
};

}

#endif

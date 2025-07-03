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
#include "vectordense.h"

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

    constexpr bool isSatisfied(const VectorDense<E>& z) const {
        if (variables() == z.size()) {
            return (a * z) * (b * z) == c * z;
        } else {
            throw std::runtime_error(fmt::format("Assigned {} variables instead of {} required", z.size(), variables()));
        }
    }

    constexpr bool isSatisfied(const VectorDense<E>& z, const VectorDense<E>& e) const {
        if (variables() == z.size()) {
            return error(z) == e;
        } else {
            throw std::runtime_error(fmt::format("Assigned {} variables instead of {} required", z.size(), variables()));
        }
    }

    constexpr void fold(
        const E& r,
        VectorDense<E>& z, VectorDense<E>& e,
        const VectorDense<E>& z1, const VectorDense<E>& e1,
        const VectorDense<E>& z2, const VectorDense<E>& e2
    ) const {
        const E& u1 = z1[0];
        const E& u2 = z2[0];
        VectorDense<E> z12{ z1 + z2 };
        VectorDense<E> t{ (a * z12) * (b * z12) - (u1 + u2) * (c * z12) - e1 - e2 };
        z = z1 + r * z2;
        e = e1 + r * t + r.square() * e2;
    }

    constexpr VectorDense<E> assigment(E&& constant = E::multiplicative_identity()) const {
        VectorDense<E> z;
        z.elements.reserve(variables());
        z.elements.emplace_back(constant);
        return z;
    }

    friend std::ostream& operator << (std::ostream& out, const R1CS& val)
    {
        return out << '[' << val.a << ", " << val.b << ", " << val.c << ']';
    }

    template<typename Sponge>
    constexpr std::pair<VectorDense<E>, VectorDense<E>> squeeze(Sponge& sponge) const {
        auto z = VectorDense<E>::squeeze(sponge, variables());
        return { z, error(z) };
    }

    template<std::uniform_random_bit_generator RNG>
    std::pair<VectorDense<E>, VectorDense<E>> random(RNG& rng) const {
        auto z = VectorDense<E>::random(rng, variables());
        return { z, error(z) };
    }
private:
    constexpr VectorDense<E> error(const VectorDense<E>& z) const {
        const E& u = z[0];
        return (a * z) * (b * z) - u * (c * z);
    }
};

}

#endif

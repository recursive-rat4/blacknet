/*
 * Copyright (c) 2025 Pavel Vasin
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

#ifndef BLACKNET_CRYPTO_FS_H
#define BLACKNET_CRYPTO_FS_H

#include <concepts>
#include <ostream>
#include <random>

#include "vectordense.h"

namespace blacknet::crypto {

template<typename CS, typename Duplex>
requires(std::same_as<typename CS::ElementType, typename Duplex::Z>)
struct FS {
    using F = Duplex::Z;

    CS& cs;

    constexpr bool operator == (const FS&) const = default;

    constexpr void fold(
        VectorDense<F>& z, VectorDense<F>& e,
        const VectorDense<F>& z1, const VectorDense<F>& e1,
        const VectorDense<F>& z2, const VectorDense<F>& e2
    ) const {
        Duplex duplex; //XXX iv

        // Size of vectors is implied by constraint system
        for (const F& e : z1.elements) duplex.absorb(e);
        for (const F& e : e1.elements) duplex.absorb(e);
        for (const F& e : z2.elements) duplex.absorb(e);
        for (const F& e : e2.elements) duplex.absorb(e);

        F r(F::squeeze(duplex));
        cs.fold(r, z, e, z1, e1, z2, e2);
    }

    template<std::uniform_random_bit_generator RNG>
    void randomize(
        RNG& rng,
        VectorDense<F>& z, VectorDense<F>& e,
        const VectorDense<F>& z1, const VectorDense<F>& e1
    ) const {
        auto [z2, e2] = cs.random(rng);
        fold(z, e, z1, e1, z2, e2);
    }

    friend std::ostream& operator << (std::ostream& out, const FS& val)
    {
        return out << '(' << val.cs << ')';
    }
};

}

#endif

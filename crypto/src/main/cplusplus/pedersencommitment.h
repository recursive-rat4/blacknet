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

#ifndef BLACKNET_CRYPTO_PEDERSENCOMMITMENT_H
#define BLACKNET_CRYPTO_PEDERSENCOMMITMENT_H

#include <oneapi/tbb/blocked_range.h>
#include <oneapi/tbb/parallel_reduce.h>

#include "vector.h"

namespace blacknet::crypto {

/*
 * Non-Interactive and Information-Theoretic Secure Verifiable Secret Sharing
 * Torben Pryds Pedersen
 * 1991
 * https://www.cs.cornell.edu/courses/cs754/2001fa/129.PDF
 */

template<typename G>
class PedersenCommitment {
    Vector<G> pp;
public:
    constexpr PedersenCommitment(const Vector<G>& pp) : pp(pp) {}
    constexpr PedersenCommitment(Vector<G>&& pp) : pp(std::move(pp)) {}

    template<typename Sponge>
    constexpr static Vector<G> setup(Sponge& sponge, std::size_t size) {
        return Vector<G>::squeeze(sponge, size);
    }

    constexpr G commit(const G::Scalar& s, const G::Scalar& t) const {
        return pp[0] * s + pp[1] * t;
    }

    constexpr bool open(const G& e, const G::Scalar& s, const G::Scalar& t) const {
        return e == commit(s, t);
    }

    constexpr G commit(const Vector<typename G::Scalar>& v) const {
        using namespace oneapi::tbb;
        return parallel_reduce(
            blocked_range<std::size_t>(0, v.size()),
            G::additive_identity(),
            [&](const blocked_range<std::size_t>& range, G acc) -> G {
                for (std::size_t i = range.begin(); i != range.end(); ++i)
                    acc += pp[i] * v[i];
                return acc;
            },
            [](const G& a, const G& b) -> G {
                return a + b;
            }
        );
    }

    constexpr bool open(const G& e, const Vector<typename G::Scalar>& v) const {
        return e == commit(v);
    }
};

}

#endif

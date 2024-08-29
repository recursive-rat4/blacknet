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

#ifndef BLACKNET_CRYPTO_PEDERSENCOMMITMENT_H
#define BLACKNET_CRYPTO_PEDERSENCOMMITMENT_H

#include <vector>

/*
 * Non-Interactive and Information-Theoretic Secure Verifiable Secret Sharing
 * Torben Pryds Pedersen
 * 1991
 * https://www.cs.cornell.edu/courses/cs754/2001fa/129.PDF
 */

template<typename G>
class PedersenCommitment {
    std::vector<G> pp;
public:
    constexpr PedersenCommitment(const std::vector<G>& pp) : pp(pp) {}
    constexpr PedersenCommitment(std::vector<G>&& pp) : pp(std::move(pp)) {}

    template<typename DRG>
    constexpr static std::vector<G> setup(DRG& drg, std::size_t size) {
        std::vector<G> t(size);
        for (std::size_t i = 0; i < size; ++i)
            t[i] = G::squeeze(drg);
        return t;
    }

    constexpr G commit(const G::Scalar& s, const G::Scalar& t) const {
        return pp[0] * s + pp[1] * t;
    }

    constexpr bool open(const G& e, const G::Scalar& s, const G::Scalar& t) const {
        return e == commit(s, t);
    }

    constexpr G commit(const std::vector<typename G::Scalar>& v) const {
        G sigma(G::LEFT_ADDITIVE_IDENTITY());
        for (std::size_t i = 0; i < v.size(); ++i)
            sigma += pp[i] * v[i];
        return sigma;
    }

    constexpr bool open(const G& e, const std::vector<typename G::Scalar>& v) const {
        return e == commit(v);
    }
};

#endif

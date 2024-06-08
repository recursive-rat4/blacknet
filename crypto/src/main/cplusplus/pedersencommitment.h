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

/*
 * Non-Interactive and Information-Theoretic Secure Verifiable Secret Sharing
 * Torben Pryds Pedersen
 * 1991
 * https://www.cs.cornell.edu/courses/cs754/2001fa/129.PDF
 */

template<typename G>
class PedersenCommitment {
    G g;
    G h;
public:
    constexpr PedersenCommitment(const G& g, const G& h) : g{g}, h(h) {}

    constexpr G commit(const G::Scalar& s, const G::Scalar& t) const {
        return g * s + h * t;
    }

    constexpr bool open(const G& e, const G::Scalar& s, const G::Scalar& t) const {
        return e == commit(s, t);
    }
};

#endif

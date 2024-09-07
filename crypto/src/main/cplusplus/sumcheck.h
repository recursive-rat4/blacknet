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

#ifndef BLACKNET_CRYPTO_SUMCHECK_H
#define BLACKNET_CRYPTO_SUMCHECK_H

#include <numeric>

#include "multilinearextension.h"
#include "univariatepolynomial.h"

/*
 * Algebraic Methods for Interactive Proof Systems
 * Carsten Lund, Lance Fortnow, Howard Karloff, Noam Nisan
 * 1992
 * https://users.cs.fiu.edu/~giri/teach/5420/f01/LundIPS.pdf
 */

template<
    typename Z,
    typename RO
>
class SumCheck {
public:
    class Proof {
    public:
        std::vector<UnivariatePolynomial<Z>> claims;

        constexpr Proof(std::size_t capacity) {
            claims.reserve(capacity);
        }

        friend std::ostream& operator << (std::ostream& out, const Proof& val)
        {
            out << '[';
            std::copy(val.claims.begin(), val.claims.end(), boost::io::make_ostream_joiner(out, ", "));
            return out << ']';
        }
    };

    constexpr static Proof prove(const MultilinearExtension<Z>& polynomial) {
        Proof proof(polynomial.variables());
        RO ro;
        MultilinearExtension<Z> state(polynomial);
        for (std::size_t round = 0; round < polynomial.variables(); ++round) {
            MultilinearExtension<Z> p0(state.template bind<0>());
            MultilinearExtension<Z> p1(state.template bind<1>());
            Z v0(std::reduce(p0.coefficients.begin(), p0.coefficients.end()));
            Z v1(std::reduce(p1.coefficients.begin(), p1.coefficients.end()));
            auto claim(UnivariatePolynomial<Z>::interpolate(v0, v1));
            for (const Z& c : claim.coefficients)
                ro.absorb(c);
            proof.claims.emplace_back(std::move(claim));
            Z challenge(RO(ro).squeeze());
            state = state.bind(challenge);
        }
        return proof;
    }

    constexpr static bool verify(const MultilinearExtension<Z>& polynomial, const Z& sum, const Proof& proof) {
        if (proof.claims.size() != polynomial.variables())
            return false;
        RO ro;
        std::vector<Z> r(polynomial.variables());
        Z state(sum);
        for (std::size_t round = 0; round < polynomial.variables(); ++round) {
            const auto& claim = proof.claims[round];
            if (claim.degree() != polynomial.degree())
                return false;
            if (state != claim(0) + claim(1))
                return false;
            for (const Z& c : claim.coefficients)
                ro.absorb(c);
            Z challenge(RO(ro).squeeze());
            r[round] = challenge;
            state = claim(challenge);
        }
        if (state != polynomial(r))
            return false;
        return true;
    }
};

#endif

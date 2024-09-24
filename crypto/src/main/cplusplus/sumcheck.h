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

#include <algorithm>

#include "univariatepolynomial.h"

/*
 * Algebraic Methods for Interactive Proof Systems
 * Carsten Lund, Lance Fortnow, Howard Karloff, Noam Nisan
 * 1992
 * https://users.cs.fiu.edu/~giri/teach/5420/f01/LundIPS.pdf
 */

template<
    typename Z,
    typename F,
    template<typename> typename P,
    typename RO
>
class SumCheck {
public:
    class Proof {
    public:
        std::vector<UnivariatePolynomial<F>> claims;

        consteval Proof() : claims() {}
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

    constexpr static Proof prove(const P<Z>& polynomial) {
        Proof proof(polynomial.variables());
        RO ro;
        P<F> state(polynomial.template homomorph<F>());
        for (std::size_t round = 0; round < polynomial.variables(); ++round) {
            UnivariatePolynomial<F> claim;
            if constexpr (polynomial.degree() == 2) {
                P<F> p0(state.template bind<F(0)>());
                P<F> p1(state.template bind<F(1)>());
                P<F> p2(state.template bind<F(2)>());
                F v0(*std::ranges::fold_left_first(p0(), std::plus<F>()));
                F v1(*std::ranges::fold_left_first(p1(), std::plus<F>()));
                F v2(*std::ranges::fold_left_first(p2(), std::plus<F>()));
                claim = interpolate(v0, v1, v2);
            } else if constexpr (polynomial.degree() == 1) {
                P<F> p0(state.template bind<F(0)>());
                P<F> p1(state.template bind<F(1)>());
                F v0(*std::ranges::fold_left_first(p0(), std::plus<F>()));
                F v1(*std::ranges::fold_left_first(p1(), std::plus<F>()));
                claim = interpolate(v0, v1);
            } else {
                static_assert(false, "Not implemented");
            }
            claim.absorb(ro);
            proof.claims.emplace_back(std::move(claim));
            RO fork(ro);
            F challenge(F::squeeze(fork));
            state = state.bind(challenge);
        }
        return proof;
    }

    constexpr static bool verify(const P<Z>& polynomial, const Z& sum, const Proof& proof) {
        if (proof.claims.size() != polynomial.variables())
            return false;
        RO ro;
        std::vector<F> r(polynomial.variables());
        F state(sum);
        for (std::size_t round = 0; round < polynomial.variables(); ++round) {
            const auto& claim = proof.claims[round];
            if (claim.degree() != polynomial.degree())
                return false;
            if (state != claim(F(0)) + claim(F(1)))
                return false;
            claim.absorb(ro);
            RO fork(ro);
            F challenge(F::squeeze(fork));
            r[round] = challenge;
            state = claim(challenge);
        }
        if (state != polynomial.template homomorph<F>()(r))
            return false;
        return true;
    }

    constexpr static UnivariatePolynomial<F> interpolate(const F& p0, const F& p1) {
        return UnivariatePolynomial<F>{p0, p1 - p0};
    }
    constexpr static UnivariatePolynomial<F> interpolate(const F& p0, const F& p1, const F& p2) {
        // Undefined behaviour is prohibited in consteval
        static const F inv2 = Z(2).invert().value();

        F a(inv2 * (p2 - p1.douple() + p0));
        F b(p1 - p0 - a);
        F c(p0);
        return UnivariatePolynomial<F>{c, b, a};
    }
};

#endif

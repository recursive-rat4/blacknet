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
        {
            // Perform the zeroth round over the base structure abaft the strong sampling set
            UnivariatePolynomial<F> claim(proveRound<Z>(polynomial).template homomorph<F>());
            claim.absorb(ro);
            proof.claims.emplace_back(std::move(claim));
            RO fork(ro);
            F challenge(F::squeeze(fork));
            state = state.bind(challenge);
        }
        for (std::size_t round = 1; round < polynomial.variables(); ++round) {
            UnivariatePolynomial<F> claim(proveRound<F>(state));
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

    template<typename S>
    constexpr static UnivariatePolynomial<S> interpolate(const S& p0, const S& p1) {
        return UnivariatePolynomial<S>{p0, p1 - p0};
    }
    template<typename S>
    constexpr static UnivariatePolynomial<S> interpolate(const S& p0, const S& p1, const S& p2) {
        // Undefined behaviour is prohibited in consteval
        static const S inv2 = Z(2).invert().value();

        S a(inv2 * (p2 - p1.douple() + p0));
        S b(p1 - p0 - a);
        S c(p0);
        return UnivariatePolynomial<S>{c, b, a};
    }
    template<typename S>
    constexpr static UnivariatePolynomial<S> interpolate(const S& p0, const S& p1, const S& p2, const S& p3, const S& p4) {
        // Undefined behaviour is prohibited in consteval
        static const S inv2 = Z(2).invert().value();
        static const S inv3 = Z(3).invert().value();
        static const S inv4 = Z(4).invert().value();
        static const S inv6 = Z(6).invert().value();
        static const S inv12 = Z(12).invert().value();
        static const S inv24 = Z(24).invert().value();

        S a(p4 * inv24 - p3 * inv6 + p2 * inv4 - p1 * inv6 + p0 * inv24);
        S b(- p4 * inv4 + p3 * Z(7) * inv6 - p2.douple() + p1 * Z(3) * inv2 - p0 * Z(5) * inv12);
        S c(p4 * Z(11) * inv24 - p3 * Z(7) * inv3 + p2 * Z(19) * inv4 - p1 * Z(13) * inv3 + p0 * Z(35) * inv24);
        S d(- p4 * inv4 + p3 * Z(4) * inv3 - p2 * Z(3) + p1.douple().douple() - p0 * Z(25) * inv12);
        S e(p0);
        return UnivariatePolynomial<S>{e, d, c, b, a};
    }
private:
    template<typename S>
    constexpr static UnivariatePolynomial<S> proveRound(const P<S>& state) {
        if constexpr (state.degree() == 4) {
            P<S> p0(state.template bind<S(0)>());
            P<S> p1(state.template bind<S(1)>());
            P<S> p2(state.template bind<S(2)>());
            P<S> p3(state.template bind<S(3)>());
            P<S> p4(state.template bind<S(4)>());
            S v0(*std::ranges::fold_left_first(p0(), std::plus<S>()));
            S v1(*std::ranges::fold_left_first(p1(), std::plus<S>()));
            S v2(*std::ranges::fold_left_first(p2(), std::plus<S>()));
            S v3(*std::ranges::fold_left_first(p3(), std::plus<S>()));
            S v4(*std::ranges::fold_left_first(p4(), std::plus<S>()));
            return interpolate<S>(v0, v1, v2, v3, v4);
        } else if constexpr (state.degree() == 2) {
            P<S> p0(state.template bind<S(0)>());
            P<S> p1(state.template bind<S(1)>());
            P<S> p2(state.template bind<S(2)>());
            S v0(*std::ranges::fold_left_first(p0(), std::plus<S>()));
            S v1(*std::ranges::fold_left_first(p1(), std::plus<S>()));
            S v2(*std::ranges::fold_left_first(p2(), std::plus<S>()));
            return interpolate<S>(v0, v1, v2);
        } else if constexpr (state.degree() == 1) {
            P<S> p0(state.template bind<S(0)>());
            P<S> p1(state.template bind<S(1)>());
            S v0(*std::ranges::fold_left_first(p0(), std::plus<S>()));
            S v1(*std::ranges::fold_left_first(p1(), std::plus<S>()));
            return interpolate<S>(v0, v1);
        } else {
            static_assert(false, "Not implemented");
        }
    }
};

#endif

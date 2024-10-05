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

#include "univariatepolynomial.h"
#include "util.h"

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

        constexpr bool operator == (const Proof&) const = default;

        friend std::ostream& operator << (std::ostream& out, const Proof& val)
        {
            out << '[';
            std::copy(val.claims.begin(), val.claims.end(), boost::io::make_ostream_joiner(out, ", "));
            return out << ']';
        }
    };

    constexpr static Proof prove(const P<Z>& polynomial, const Z& sum) {
        Proof proof(polynomial.variables());
        RO ro;
        P<F> state(polynomial.template homomorph<F>());
        F hint;
        {
            // Perform the zeroth round over the base structure abaft the strong sampling set
            UnivariatePolynomial<F> claim(proveRound<Z>(polynomial, sum).template homomorph<F>());
            claim.absorb(ro);
            RO fork(ro);
            F challenge(F::squeeze(fork));
            state.bind(challenge);
            hint = claim(challenge);
            proof.claims.emplace_back(std::move(claim));
        }
        for (std::size_t round = 1; round < polynomial.variables(); ++round) {
            UnivariatePolynomial<F> claim(proveRound<F>(state, hint));
            claim.absorb(ro);
            RO fork(ro);
            F challenge(F::squeeze(fork));
            state.bind(challenge);
            hint = claim(challenge);
            proof.claims.emplace_back(std::move(claim));
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
    constexpr static UnivariatePolynomial<S> interpolate(const S& z0, const S& p1) {
        return UnivariatePolynomial<S>{z0, p1 - z0};
    }
    template<typename S>
    constexpr static UnivariatePolynomial<S> interpolate(const S& n1, const S& z0, const S& p1) {
        // Undefined behaviour is prohibited in consteval
        static const S inv2 = Z(2).invert().value();

        S a(p1 * inv2 + n1 * inv2 - z0);
        S b(p1 * inv2 - n1 * inv2);
        S c(z0);
        return UnivariatePolynomial<S>{c, b, a};
    }
    template<typename S>
    constexpr static UnivariatePolynomial<S> interpolate(const S& n2, const S& n1, const S& z0, const S& p1, const S& p2) {
        // Undefined behaviour is prohibited in consteval
        static const S mul_2_div_3 = Z(2) * Z(3).invert().value();
        static const S inv4 = Z(4).invert().value();
        static const S inv6 = Z(6).invert().value();
        static const S inv12 = Z(12).invert().value();
        static const S inv24 = Z(24).invert().value();

        S a(z0 * inv4 - p1 * inv6 + p2 * inv24 - n1 * inv6 + n2 * inv24);
        S b(- p1 * inv6 + p2 * inv12 + n1 * inv6 - n2 * inv12);
        S c(- z0 * Z(5) * inv4 + p1 * mul_2_div_3 - p2 * inv24 + n1 * mul_2_div_3 - n2 * inv24);
        S d(p1 * mul_2_div_3 - p2 * inv12 - n1 * mul_2_div_3 + n2 * inv12);
        S e(z0);
        return UnivariatePolynomial<S>{e, d, c, b, a};
    }
private:
    template<typename S>
    constexpr static UnivariatePolynomial<S> proveRound(const P<S>& state, const S& hint) {
        std::vector<S> evaluations(1 << (state.variables() - 1));
        // Lagrange basis aboard, take the hint for zero
        if constexpr (state.degree() == 4) {
            state.template bind<S(-2), util::Assign<S>>(evaluations);
            S n2(util::Sum<S>::call(evaluations));
            state.template bind<S(-1), util::Assign<S>>(evaluations);
            S n1(util::Sum<S>::call(evaluations));
            state.template bind<S(1), util::Assign<S>>(evaluations);
            S p1(util::Sum<S>::call(evaluations));
            state.template bind<S(2), util::Assign<S>>(evaluations);
            S p2(util::Sum<S>::call(evaluations));
            return interpolate<S>(n2, n1, hint - p1, p1, p2);
        } else if constexpr (state.degree() == 2) {
            state.template bind<S(-1), util::Assign<S>>(evaluations);
            S n1(util::Sum<S>::call(evaluations));
            state.template bind<S(1), util::Assign<S>>(evaluations);
            S p1(util::Sum<S>::call(evaluations));
            return interpolate<S>(n1, hint - p1, p1);
        } else if constexpr (state.degree() == 1) {
            state.template bind<S(1), util::Assign<S>>(evaluations);
            S p1(util::Sum<S>::call(evaluations));
            return interpolate<S>(hint - p1, p1);
        } else {
            static_assert(false, "Not implemented");
        }
    }
};

#endif

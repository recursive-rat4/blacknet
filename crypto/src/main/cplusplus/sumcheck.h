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

#ifndef BLACKNET_CRYPTO_SUMCHECK_H
#define BLACKNET_CRYPTO_SUMCHECK_H

#include <stdexcept>
#include <fmt/format.h>

#include "interpolation.h"
#include "univariatepolynomial.h"
#include "util.h"

namespace blacknet::crypto {

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
    struct Proof {
        std::vector<UnivariatePolynomial<F>> claims;

        constexpr Proof() = default;
        constexpr Proof(std::size_t capacity) {
            claims.reserve(capacity);
        }

        constexpr bool operator == (const Proof&) const = default;

        friend std::ostream& operator << (std::ostream& out, const Proof& val)
        {
            return out << val.claims;
        }
    };

    constexpr static Proof prove(const P<Z>& polynomial, const Z& sum) {
        Proof proof(polynomial.variables());
        RO ro;
        P<F> state(polynomial.template homomorph<F>());
        F hint;
        {
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

    struct ProofEarlyStopped {
        F state;
        UnivariatePolynomial<F> claim;
        F challenge;

        constexpr ProofEarlyStopped() = default;

        constexpr bool operator == (const ProofEarlyStopped&) const = default;

        friend std::ostream& operator << (std::ostream& out, const ProofEarlyStopped& val)
        {
            return out << '('  << val.state << ", " << val.claim << ", " << val.challenge << ')';
        }
    };

    constexpr static ProofEarlyStopped proveEarlyStopping(const P<F>& polynomial, const F& sum) {
        ProofEarlyStopped proof;
        RO ro;

        UnivariatePolynomial<F> claim(proveRound<F>(polynomial, sum));
        claim.absorb(ro);
        F challenge(F::squeeze(ro));
        proof.state = claim(challenge);
        proof.claim = std::move(claim);
        proof.challenge = std::move(challenge);

        return proof;
    }

    constexpr static bool verifyEarlyStopping(const P<F>& polynomial, const F& sum, const ProofEarlyStopped& proof) {
        RO ro;

        if (proof.claim.degree() != polynomial.degree())
            return false;
        if (sum != proof.claim(F(0)) + proof.claim(F(1)))
            return false;
        proof.claim.absorb(ro);
        F challenge(F::squeeze(ro));
        if (proof.challenge != challenge)
            return false;
        if (proof.state != proof.claim(proof.challenge))
            return false;

        return true;
    }
private:
    template<typename S>
    constexpr static UnivariatePolynomial<S> proveRound(const P<S>& state, const S& hint) {
        std::vector<S> evaluations(1 << (state.variables() - 1));
        if (state.degree() == 5) {
            state.template bind<Z(-2), util::Assign<S>>(evaluations);
            S n2(util::Sum<S>::call(evaluations));
            state.template bind<Z(-1), util::Assign<S>>(evaluations);
            S n1(util::Sum<S>::call(evaluations));
            state.template bind<Z(1), util::Assign<S>>(evaluations);
            S p1(util::Sum<S>::call(evaluations));
            state.template bind<Z(2), util::Assign<S>>(evaluations);
            S p2(util::Sum<S>::call(evaluations));
            state.template bind<Z(3), util::Assign<S>>(evaluations);
            S p3(util::Sum<S>::call(evaluations));
            return Interpolation<Z, S>::balanced(n2, n1, hint - p1, p1, p2, p3);
        } else if (state.degree() == 4) {
            state.template bind<Z(-2), util::Assign<S>>(evaluations);
            S n2(util::Sum<S>::call(evaluations));
            state.template bind<Z(-1), util::Assign<S>>(evaluations);
            S n1(util::Sum<S>::call(evaluations));
            state.template bind<Z(1), util::Assign<S>>(evaluations);
            S p1(util::Sum<S>::call(evaluations));
            state.template bind<Z(2), util::Assign<S>>(evaluations);
            S p2(util::Sum<S>::call(evaluations));
            return Interpolation<Z, S>::balanced(n2, n1, hint - p1, p1, p2);
        } else if (state.degree() == 3) {
            state.template bind<Z(-1), util::Assign<S>>(evaluations);
            S n1(util::Sum<S>::call(evaluations));
            state.template bind<Z(1), util::Assign<S>>(evaluations);
            S p1(util::Sum<S>::call(evaluations));
            state.template bind<Z(2), util::Assign<S>>(evaluations);
            S p2(util::Sum<S>::call(evaluations));
            return Interpolation<Z, S>::balanced(n1, hint - p1, p1, p2);
        } else if (state.degree() == 2) {
            state.template bind<Z(-1), util::Assign<S>>(evaluations);
            S n1(util::Sum<S>::call(evaluations));
            state.template bind<Z(1), util::Assign<S>>(evaluations);
            S p1(util::Sum<S>::call(evaluations));
            return Interpolation<Z, S>::balanced(n1, hint - p1, p1);
        } else if (state.degree() == 1) {
            state.template bind<Z(1), util::Assign<S>>(evaluations);
            S p1(util::Sum<S>::call(evaluations));
            return Interpolation<Z, S>::balanced(hint - p1, p1);
        } else {
            throw std::runtime_error(fmt::format(
                "Sum-check prover not implemented for degree {}", state.degree()
            ));
        }
    }
};

}

#endif

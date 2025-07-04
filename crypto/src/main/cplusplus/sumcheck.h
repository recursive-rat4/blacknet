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

#include <optional>
#include <stdexcept>
#include <utility>
#include <fmt/format.h>

#include "interpolation.h"
#include "point.h"
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
    typename R,
    typename P,
    typename Duplex,
    typename E = R
>
class SumCheck {
public:
    struct Proof {
        std::vector<UnivariatePolynomial<R>> claims;

        constexpr Proof() = default;
        constexpr Proof(std::size_t capacity) {
            claims.reserve(capacity);
        }

        constexpr bool operator == (const Proof&) const = default;

        friend std::ostream& operator << (std::ostream& out, const Proof& val)
        {
            return out << val.claims;
        }

        template<typename Builder>
        requires(std::same_as<R, typename Builder::R>)
        struct Circuit {
            using Variable = Builder::Variable;
            using UnivariatePolynomial = UnivariatePolynomial<R>::template Circuit<Builder>;

            std::vector<UnivariatePolynomial> claims;

            constexpr Circuit(
                Builder* circuit,
                Variable::Type type,
                std::size_t variables,
                std::size_t degree
            ) : claims() {
                claims.reserve(variables);
                std::ranges::generate_n(std::back_inserter(claims), variables, [&]{
                    return UnivariatePolynomial(circuit, type, degree);
                });
            }
        };
    };

    constexpr static Proof prove(
        const P& polynomial,
        const R& sum,
        Duplex& duplex
    ) {
        Proof proof(polynomial.variables());
        P state(polynomial);
        R hint(sum);
        for (std::size_t round = 0; round < polynomial.variables(); ++round) {
            UnivariatePolynomial<R> claim(proveRound(state, hint));
            claim.absorb(duplex);
            E challenge(E::squeeze(duplex));
            state.bind(challenge);
            hint = claim(challenge);
            proof.claims.emplace_back(std::move(claim));
        }
        return proof;
    }

    constexpr static bool verify(
        const P& polynomial,
        const R& sum,
        const Proof& proof,
        Duplex& duplex
    ) {
        if (proof.claims.size() != polynomial.variables())
            return false;
        Point<R> r(polynomial.variables());
        R state(sum);
        for (std::size_t round = 0; round < polynomial.variables(); ++round) {
            const auto& claim = proof.claims[round];
            if (claim.degree() != polynomial.degree())
                return false;
            if (state != claim.at_0_plus_1())
                return false;
            claim.absorb(duplex);
            E challenge(E::squeeze(duplex));
            r[round] = challenge;
            state = claim(challenge);
        }
        if (state != polynomial(r))
            return false;
        return true;
    }

    constexpr static std::optional<std::pair<Point<R>, R>> verifyEarlyStopping(
        const P& polynomial,
        const R& sum,
        const Proof& proof,
        Duplex& duplex
    ) {
        if (proof.claims.size() != polynomial.variables())
            return std::nullopt;
        Point<R> r(polynomial.variables());
        R state(sum);
        for (std::size_t round = 0; round < polynomial.variables(); ++round) {
            const auto& claim = proof.claims[round];
            if (claim.degree() != polynomial.degree())
                return std::nullopt;
            if (state != claim.at_0_plus_1())
                return std::nullopt;
            claim.absorb(duplex);
            E challenge(E::squeeze(duplex));
            r[round] = challenge;
            state = claim(challenge);
        }

        return {{ std::move(r), std::move(state) }};
    }
private:
    constexpr static UnivariatePolynomial<R> proveRound(const P& state, const R& hint) {
        std::vector<R> evaluations(1 << (state.variables() - 1));
        if (state.degree() == 5) {
            state.template bind<R(-2), util::Assign<R>>(evaluations);
            R n2(util::Sum<R>::call(evaluations));
            state.template bind<R(-1), util::Assign<R>>(evaluations);
            R n1(util::Sum<R>::call(evaluations));
            state.template bind<R(1), util::Assign<R>>(evaluations);
            R p1(util::Sum<R>::call(evaluations));
            state.template bind<R(2), util::Assign<R>>(evaluations);
            R p2(util::Sum<R>::call(evaluations));
            state.template bind<R(3), util::Assign<R>>(evaluations);
            R p3(util::Sum<R>::call(evaluations));
            return Interpolation<R>::balanced(n2, n1, hint - p1, p1, p2, p3);
        } else if (state.degree() == 4) {
            state.template bind<R(-2), util::Assign<R>>(evaluations);
            R n2(util::Sum<R>::call(evaluations));
            state.template bind<R(-1), util::Assign<R>>(evaluations);
            R n1(util::Sum<R>::call(evaluations));
            state.template bind<R(1), util::Assign<R>>(evaluations);
            R p1(util::Sum<R>::call(evaluations));
            state.template bind<R(2), util::Assign<R>>(evaluations);
            R p2(util::Sum<R>::call(evaluations));
            return Interpolation<R>::balanced(n2, n1, hint - p1, p1, p2);
        } else if (state.degree() == 3) {
            state.template bind<R(-1), util::Assign<R>>(evaluations);
            R n1(util::Sum<R>::call(evaluations));
            state.template bind<R(1), util::Assign<R>>(evaluations);
            R p1(util::Sum<R>::call(evaluations));
            state.template bind<R(2), util::Assign<R>>(evaluations);
            R p2(util::Sum<R>::call(evaluations));
            return Interpolation<R>::balanced(n1, hint - p1, p1, p2);
        } else if (state.degree() == 2) {
            state.template bind<R(-1), util::Assign<R>>(evaluations);
            R n1(util::Sum<R>::call(evaluations));
            state.template bind<R(1), util::Assign<R>>(evaluations);
            R p1(util::Sum<R>::call(evaluations));
            return Interpolation<R>::balanced(n1, hint - p1, p1);
        } else if (state.degree() == 1) {
            state.template bind<R(1), util::Assign<R>>(evaluations);
            R p1(util::Sum<R>::call(evaluations));
            return Interpolation<R>::balanced(hint - p1, p1);
        } else {
            throw std::runtime_error(fmt::format(
                "Sum-check prover not implemented for degree {}", state.degree()
            ));
        }
    }
public:
template<typename Builder>
requires(std::same_as<R, typename Builder::R>)
struct Circuit {
    using Variable = Builder::Variable;
    using LinearCombination = Builder::LinearCombination;
    using Polynomial = P::template Circuit<Builder>;
    using ProofCircuit = Proof::template Circuit<Builder>;
    using DuplexCircuit = Duplex::template Circuit<Builder>;
    using Point = Point<R>::template Circuit<Builder>;

    Builder* circuit;

    constexpr Circuit(Builder* circuit) : circuit(circuit) {}

    constexpr void verify(
        const Polynomial& polynomial,
        const LinearCombination& sum,
        const ProofCircuit& proof,
        DuplexCircuit& duplex
    ) {
        auto scope = circuit->scope("SumCheck::verify");
        Point r(polynomial.variables());
        LinearCombination state(sum);
        for (std::size_t round = 0; round < polynomial.variables(); ++round) {
            const auto& claim = proof.claims[round];
            scope(state == claim.at_0_plus_1());
            claim.absorb(duplex);
            LinearCombination challenge(duplex.squeeze());
            r[round] = challenge;
            state = claim(challenge);
        }
        scope(state == polynomial(r));
    }

    constexpr std::pair<Point, LinearCombination> verifyEarlyStopping(
        const Polynomial& polynomial,
        const LinearCombination& sum,
        const ProofCircuit& proof,
        DuplexCircuit& duplex
    ) {
        auto scope = circuit->scope("SumCheck::verifyEarlyStopping");
        Point r(polynomial.variables());
        LinearCombination state(sum);
        for (std::size_t round = 0; round < polynomial.variables(); ++round) {
            const auto& claim = proof.claims[round];
            scope(state == claim.at_0_plus_1());
            claim.absorb(duplex);
            LinearCombination challenge(duplex.squeeze());
            r[round] = challenge;
            state = claim(challenge);
        }

        return { std::move(r), std::move(state) };
    }
};

template<std::size_t Degree>
struct Assigner {
    using PAssigner = P::template Assigner<Degree>;
    using DuplexAssigner = Duplex::template Assigner<Degree>;
    using UnivariatePolynomial = UnivariatePolynomial<R>::template Assigner<Degree>;

    std::vector<R>* assigment;

    constexpr bool verify(
        const P& polynomial_,
        const R& sum,
        const Proof& proof,
        DuplexAssigner& duplex
    ) {
        PAssigner polynomial(polynomial_, assigment);
        if (proof.claims.size() != polynomial.variables())
            return false;
        Point<R> r(polynomial.variables());
        R state(sum);
        for (std::size_t round = 0; round < polynomial.variables(); ++round) {
            auto claim = UnivariatePolynomial(proof.claims[round], assigment);
            if (claim.degree() != polynomial.degree())
                return false;
            if (state != claim.at_0_plus_1())
                return false;
            claim.absorb(duplex);
            E challenge(duplex.squeeze());
            r[round] = challenge;
            state = claim(challenge);
        }
        if (state != polynomial(r))
            return false;
        return true;
    }

    constexpr std::optional<std::pair<Point<R>, R>> verifyEarlyStopping(
        const P& polynomial_,
        const R& sum,
        const Proof& proof,
        DuplexAssigner& duplex
    ) {
        PAssigner polynomial(polynomial_, assigment);
        if (proof.claims.size() != polynomial.variables())
            return std::nullopt;
        Point<R> r(polynomial.variables());
        R state(sum);
        for (std::size_t round = 0; round < polynomial.variables(); ++round) {
            auto claim = UnivariatePolynomial(proof.claims[round], assigment);
            if (claim.degree() != polynomial.degree())
                return std::nullopt;
            if (state != claim.at_0_plus_1())
                return std::nullopt;
            claim.absorb(duplex);
            E challenge(duplex.squeeze());
            r[round] = challenge;
            state = claim(challenge);
        }

        return {{ std::move(r), std::move(state) }};
    }
};
};

}

#endif

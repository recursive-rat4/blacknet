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

#ifndef BLACKNET_CRYPTO_LATTICEFOLD_H
#define BLACKNET_CRYPTO_LATTICEFOLD_H

#include "eqextension.h"
#include "matrix.h"
#include "multilinearextension.h"
#include "polynomial.h"
#include "polynomialring.h"
#include "vector.h"

/*
 * LatticeFold: A Lattice-based Folding Scheme and its Applications to Succinct Proof Systems
 * Dan Boneh, Binyi Chen
 * July 30, 2024
 * https://eprint.iacr.org/2024/257
 */

namespace latticefold {
    constexpr ssize_t b = 2;
    const std::size_t k = 16;
    const std::size_t B = 65536;
    const std::size_t D = 64;
    const std::size_t K = 16;

    template<typename Z>
    using Rq = CyclotomicRing<
        Z,
        D
    >;

    template<typename R>
    constexpr Matrix<R> gadget(std::size_t m, std::size_t n) {
        Vector<R> bpm(n);
        bpm[0] = R(1);
        for (std::size_t i = 1; i < n; ++i)
            bpm[i] = bpm[i - 1] * B;
        return Vector<R>::identity(m).tensor(bpm);
    }

    template<typename Z>
    class G1 {
        EqExtension<Z> eq;
        MultilinearExtension<Z> mle;
    public:
        constexpr G1(const std::vector<Z>& r, const Vector<Rq<Z>>& f) : eq(r), mle(f) {}
        constexpr G1(EqExtension<Z>&& eq, MultilinearExtension<Z>&& mle) : eq(std::move(eq)), mle(std::move(mle)) {}

        constexpr std::vector<Z> operator () () const {
            std::vector<Z> r(eq());
            mle.pi(r);
            return r;
        }

        constexpr Z operator () (const std::vector<Z>& point) const {
            return eq(point) * mle(point);
        }

        template<Z e>
        constexpr G1 bind() const {
            return G1(eq.template bind<e>(), mle.template bind<e>());
        }

        constexpr G1 bind(const Z& e) const {
            return G1(eq.bind(e), mle.bind(e));
        }

        consteval std::size_t degree() const {
            return eq.degree() + mle.degree();
        }

        constexpr std::size_t variables() const {
            return eq.variables();
        }

        template<typename S>
        constexpr G1<S> homomorph() const {
            return G1<S>(eq.template homomorph<S>(), mle.template homomorph<S>());
        }
    };

    template<typename Z>
    class G2 {
        using P = Polynomial<Z, MultilinearExtension>;

        EqExtension<Z> eq;
        P mles;
    public:
        constexpr G2(const std::vector<Z>& beta, const Vector<Rq<Z>>& f) : eq(beta), mles(b + b - 1) {
            for (ssize_t j = - (b - 1); j <= b - 1; ++j) {
                mles(MultilinearExtension<Z>(f) - Z(j));
            }
        }
        constexpr G2(EqExtension<Z>&& eq, P&& mles) : eq(std::move(eq)), mles(std::move(mles)) {}

        constexpr std::vector<Z> operator () () const {
            std::vector<Z> r(eq());
            mles.pi(r);
            return r;
        }

        constexpr Z operator () (const std::vector<Z>& point) const {
            Z r(eq(point));
            mles.pi(r, point);
            return r;
        }

        template<Z e>
        constexpr G2 bind() const {
            return G2(eq.template bind<e>(), mles.template bind<e>());
        }

        constexpr G2 bind(const Z& e) const {
            return G2(eq.bind(e), mles.bind(e));
        }

        consteval std::size_t degree() const {
            return eq.degree() + (b + b - 1);
        }

        constexpr std::size_t variables() const {
            return eq.variables();
        }

        template<typename S>
        constexpr G2<S> homomorph() const {
            return G2<S>(eq.template homomorph<S>(), mles.template homomorph<S>());
        }
    };
}

#endif

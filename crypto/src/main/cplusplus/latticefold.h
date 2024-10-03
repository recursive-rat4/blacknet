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
#include "util.h"

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
        constexpr G1(const Z& alpha, const std::vector<Z>& r, const Vector<Rq<Z>>& f) : eq(r), mle(f) {
            eq *= alpha;
        }
        constexpr G1(EqExtension<Z>&& eq, MultilinearExtension<Z>&& mle) : eq(std::move(eq)), mle(std::move(mle)) {}

        constexpr Z operator () (const std::vector<Z>& point) const {
            return eq(point) * mle(point);
        }

        template<Z e, typename Fuse>
        constexpr void bind(std::vector<Z>& hypercube) const {
            std::vector<Z> t(hypercube.size());
            mle.template bind<e, util::Assign<Z>>(t);
            eq.template bind<e, util::Mul<Z>>(t);
            Fuse::call(hypercube, std::move(t));
        }

        constexpr void bind(const Z& e) {
            eq.bind(e);
            mle.bind(e);
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
        // Beta vulgaris
        Polynomial<Z, MultilinearExtension> mles;
    public:
        constexpr G2(const Vector<Rq<Z>>& f) : mles(b + b - 1) {
            for (ssize_t j = - (b - 1); j <= b - 1; ++j) {
                mles(MultilinearExtension<Z>(f) - Z(j));
            }
        }
        constexpr G2(const Z& mu, const Vector<Rq<Z>>& f) : mles(b + b - 1) {
            for (ssize_t j = - (b - 1); j <= b - 1; ++j) {
                mles((MultilinearExtension<Z>(f) - Z(j)) * mu);
            }
        }
        constexpr G2(Polynomial<Z, MultilinearExtension>&& mles) : mles(std::move(mles)) {}

        constexpr Z operator () (const std::vector<Z>& point) const {
            Z r;
            mles.template apply<util::Mul<Z>, util::Assign<Z>>(r, point);
            return r;
        }

        template<Z e, typename Fuse>
        constexpr void bind(std::vector<Z>& hypercube) const {
            std::vector<Z> t(hypercube.size());
            mles.template bind<e, util::Mul<Z>, util::Assign<Z>>(t);
            Fuse::call(hypercube, std::move(t));
        }

        constexpr void bind(const Z& e) {
            mles.bind(e);
        }

        consteval std::size_t degree() const {
            return b + b - 1;
        }

        constexpr std::size_t variables() const {
            return mles.variables();
        }

        template<typename S>
        constexpr G2<S> homomorph() const {
            return G2<S>(mles.template homomorph<S>());
        }
    };

    template<typename Z>
    class GEval {
        Polynomial<Z, G1> g1s;
    public:
        constexpr GEval(
            const std::vector<Z>& alpha,
            const std::vector<std::vector<Z>>& r,
            const std::vector<Vector<Rq<Z>>>& f
        ) : g1s(k + k) {
            for (std::size_t i = 0; i < k + k; ++i) {
                g1s(G1<Z>(alpha[i], r[i], f[i]));
            }
        }
        constexpr GEval(Polynomial<Z, G1>&& g1s) : g1s(std::move(g1s)) {}

        constexpr Z operator () (const std::vector<Z>& point) const {
            Z r;
            g1s.template apply<util::Add<Z>, util::Assign<Z>>(r, point);
            return r;
        }

        template<Z e, typename Fuse>
        constexpr void bind(std::vector<Z>& hypercube) const {
            std::vector<Z> t(hypercube.size());
            g1s.template bind<e, util::Add<Z>, util::Assign<Z>>(t);
            Fuse::call(hypercube, std::move(t));
        }

        constexpr void bind(const Z& e) {
            g1s.bind(e);
        }

        consteval std::size_t degree() const {
            return 1 + 1;
        }

        constexpr std::size_t variables() const {
            return g1s.variables();
        }

        template<typename S>
        constexpr GEval<S> homomorph() const {
            return GEval<S>(g1s.template homomorph<S>());
        }
    };

    template<typename Z>
    class GNorm {
        // GNorm(x) = eq(β, x) Σ G2(μ, f, x)
        EqExtension<Z> eq;
        Polynomial<Z, G2> g2s;
    public:
        constexpr GNorm(
            const std::vector<Z>& beta,
            const std::vector<Z>& mu,
            const std::vector<Vector<Rq<Z>>>& f
        ) : eq(beta), g2s(k + k) {
            for (std::size_t i = 0; i < k + k; ++i) {
                g2s(G2<Z>(mu[i], f[i]));
            }
        }
        constexpr GNorm(EqExtension<Z>&& eq, Polynomial<Z, G2>&& g2s) : eq(std::move(eq)), g2s(std::move(g2s)) {}

        constexpr Z operator () (const std::vector<Z>& point) const {
            Z r;
            g2s.template apply<util::Add<Z>, util::Assign<Z>>(r, point);
            return r * eq(point);
        }

        template<Z e, typename Fuse>
        constexpr void bind(std::vector<Z>& hypercube) const {
            std::vector<Z> t(hypercube.size());
            g2s.template bind<e, util::Add<Z>, util::Assign<Z>>(t);
            eq.template bind<e, util::Mul<Z>>(t);
            Fuse::call(hypercube, std::move(t));
        }

        constexpr void bind(const Z& e) {
            eq.bind(e);
            g2s.bind(e);
        }

        consteval std::size_t degree() const {
            return eq.degree() + (b + b - 1);
        }

        constexpr std::size_t variables() const {
            return eq.variables();
        }

        template<typename S>
        constexpr GNorm<S> homomorph() const {
            return GNorm<S>(eq.template homomorph<S>(), g2s.template homomorph<S>());
        }
    };
}

#endif

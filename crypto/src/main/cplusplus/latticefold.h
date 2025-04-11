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

#ifndef BLACKNET_CRYPTO_LATTICEFOLD_H
#define BLACKNET_CRYPTO_LATTICEFOLD_H

#include <random>

#include "ajtaicommitment.h"
#include "convolution.h"
#include "customizableconstraintsystem.h"
#include "eqextension.h"
#include "matrix.h"
#include "multilinearextension.h"
#include "numbertheoretictransform.h"
#include "polynomial.h"
#include "polynomialring.h"
#include "powextension.h"
#include "vector.h"
#include "util.h"

namespace blacknet::crypto {

/*
 * LatticeFold: A Lattice-based Folding Scheme and its Applications to Succinct Proof Systems
 * Dan Boneh, Binyi Chen
 * July 30, 2024
 * https://eprint.iacr.org/2024/257
 */

template<typename Zq>
struct LatticeFold {
    constexpr static ssize_t b = 2;
    static const std::size_t k = 16;
    static const std::size_t t = 16;
    static const std::size_t B = 65536;
    static const std::size_t D = 64;
    static const std::size_t K = 16;

    static_assert(t == Zq::twiddles());
    static_assert(std::is_signed_v<typename Zq::NumericType>);

    struct CanonicalRingParams {
        using Z = Zq;

        constexpr static const std::size_t N = D;

        constexpr static void convolute(std::array<Z, N>& r, const std::array<Z, N>& a, const std::array<Z, N>& b) {
            convolution::negacyclic<Z, N>(r, a, b);
        }
        constexpr static void toForm(std::array<Z, N>&) {}
        constexpr static void fromForm(std::array<Z, N>&) {}
    };

    struct NTTRingParams {
        using Z = Zq;

        constexpr static const std::size_t N = D;

        constexpr static void convolute(std::array<Z, N>& r, const std::array<Z, N>& a, const std::array<Z, N>& b) {
            ntt::convolute<Z, N>(r, a, b);
        }
        constexpr static void toForm(std::array<Z, N>& a) {
            ntt::cooley_tukey<Z, N>(a);
        }
        constexpr static void fromForm(std::array<Z, N>& a) {
            ntt::gentleman_sande<Z, N>(a);
        }
    };

    using Rq = PolynomialRing<CanonicalRingParams>;
    using RqIso = PolynomialRing<NTTRingParams>;

    constexpr static RqIso& isomorph(Rq&& f) {
        NTTRingParams::toForm(f.coefficients);
        return reinterpret_cast<RqIso&>(f);
    }
    constexpr static Rq& isomorph(RqIso&& f) {
        NTTRingParams::fromForm(f.coefficients);
        return reinterpret_cast<Rq&>(f);
    }

    using BindingCommitment = AjtaiCommitment<RqIso>;

    std::uniform_int_distribution<typename Zq::NumericType> small_distribution{-1, 2};

    template<typename R>
    constexpr static Matrix<R> gadget(std::size_t m, std::size_t n) {
        Vector<R> bpm(n);
        bpm[0] = R(1);
        for (std::size_t i = 1; i < n; ++i)
            bpm[i] = bpm[i - 1] * B;
        return Vector<R>::identity(m).tensor(bpm);
    }

    template<typename Z = Zq>
    class G1 {
        // r 何处
        EqExtension<Z> eq;
        MultilinearExtension<Z> mle;
    public:
        constexpr G1(const std::vector<Z>& r, const Vector<Rq>& f) : eq(r), mle(f) {}
        constexpr G1(const Z& alpha, const std::vector<Z>& r, const Vector<Rq>& f) : eq(r, alpha), mle(f) {}
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

    template<typename Z = Zq>
    class G2 {
        static_assert(b == 2, "Not implemented");
        // G2(x) = μ （ mle³[f](x) - mle[f](x) ）
        Z mu;
        MultilinearExtension<Z> mle;
    public:
        constexpr G2(const Vector<Rq>& f) : mu(Z::LEFT_MULTIPLICATIVE_IDENTITY()), mle(f) {}
        constexpr G2(const Z& mu, const Vector<Rq>& f) : mu(mu), mle(f) {}
        constexpr G2(Z&& mu, MultilinearExtension<Z>&& mle) : mu(std::move(mu)), mle(std::move(mle)) {}

        constexpr Z operator () (const std::vector<Z>& point) const {
            Z t(mle(point));
            return mu * (t * t * t - t);
        }

        template<Z e, typename Fuse>
        constexpr void bind(std::vector<Z>& hypercube) const {
            std::vector<Z> t(hypercube.size());
            mle.template bind<e, util::Assign<Z>>(t);
            std::vector<Z> r(t);
            util::Mul<Z>::call(r, t);
            util::Mul<Z>::call(r, t);
            util::Sub<Z>::call(r, t);
            util::Mul<Z>::call(r, mu);
            Fuse::call(hypercube, std::move(r));
        }

        constexpr void bind(const Z& e) {
            mle.bind(e);
        }

        consteval std::size_t degree() const {
            return b + b - 1;
        }

        constexpr std::size_t variables() const {
            return mle.variables();
        }

        template<typename S>
        constexpr G2<S> homomorph() const {
            return G2<S>(S(mu), mle.template homomorph<S>());
        }
    };

    // r 何处
    template<typename Z = Zq>
    using G3 = CustomizableConstraintSystem<Z>::template Polynomial<Z>;

    template<typename Z = Zq>
    class GEval {
        Polynomial<Z, G1> g1s;
    public:
        constexpr GEval(
            const std::vector<Z>& alpha,
            const std::vector<std::vector<Z>>& r,
            const std::vector<Vector<Rq>>& f
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

    template<typename Z = Zq>
    class GNorm {
        // GNorm(x) = pow(β, x) Σ G2(μ, f, x)
        PowExtension<Z> pow;
        Polynomial<Z, G2> g2s;
    public:
        constexpr GNorm(
            const Z& beta,
            const std::vector<Z>& mu,
            const std::vector<Vector<Rq>>& f
        ) : pow(beta, std::log2(f[0].size() * D)), g2s(k + k) {
            for (std::size_t i = 0; i < k + k; ++i) {
                g2s(G2<Z>(mu[i], f[i]));
            }
        }
        constexpr GNorm(PowExtension<Z>&& pow, Polynomial<Z, G2>&& g2s) : pow(std::move(pow)), g2s(std::move(g2s)) {}

        constexpr Z operator () (const std::vector<Z>& point) const {
            Z r;
            g2s.template apply<util::Add<Z>, util::Assign<Z>>(r, point);
            return r * pow(point);
        }

        template<Z e, typename Fuse>
        constexpr void bind(std::vector<Z>& hypercube) const {
            std::vector<Z> t(hypercube.size());
            g2s.template bind<e, util::Add<Z>, util::Assign<Z>>(t);
            pow.template bind<e, util::Mul<Z>>(t);
            Fuse::call(hypercube, std::move(t));
        }

        constexpr void bind(const Z& e) {
            pow.bind(e);
            g2s.bind(e);
        }

        consteval std::size_t degree() const {
            return pow.degree() + (b + b - 1);
        }

        constexpr std::size_t variables() const {
            return pow.variables();
        }

        template<typename S>
        constexpr GNorm<S> homomorph() const {
            return GNorm<S>(pow.template homomorph<S>(), g2s.template homomorph<S>());
        }
    };

    // 从 Πꟳᴼᴸᴰ
    template<typename Z = Zq>
    class GFold {
        GEval<Z> geval;
        GNorm<Z> gnorm;
    public:
        constexpr GFold(
            const std::vector<Z>& alpha,
            const Z& beta,
            const std::vector<Z>& mu,
            const std::vector<std::vector<Z>>& r,
            const std::vector<Vector<Rq>>& f
        ) : geval(alpha, r, f), gnorm(beta, mu, f) {}
        constexpr GFold(GEval<Z>&& geval, GNorm<Z>&& gnorm) : geval(std::move(geval)), gnorm(std::move(gnorm)) {}

        constexpr Z operator () (const std::vector<Z>& point) const {
            return geval(point) + gnorm(point);
        }

        template<Z e, typename Fuse>
        constexpr void bind(std::vector<Z>& hypercube) const {
            std::vector<Z> t(hypercube.size());
            geval.template bind<e, util::Assign<Z>>(t);
            gnorm.template bind<e, util::Add<Z>>(t);
            Fuse::call(hypercube, std::move(t));
        }

        constexpr void bind(const Z& e) {
            geval.bind(e);
            gnorm.bind(e);
        }

        consteval std::size_t degree() const {
            return gnorm.degree();
        }

        constexpr std::size_t variables() const {
            return gnorm.variables();
        }

        template<typename S>
        constexpr GFold<S> homomorph() const {
            return GFold<S>(geval.template homomorph<S>(), gnorm.template homomorph<S>());
        }
    };
};

}

#endif

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

#include <concepts>
#include <random>

#include "ajtaicommitment.h"
#include "convolution.h"
#include "customizableconstraintsystem.h"
#include "eqextension.h"
#include "latticegadget.h"
#include "matrix.h"
#include "multilinearextension.h"
#include "numbertheoretictransform.h"
#include "point.h"
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

template<
    typename Zq,
    typename Fq,
    typename Rq,
    typename RqIso
>
requires(
    std::same_as<Zq, typename Fq::BaseRing> &&
    std::same_as<Zq, typename Rq::BaseRing> &&
    std::same_as<Zq, typename RqIso::BaseRing>
)
struct LatticeFold {
    constexpr static ssize_t b = 2;
    static const std::size_t b_digits = 64;
    static const std::size_t k = 16;
    static const std::size_t t = 16;
    static const std::size_t B = 65536;
    static const std::size_t B_digits = 4;
    static const std::size_t D = 64;
    static const std::size_t K = 16;

    static_assert(Rq::dimension() == RqIso::dimension());
    static_assert(Rq::dimension() == D);
    static_assert(t == Zq::twiddles());
    static_assert(std::is_signed_v<typename Zq::NumericType>);

    constexpr static RqIso& isomorph(Rq&& f) {
        ntt::cooley_tukey<Zq, D>(f.coefficients);
        return reinterpret_cast<RqIso&>(f);
    }
    constexpr static Rq& isomorph(RqIso&& f) {
        ntt::gentleman_sande<Zq, D>(f.coefficients);
        return reinterpret_cast<Rq&>(f);
    }

    using BindingCommitment = AjtaiCommitment<RqIso>;

    std::uniform_int_distribution<typename Zq::NumericType> small_distribution{-1, 2};

    constexpr static Matrix<Rq> gadget_medium(std::size_t m, std::size_t n) {
        return lattice_gadget::matrix<Rq, B>(m, n);
    }
    constexpr static Matrix<Rq> gadget_small(std::size_t m, std::size_t n) {
        return lattice_gadget::matrix<Rq, b>(m, n);
    }
    constexpr static Vector<Rq> decompose_medium(const Vector<Rq>& f) {
        return lattice_gadget::decompose<Rq, B, B_digits>(f);
    }
    constexpr static Vector<Rq> decompose_small(const Vector<Rq>& f) {
        return lattice_gadget::decompose<Rq, b, b_digits>(f);
    }

    class G1 {
        // r 何处
        EqExtension<Fq> eq;
        MultilinearExtension<Fq> mle;
    public:
        constexpr G1(const std::vector<Fq>& r, const Vector<Rq>& f) : eq(r), mle(f) {}
        constexpr G1(const Fq& alpha, const std::vector<Fq>& r, const Vector<Rq>& f) : eq(r, alpha), mle(f) {}
        constexpr G1(EqExtension<Fq>&& eq, MultilinearExtension<Fq>&& mle) : eq(std::move(eq)), mle(std::move(mle)) {}

        constexpr Fq operator () (const Point<Fq>& point) const {
            return eq(point) * mle(point);
        }

        template<Fq e, typename Fuse>
        constexpr void bind(std::vector<Fq>& hypercube) const {
            std::vector<Fq> t(hypercube.size());
            mle.template bind<e, util::Assign<Fq>>(t);
            eq.template bind<e, util::Mul<Fq>>(t);
            Fuse::call(hypercube, std::move(t));
        }

        constexpr void bind(const Fq& e) {
            eq.bind(e);
            mle.bind(e);
        }

        consteval std::size_t degree() const {
            return eq.degree() + mle.degree();
        }

        constexpr std::size_t variables() const {
            return eq.variables();
        }
    };

    struct G2 {
        static_assert(b == 2, "Not implemented");
        // G2(x) = μ （ mle²[f](x) - mle[f](x) ）
        Fq mu;
        MultilinearExtension<Fq> mle;

        constexpr G2(const Vector<Rq>& f) : mu(Fq::LEFT_MULTIPLICATIVE_IDENTITY()), mle(f) {}
        constexpr G2(const Fq& mu, const Vector<Rq>& f) : mu(mu), mle(f) {}
        constexpr G2(Fq&& mu, MultilinearExtension<Fq>&& mle) : mu(std::move(mu)), mle(std::move(mle)) {}

        constexpr Fq operator () (const Point<Fq>& point) const {
            Fq t = mle(point);
            return mu * (t.square() - t);
        }

        template<Fq e, typename Fuse>
        constexpr void bind(std::vector<Fq>& hypercube) const {
            std::vector<Fq> t(hypercube.size());
            mle.template bind<e, util::Assign<Fq>>(t);
            std::vector<Fq> r(t);
            util::Mul<Fq>::call(r, t);
            util::Sub<Fq>::call(r, t);
            util::Mul<Fq>::call(r, mu);
            Fuse::call(hypercube, std::move(r));
        }

        constexpr void bind(const Fq& e) {
            mle.bind(e);
        }

        consteval std::size_t degree() const {
            return b;
        }

        constexpr std::size_t variables() const {
            return mle.variables();
        }

    template<typename Circuit>
    requires(std::same_as<Fq, typename Circuit::R>)
    struct Gadget {
        using Variable = Circuit::Variable;
        using LinearCombination = Circuit::LinearCombination;
        using MultilinearExtension = typename MultilinearExtension<Fq>::Gadget<Circuit>;
        using Point = typename Point<Fq>::Gadget<Circuit>;

        Circuit& circuit;
        LinearCombination mu;
        MultilinearExtension mle;

        constexpr Gadget(Circuit& circuit, Variable::Type type, std::size_t variables)
            : circuit(circuit),
            mu(circuit.variable(type)),
            mle(circuit, type, variables) {}

        constexpr LinearCombination operator () (const Point& point) const {
            auto scope = circuit.scope("LatticeFold::G2::point");
            LinearCombination t = mle(point);
            // circuit degree 2
            auto tt = circuit.auxiliary();
            circuit(tt == t * t);
            auto r = circuit.auxiliary();
            circuit(r == mu * (tt - t));
            return r;
        }

        consteval std::size_t degree() const {
            return b;
        }

        constexpr std::size_t variables() const {
            return mle.variables();
        }
    };

    struct Tracer {
        using MultilinearExtension = MultilinearExtension<Fq>::Tracer;

        Fq mu;
        MultilinearExtension mle;
        std::vector<Fq>& trace;

        constexpr Tracer(const G2& g2, std::vector<Fq>& trace)
            : mu(g2.mu), mle(g2.mle, trace), trace(trace) {}

        constexpr Fq operator () (const Point<Fq>& point) const {
            Fq t = mle(point);
            return trace.emplace_back(
                mu * (trace.emplace_back(
                    t.square()
                ) - t)
            );
        }

        consteval std::size_t degree() const {
            return b;
        }

        constexpr std::size_t variables() const {
            return mle.variables();
        }
    };
    };

    // r 何处
    using G3 = CustomizableConstraintSystem<Fq>::Polynomial;

    class GEval {
        Polynomial<Fq, G1> g1s;
    public:
        constexpr GEval(
            const std::vector<Fq>& alpha,
            const std::vector<std::vector<Fq>>& r,
            const std::vector<Vector<Rq>>& f
        ) : g1s(k + k) {
            for (std::size_t i = 0; i < k + k; ++i) {
                g1s(G1(alpha[i], r[i], f[i]));
            }
        }
        constexpr GEval(Polynomial<Fq, G1>&& g1s) : g1s(std::move(g1s)) {}

        constexpr Fq operator () (const Point<Fq>& point) const {
            Fq r;
            g1s.template apply<util::Add<Fq>, util::Assign<Fq>>(r, point);
            return r;
        }

        template<Fq e, typename Fuse>
        constexpr void bind(std::vector<Fq>& hypercube) const {
            std::vector<Fq> t(hypercube.size());
            g1s.template bind<e, util::Add<Fq>, util::Assign<Fq>>(t);
            Fuse::call(hypercube, std::move(t));
        }

        constexpr void bind(const Fq& e) {
            g1s.bind(e);
        }

        consteval std::size_t degree() const {
            return 1 + 1;
        }

        constexpr std::size_t variables() const {
            return g1s.variables();
        }
    };

    class GNorm {
        // GNorm(x) = pow(β, x) Σ G2(μ, f, x)
        PowExtension<Fq> pow;
        Polynomial<Fq, G2> g2s;
    public:
        constexpr GNorm(
            const Fq& beta,
            const std::vector<Fq>& mu,
            const std::vector<Vector<Rq>>& f
        ) : pow(beta, std::countr_zero(f[0].size() * D)), g2s(k + k) {
            for (std::size_t i = 0; i < k + k; ++i) {
                g2s(G2(mu[i], f[i]));
            }
        }
        constexpr GNorm(PowExtension<Fq>&& pow, Polynomial<Fq, G2>&& g2s) : pow(std::move(pow)), g2s(std::move(g2s)) {}

        constexpr Fq operator () (const Point<Fq>& point) const {
            Fq r;
            g2s.template apply<util::Add<Fq>, util::Assign<Fq>>(r, point);
            return r * pow(point);
        }

        template<Fq e, typename Fuse>
        constexpr void bind(std::vector<Fq>& hypercube) const {
            std::vector<Fq> t(hypercube.size());
            g2s.template bind<e, util::Add<Fq>, util::Assign<Fq>>(t);
            pow.template bind<e, util::Mul<Fq>>(t);
            Fuse::call(hypercube, std::move(t));
        }

        constexpr void bind(const Fq& e) {
            pow.bind(e);
            g2s.bind(e);
        }

        consteval std::size_t degree() const {
            return pow.degree() + b;
        }

        constexpr std::size_t variables() const {
            return pow.variables();
        }
    };

    // 从 Πꟳᴼᴸᴰ
    class GFold {
        GEval geval;
        GNorm gnorm;
    public:
        constexpr GFold(
            const std::vector<Fq>& alpha,
            const Fq& beta,
            const std::vector<Fq>& mu,
            const std::vector<std::vector<Fq>>& r,
            const std::vector<Vector<Rq>>& f
        ) : geval(alpha, r, f), gnorm(beta, mu, f) {}
        constexpr GFold(GEval&& geval, GNorm&& gnorm) : geval(std::move(geval)), gnorm(std::move(gnorm)) {}

        constexpr Fq operator () (const Point<Fq>& point) const {
            return geval(point) + gnorm(point);
        }

        template<Fq e, typename Fuse>
        constexpr void bind(std::vector<Fq>& hypercube) const {
            std::vector<Fq> t(hypercube.size());
            geval.template bind<e, util::Assign<Fq>>(t);
            gnorm.template bind<e, util::Add<Fq>>(t);
            Fuse::call(hypercube, std::move(t));
        }

        constexpr void bind(const Fq& e) {
            geval.bind(e);
            gnorm.bind(e);
        }

        consteval std::size_t degree() const {
            return gnorm.degree();
        }

        constexpr std::size_t variables() const {
            return gnorm.variables();
        }
    };
};

}

#endif

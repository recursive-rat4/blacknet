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

#include <algorithm>
#include <concepts>

#include "ajtaicommitment.h"
#include "binaryuniformdistribution.h"
#include "customizableconstraintsystem.h"
#include "eqextension.h"
#include "latticegadget.h"
#include "matrixdense.h"
#include "multilinearextension.h"
#include "point.h"
#include "polynomial.h"
#include "powextension.h"
#include "vectordense.h"
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

    using BindingCommitment = AjtaiCommitment<RqIso, NormP::Infinity>;

    template<typename Sponge>
    struct Distribution {
        using result_type = RqIso;

        BinaryUniformDistributionSponge<Sponge> bud;

        constexpr Distribution() noexcept = default;

        constexpr void reset() noexcept {
            bud.reset();
        }

        constexpr result_type operator () (Sponge& sponge) {
            Rq r;
            std::ranges::generate(r.coefficients, [&] {
                return bud(sponge).douple() - bud(sponge);
            });
            return r;
        }

    template<typename Builder>
    requires(std::same_as<Fq, typename Builder::R>)
    struct Circuit {
        using Variable = Builder::Variable;
        using LinearCombination = Builder::LinearCombination;
        using BinaryUniformDistribution = BinaryUniformDistributionSponge<Sponge>::template Circuit<Builder>;
        using RqCircuit = Rq::template Circuit<Builder>;
        using RqIsoCircuit = RqIso::template Circuit<Builder>;
        using SpongeCircuit = Sponge::template Circuit<Builder>;

        Builder* circuit;
        BinaryUniformDistribution bud;

        constexpr Circuit(Builder* circuit) : circuit(circuit), bud(circuit) {}

        constexpr void reset() noexcept {
            bud.reset();
        }

        constexpr RqIsoCircuit operator () (SpongeCircuit& sponge) {
            RqCircuit r(circuit);
            std::ranges::generate(r.coefficients, [&] {
                return bud(sponge) * Fq(2) - bud(sponge);
            });
            return r;
        }
    };

    template<std::size_t Degree>
    struct Assigner {
        using BinaryUniformDistribution = BinaryUniformDistributionSponge<Sponge>::template Assigner<Degree>;
        using RqAssigner = Rq::template Assigner<Degree>;
        using RqIsoAssigner = RqIso::template Assigner<Degree>;
        using SpongeAssigner = Sponge::template Assigner<Degree>;

        BinaryUniformDistribution bud;
        std::vector<Fq>* assigment;

        constexpr Assigner(std::vector<Fq>* assigment) : bud(assigment), assigment(assigment) {}

        constexpr void reset() noexcept {
            bud.reset();
        }

        constexpr RqIsoAssigner operator () (SpongeAssigner& sponge) {
            RqAssigner r(assigment);
            std::ranges::generate(r.polynomial.coefficients, [&] {
                return bud(sponge).douple() - bud(sponge);
            });
            return r;
        }
    };

    };

    constexpr static MatrixDense<Rq> gadget_medium(std::size_t m, std::size_t n) {
        return LatticeGadget<Rq>::matrix(B, m, n);
    }
    constexpr static MatrixDense<Rq> gadget_small(std::size_t m, std::size_t n) {
        return LatticeGadget<Rq>::matrix(b, m, n);
    }
    constexpr static VectorDense<Rq> decompose_medium(const VectorDense<Rq>& f) {
        return LatticeGadget<Rq>::decompose(B, B_digits, f);
    }
    constexpr static VectorDense<Rq> decompose_small(const VectorDense<Rq>& f) {
        return LatticeGadget<Rq>::decompose(b, b_digits, f);
    }

    class G1 {
        // r 何处
        EqExtension<Fq> eq;
        MultilinearExtension<Fq> mle;
    public:
        constexpr G1(const std::vector<Fq>& r, const VectorDense<Rq>& f) : eq(r), mle(f) {}
        constexpr G1(const Fq& alpha, const std::vector<Fq>& r, const VectorDense<Rq>& f) : eq(r, alpha), mle(f) {}
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

        constexpr G2(const VectorDense<Rq>& f) : mu(Fq::multiplicative_identity()), mle(f) {}
        constexpr G2(const Fq& mu, const VectorDense<Rq>& f) : mu(mu), mle(f) {}
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

    template<typename Builder>
    requires(std::same_as<Fq, typename Builder::R>)
    struct Circuit {
        using Variable = Builder::Variable;
        using LinearCombination = Builder::LinearCombination;
        using MultilinearExtension = MultilinearExtension<Fq>::template Circuit<Builder>;
        using Point = Point<Fq>::template Circuit<Builder>;

        Builder* circuit;
        LinearCombination mu;
        MultilinearExtension mle;

        constexpr Circuit(Builder* circuit, Variable::Type type, std::size_t variables)
            : circuit(circuit),
            mu(circuit->variable(type)),
            mle(circuit, type, variables) {}

        constexpr LinearCombination operator () (const Point& point) const {
            auto scope = circuit->scope("LatticeFold::G2::point");
            LinearCombination t = mle(point);
            // circuit degree 2
            LinearCombination tt = circuit->auxiliary();
            scope(tt == t * t);
            auto r = circuit->auxiliary();
            scope(r == mu * (tt - t));
            return r;
        }

        consteval std::size_t degree() const {
            return b;
        }

        constexpr std::size_t variables() const {
            return mle.variables();
        }
    };

    template<std::size_t Degree>
    struct Assigner {
        using MultilinearExtension = MultilinearExtension<Fq>::template Assigner<Degree>;

        Fq mu;
        MultilinearExtension mle;
        std::vector<Fq>* assigment;

        constexpr Assigner(const G2& g2, std::vector<Fq>* assigment)
            : mu(g2.mu), mle(g2.mle, assigment), assigment(assigment) {}

        constexpr Fq operator () (const Point<Fq>& point) const {
            Fq t = mle(point);
            return assigment->emplace_back(
                mu * (assigment->emplace_back(
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
            const std::vector<VectorDense<Rq>>& f
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
            const std::vector<VectorDense<Rq>>& f
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
            const std::vector<VectorDense<Rq>>& f
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

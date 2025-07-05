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

#include <boost/test/unit_test.hpp>

#include "circuitbuilder.h"
#include "customizableconstraintsystem.h"
#include "hypercube.h"
#include "latticefold.h"
#include "lm62.h"
#include "lm62extension.h"
#include "matrixdense.h"
#include "point.h"
#include "poseidon2lm62.h"
#include "sumcheck.h"
#include "vectordense.h"

using namespace blacknet::crypto;

BOOST_AUTO_TEST_SUITE(LatticeFold_Plain)

using Z = LM62Ring;
using F = LM62RingDegree2;
using R = LM62RingDegree64;
using RIso = LM62RingDegree64NTT;
using LatticeFold = LatticeFold<Z, F, R, RIso>;

BOOST_AUTO_TEST_CASE(Gadget) {
    const auto a = VectorDense<R>{ R(3), R(2), R(1), R(0) };
    const auto b = VectorDense<R>{ R(4295098371) };
    auto g = LatticeFold::gadget_medium(1, 4);
    BOOST_TEST(b == g * a);
    auto c = LatticeFold::decompose_medium(b);
    BOOST_TEST(a == c);
}

BOOST_AUTO_TEST_CASE(G1s) {
    Point<F> r1{Z(0), Z(0), Z(0), Z(0), Z(0), Z(0)};
    Point<F> r2{Z(0), Z(0), Z(0), Z(0), Z(0), Z(1)};
    VectorDense<R> f{R{3, 4}};
    auto g1_1 = LatticeFold::G1(r1.coordinates, f);
    auto g1_2 = LatticeFold::G1(r2.coordinates, f);
    BOOST_TEST(6 == g1_1.variables());
    BOOST_TEST(2 == g1_1.degree());
    BOOST_TEST(F(3) == g1_1(r1));
    BOOST_TEST(F(0) == g1_1(r2));
    BOOST_TEST(F(4) == g1_2(r2));
    BOOST_TEST(F(0) == g1_2(r1));
}

BOOST_AUTO_TEST_CASE(G2s) {
    VectorDense<R> f1{R{1, -1}};
    VectorDense<R> f2{R{2, -2}};
    VectorDense<R> f3{R{1, 1, 0, 1}};
    auto g2_1 = LatticeFold::G2(f1);
    auto g2_2 = LatticeFold::G2(f2);
    auto g2_3 = LatticeFold::G2(f3);
    BOOST_TEST(6 == g2_1.variables());
    BOOST_TEST(2 == g2_1.degree());
    BOOST_TEST(F(0) != Hypercube<F>::sum(g2_1));
    BOOST_TEST(F(0) != Hypercube<F>::sum(g2_2));
    BOOST_TEST(F(0) == Hypercube<F>::sum(g2_3));
}

BOOST_AUTO_TEST_CASE(GEvals) {
    std::vector<F> alpha(LatticeFold::k * 2, Z(2));
    std::vector<std::vector<F>> r(LatticeFold::k * 2, {Z(0), Z(0), Z(0), Z(0), Z(1), Z(0)});
    std::vector<VectorDense<R>> f;
    for (std::size_t i = 0; i < LatticeFold::k * 2; ++i) {
        R rq(0);
        rq.coefficients[i] = Z(i);
        f.emplace_back(VectorDense<R>{rq});
    }
    auto geval = LatticeFold::GEval(alpha, r, f);
    BOOST_TEST(6 == geval.variables());
    BOOST_TEST(2 == geval.degree());
    BOOST_TEST(F(0) == geval({Z(0), Z(0), Z(0), Z(0), Z(0), Z(1)}));
    BOOST_TEST(F(4) == geval({Z(0), Z(0), Z(0), Z(0), Z(1), Z(0)}));
}

BOOST_AUTO_TEST_CASE(GNorms) {
    F beta(2);
    std::vector<F> mu(LatticeFold::k * 2, Z(1));
    std::vector<VectorDense<R>> f1(LatticeFold::k * 2, VectorDense<R>{R{1, 1, 0, -1}});
    std::vector<VectorDense<R>> f2(LatticeFold::k * 2, VectorDense<R>{R{2, 0, 0, -2}});
    std::vector<VectorDense<R>> f3(LatticeFold::k * 2, VectorDense<R>{R{1, 0, 1, 1}});
    auto gnorm_1 = LatticeFold::GNorm(beta, mu, f1);
    auto gnorm_2 = LatticeFold::GNorm(beta, mu, f2);
    auto gnorm_3 = LatticeFold::GNorm(beta, mu, f3);
    BOOST_TEST(6 == gnorm_1.variables());
    BOOST_TEST(3 == gnorm_2.degree());
    BOOST_TEST(F(0) != Hypercube<F>::sum(gnorm_1));
    BOOST_TEST(F(0) != Hypercube<F>::sum(gnorm_2));
    BOOST_TEST(F(0) == Hypercube<F>::sum(gnorm_3));
}

BOOST_AUTO_TEST_CASE(GFolds) {
    std::vector<F> alpha(LatticeFold::k * 2, Z(1));
    F beta(3);
    std::vector<F> mu(LatticeFold::k * 2, Z(1));
    std::vector<std::vector<F>> r(LatticeFold::k * 2, {Z(0), Z(0), Z(0), Z(0), Z(1), Z(1)});
    std::vector<VectorDense<R>> f(LatticeFold::k * 2, VectorDense<R>{R{1, 0, 1, 1, 0, 1}});
    auto gfold = LatticeFold::GFold(alpha, beta, mu, r, f);
    BOOST_TEST(6 == gfold.variables());
    BOOST_TEST(3 == gfold.degree());
    BOOST_TEST(F(32) == Hypercube<F>::sum(gfold));
}

BOOST_AUTO_TEST_CASE(RingIsomorphisms) {
    const R a({4, 0, 0, 1, 5,});
    RIso b(a);
    R c = b.isomorph();
    BOOST_TEST(a == c);
}

BOOST_AUTO_TEST_SUITE_END()

BOOST_AUTO_TEST_SUITE(LatticeFold_Circuit)

using Z = LM62Ring;
using F = LM62Ring; //XXX
using R = LM62RingDegree64;
using RIso = LM62RingDegree64NTT;
using LatticeFold = LatticeFold<Z, F, R, RIso>;
using Duplex = Poseidon2LM62Sponge<{64, 65, 66, 67}>;

BOOST_AUTO_TEST_CASE(Distributions) {
    Duplex duplex_plain;
    LatticeFold::Distribution<Duplex> distribution_plain;
    RIso a_plain = distribution_plain(duplex_plain);

    using Builder = CircuitBuilder<F, 2>;
    Builder circuit;
    using DuplexCircuit = Duplex::Circuit<Builder>;
    DuplexCircuit duplex_circuit(&circuit);
    using DistributionCircuit = LatticeFold::Distribution<Duplex>::Circuit<Builder>;
    DistributionCircuit distribution_circuit(&circuit);
    using RIsoCircuit = RIso::Circuit<Builder>;
    RIsoCircuit a_circuit(&circuit);
    a_circuit = distribution_circuit(duplex_circuit);
    for (std::size_t i = 0; i < RIso::dimension(); ++i)
        circuit(a_plain[i] == a_circuit[i]);

    CustomizableConstraintSystem<F> ccs(circuit.ccs());
    VectorDense<F> z = ccs.assigment();

    using DuplexAssigner = Duplex::Assigner<Builder::degree()>;
    DuplexAssigner duplex_assigner(&z.elements);
    using DistributionAssigner = LatticeFold::Distribution<Duplex>::Assigner<Builder::degree()>;
    DistributionAssigner distribution_assigner(&z.elements);
    using RIsoAssigner = RIso::Assigner<Builder::degree()>;
    RIsoAssigner a_assigner = distribution_assigner(duplex_assigner);
    BOOST_TEST(a_plain == a_assigner.polynomial);
    BOOST_TEST(ccs.isSatisfied(z));
}

BOOST_AUTO_TEST_CASE(G2s) {
    constexpr std::size_t ell = std::countr_zero(LatticeFold::D);
    VectorDense<R> f{R{1, 1, 0, 1}};
    auto g2 = LatticeFold::G2(f);
    Point<F> x(ell);
    std::ranges::fill(x.coordinates, F(42));

    using Builder = CircuitBuilder<F, 2>;
    Builder circuit;
    using G2Circuit = LatticeFold::G2::Circuit<Builder>;
    G2Circuit g2_circuit(&circuit, Builder::Variable::Type::Input, ell);
    using PointCircuit = Point<F>::Circuit<Builder>;
    PointCircuit x_circuit(&circuit, Builder::Variable::Type::Input, ell);
    g2_circuit(x_circuit);

    CustomizableConstraintSystem<F> ccs(circuit.ccs());
    VectorDense<F> z = ccs.assigment();
    z.elements.push_back(g2.mu);
    std::ranges::copy(g2.mle.coefficients, std::back_inserter(z.elements));
    std::ranges::copy(x.coordinates, std::back_inserter(z.elements));

    using Assigner = LatticeFold::G2::Assigner<Builder::degree()>;
    Assigner g2_assigner(g2, &z.elements);
    BOOST_TEST(g2(x) == g2_assigner(x));
    BOOST_TEST(ccs.isSatisfied(z));
}

BOOST_AUTO_TEST_CASE(Verifys) {
    // Far from being complete...
    using SumCheck = SumCheck<F, LatticeFold::G2, Duplex>;
    Duplex duplex;
    constexpr std::size_t ell = std::countr_zero(LatticeFold::D);
    VectorDense<R> f{R{1, 1, 0, 1}};
    auto g2 = LatticeFold::G2(f);
    F sum = Hypercube<F>::sum(g2);

    auto proof = SumCheck::prove(g2, sum, duplex);

    using Builder = CircuitBuilder<F, 2>;
    Builder circuit;
    using G2Circuit = LatticeFold::G2::Circuit<Builder>;
    G2Circuit g2_circuit(&circuit, Builder::Variable::Type::Input, ell);
    auto sum_var = circuit.input();
    using ProofCircuit = SumCheck::Proof::Circuit<Builder>;
    ProofCircuit proof_circuit(&circuit, Builder::Variable::Type::Input, g2.variables(), g2.degree());
    using SumCheckCircuit = SumCheck::Circuit<Builder>;
    SumCheckCircuit sumcheck_circuit(&circuit);
    using DuplexCircuit = Duplex::Circuit<Builder>;
    DuplexCircuit duplex_circuit(&circuit);
    sumcheck_circuit.verifyEarlyStopping(g2_circuit, sum_var, proof_circuit, duplex_circuit);

    CustomizableConstraintSystem<F> ccs(circuit.ccs());
    VectorDense<F> z = ccs.assigment();
    z.elements.push_back(g2.mu);
    std::ranges::copy(g2.mle.coefficients, std::back_inserter(z.elements));
    z.elements.push_back(sum);
    for (const auto& claim : proof.claims)
        std::ranges::copy(claim.coefficients, std::back_inserter(z.elements));

    SumCheck::Assigner<Builder::degree()> assigner(&z.elements);
    using DuplexAssigner = Duplex::Assigner<Builder::degree()>;
    DuplexAssigner duplex_assigner(&z.elements);
    BOOST_TEST_REQUIRE(assigner.verifyEarlyStopping(g2, sum, proof, duplex_assigner).has_value());
    BOOST_TEST(ccs.isSatisfied(z));
}

BOOST_AUTO_TEST_SUITE_END()

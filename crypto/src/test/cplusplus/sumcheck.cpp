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
#include "eqextension.h"
#include "multilinearextension.h"
#include "poseidon2solinas62.h"
#include "powextension.h"
#include "solinas62.h"
#include "solinas62extension.h"
#include "sumcheck.h"

using namespace blacknet::crypto;

BOOST_AUTO_TEST_SUITE(SumChecks)

using Z = Solinas62Ring;
using R = Solinas62RingDegree2;
using Duplex = Poseidon2Solinas62Sponge<{10, 11, 12, 13}>;

BOOST_AUTO_TEST_CASE(mle) {
    using SumCheck = SumCheck<R, MultilinearExtension<R>, Duplex>;
    Duplex duplex;
    MultilinearExtension<R> p1{Z(7), Z(7), Z(7), Z(0)};
    MultilinearExtension<R> p2{Z(7), Z(7), Z(7), Z(7)};
    MultilinearExtension<R> p3{Z(7), Z(7), Z(0), Z(7)};
    R s1(21);
    R s2(28);

    auto proof = SumCheck::prove(p1, s1, duplex);
    duplex.reset();

    BOOST_TEST(SumCheck::verify(p1, s1, proof, duplex));
    duplex.reset();

    BOOST_TEST(!SumCheck::verify(p1, s2, proof, duplex));
    duplex.reset();

    BOOST_TEST(!SumCheck::verify(p2, s1, proof, duplex));
    duplex.reset();

    BOOST_TEST(!SumCheck::verify(p2, s2, proof, duplex));
    duplex.reset();

    BOOST_TEST(!SumCheck::verify(p3, s1, proof, duplex));
    duplex.reset();

    proof.claims[1].coefficients[1].coefficients[1] += Z(1);
    BOOST_TEST(!SumCheck::verify(p1, s1, proof, duplex));
    duplex.reset();

    auto proof2 = SumCheck::prove(p1, s2, duplex);
    duplex.reset();

    BOOST_TEST(!SumCheck::verify(p1, s1, proof2, duplex));
    duplex.reset();

    BOOST_TEST(!SumCheck::verify(p1, s2, proof2, duplex));
    duplex.reset();
}

BOOST_AUTO_TEST_CASE(eq) {
    using SumCheck = SumCheck<R, EqExtension<R>, Duplex>;
    Duplex duplex;
    EqExtension<R> p1({Z(45), Z(46), Z(47), Z(48)});
    EqExtension<R> p2({Z(45), Z(46), Z(48), Z(48)});
    R s1(1);
    R s2(2);

    auto proof = SumCheck::prove(p1, s1, duplex);
    duplex.reset();

    BOOST_TEST(SumCheck::verify(p1, s1, proof, duplex));
    duplex.reset();

    BOOST_TEST(!SumCheck::verify(p1, s2, proof, duplex));
    duplex.reset();

    BOOST_TEST(!SumCheck::verify(p2, s1, proof, duplex));
    duplex.reset();

    BOOST_TEST(!SumCheck::verify(p2, s2, proof, duplex));
    duplex.reset();

    proof.claims[3].coefficients[1].coefficients[1] += Z(1);
    BOOST_TEST(!SumCheck::verify(p1, s1, proof, duplex));
    duplex.reset();

    auto proof2 = SumCheck::prove(p1, s2, duplex);
    duplex.reset();

    BOOST_TEST(!SumCheck::verify(p1, s1, proof2, duplex));
    duplex.reset();

    BOOST_TEST(!SumCheck::verify(p1, s2, proof2, duplex));
    duplex.reset();
}

BOOST_AUTO_TEST_CASE(ccs) {
    using CCS = CustomizableConstraintSystem<R>;
    using SumCheck = SumCheck<R, CCS::Polynomial, Duplex>;
    Duplex duplex;
    CCS::Polynomial ccs(1, 2, {{Z(7), Z(7), Z(7), Z(0)}}, {{0}}, {Z(1)});
    R s(21);

    auto proof = SumCheck::prove(ccs, s, duplex);
    duplex.reset();

    BOOST_TEST(SumCheck::verify(ccs, s, proof, duplex));
    duplex.reset();
}

BOOST_AUTO_TEST_CASE(pow_early_stop) {
    using SumCheck = SumCheck<R, PowExtension<R>, Duplex>;
    Duplex duplex;
    PowExtension<R> p1(R(2), 4);
    PowExtension<R> p2(R(4), 4);
    R s1(1);
    R s2(2);

    auto proof = SumCheck::prove(p1, s1, duplex);
    duplex.reset();

    auto maybe = SumCheck::verifyEarlyStopping(p1, s1, proof, duplex);
    BOOST_TEST_REQUIRE(maybe.has_value());
    auto&& [point, state] = *maybe;
    BOOST_TEST(state == p1(point));
    duplex.reset();

    BOOST_TEST(!SumCheck::verifyEarlyStopping(p1, s2, proof, duplex).has_value());
    duplex.reset();

    BOOST_TEST(!SumCheck::verifyEarlyStopping(p2, s2, proof, duplex).has_value());
    duplex.reset();

    proof.claims[3].coefficients[1] += Z(1);
    BOOST_TEST(!SumCheck::verifyEarlyStopping(p1, s1, proof, duplex).has_value());
    duplex.reset();
    proof.claims[3].coefficients[1] -= Z(1);

    auto proof2 = SumCheck::prove(p1, s2, duplex);
    duplex.reset();

    BOOST_TEST(!SumCheck::verifyEarlyStopping(p1, s1, proof2, duplex).has_value());
    duplex.reset();
}

BOOST_AUTO_TEST_CASE(circuit) {
    using SumCheck = SumCheck<Z, MultilinearExtension<Z>, Duplex>;
    Duplex duplex;
    MultilinearExtension<Z> poly{Z(7), Z(7), Z(7), Z(0)};
    Z sum(21);

    auto proof = SumCheck::prove(poly, sum, duplex);

    using Builder = CircuitBuilder<Z, 2>;
    Builder circuit;
    using PolyCircuit = MultilinearExtension<Z>::Circuit<Builder>;
    PolyCircuit poly_circuit(circuit, Builder::Variable::Type::Input, poly.variables());
    auto sum_var = circuit.input();
    using ProofCircuit = SumCheck::Proof::Circuit<Builder>;
    ProofCircuit proof_circuit(circuit, Builder::Variable::Type::Input, poly.variables(), poly.degree());
    using SumCheckCircuit = SumCheck::Circuit<Builder>;
    SumCheckCircuit sumcheck_circuit(circuit);
    using DuplexCircuit = Duplex::Circuit<Builder>;
    DuplexCircuit duplex_circuit(circuit);
    sumcheck_circuit.verify(poly_circuit, sum_var, proof_circuit, duplex_circuit);

    CustomizableConstraintSystem<Z> ccs(circuit.ccs());
    Vector<Z> z = ccs.assigment();
    std::ranges::copy(poly.coefficients, std::back_inserter(z.elements));
    z.elements.push_back(sum);
    for (const auto& claim : proof.claims)
        std::ranges::copy(claim.coefficients, std::back_inserter(z.elements));
    SumCheck::Assigner<Builder::degree()> assigner(z.elements);
    using DuplexAssigner = Duplex::Assigner<Builder::degree()>;
    DuplexAssigner duplex_assigner(z.elements);
    BOOST_TEST_REQUIRE(assigner.verify(poly, sum, proof, duplex_assigner));
    BOOST_TEST(ccs.isSatisfied(z));
}

BOOST_AUTO_TEST_SUITE_END()

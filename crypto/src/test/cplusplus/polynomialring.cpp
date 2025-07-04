/*
 * Copyright (c) 2025 Pavel Vasin
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
#include "fastrng.h"
#include "lm62extension.h"
#include "polynomialring.h"
#include "r1cs.h"

using namespace blacknet::crypto;

using R = LM62RingDegree64NTT;
using Z = R::BaseRing;

static FastDRG rng;

BOOST_AUTO_TEST_SUITE(PolynomialRing_Circuit)

BOOST_AUTO_TEST_CASE(Add) {
    R a = R::random(rng);
    R b = R::random(rng);
    R c = a + b;

    using Builder = CircuitBuilder<Z, 2>;
    Builder circuit;
    using RCircuit = R::Circuit<Builder>;
    RCircuit a_input(&circuit, Builder::Variable::Type::Input);
    RCircuit b_input(&circuit, Builder::Variable::Type::Input);
    RCircuit c_input(&circuit, Builder::Variable::Type::Input);
    RCircuit c_circuit = a_input + b_input;
    for (std::size_t i = 0; i < R::dimension(); ++i) {
        circuit(c_input[i] == c_circuit[i]);
    }

    R1CS<Z> r1cs(circuit.r1cs());
    VectorDense<Z> z = r1cs.assigment();
    std::ranges::copy(a, std::back_inserter(z.elements));
    std::ranges::copy(b, std::back_inserter(z.elements));
    std::ranges::copy(c, std::back_inserter(z.elements));

    using RAssigner = R::Assigner<Builder::degree()>;
    RAssigner a_assigner(a, &z.elements);
    RAssigner b_assigner(b, &z.elements);
    RAssigner c_assigner = a_assigner + b_assigner;
    BOOST_TEST(c == c_assigner.polynomial);
    BOOST_TEST(r1cs.isSatisfied(z));
}

BOOST_AUTO_TEST_CASE(Mul) {
    R a = R::random(rng);
    R b = R::random(rng);
    R c = a * b;

    using Builder = CircuitBuilder<Z, 2>;
    Builder circuit;
    using RCircuit = R::Circuit<Builder>;
    RCircuit a_input(&circuit, Builder::Variable::Type::Input);
    RCircuit b_input(&circuit, Builder::Variable::Type::Input);
    RCircuit c_input(&circuit, Builder::Variable::Type::Input);
    RCircuit c_circuit = a_input * b_input;
    for (std::size_t i = 0; i < R::dimension(); ++i) {
        circuit(c_input[i] == c_circuit[i]);
    }

    R1CS<Z> r1cs(circuit.r1cs());
    VectorDense<Z> z = r1cs.assigment();
    std::ranges::copy(a, std::back_inserter(z.elements));
    std::ranges::copy(b, std::back_inserter(z.elements));
    std::ranges::copy(c, std::back_inserter(z.elements));

    using RAssigner = R::Assigner<Builder::degree()>;
    RAssigner a_assigner(a, &z.elements);
    RAssigner b_assigner(b, &z.elements);
    RAssigner c_assigner = a_assigner * b_assigner;
    BOOST_TEST(c == c_assigner.polynomial);
    BOOST_TEST(r1cs.isSatisfied(z));
}

BOOST_AUTO_TEST_SUITE_END()

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
#include "lm62extension.h"
#include "numbertheoretictransform.h"
#include "r1cs.h"

using namespace blacknet::crypto;

using Z = LM62Ring;
using R = LM62RingDegree64;

BOOST_AUTO_TEST_SUITE(NTT_Circuit)

BOOST_AUTO_TEST_CASE(test) {
    R a{ 3, 2, 1, };
    R b{ 4, 5, 6, };
    R c = a * b;

    using Builder = CircuitBuilder<Z, 2>;
    Builder circuit;
    using NTTCircuit = NTT<Z, R::dimension()>::Circuit<Builder>;
    NTTCircuit ntt_circuit(circuit);
    using RCircuit = std::array<Builder::LinearCombination, R::dimension()>;
    RCircuit a_input;
    std::ranges::generate(a_input, [&] { return circuit.input(); });
    RCircuit b_input;
    std::ranges::generate(b_input, [&] { return circuit.input(); });
    RCircuit c_input;
    std::ranges::generate(c_input, [&] { return circuit.input(); });
    RCircuit c_circuit;
    ntt_circuit.cooley_tukey(a_input);
    ntt_circuit.cooley_tukey(b_input);
    ntt_circuit.convolute(c_circuit, a_input, b_input);
    ntt_circuit.gentleman_sande(c_circuit);
    for (std::size_t i = 0; i < R::dimension(); ++i) {
        circuit(c_input[i] == c_circuit[i]);
    }

    R1CS<Z> r1cs(circuit.r1cs());
    VectorDense<Z> z = r1cs.assigment();
    std::ranges::copy(a.coefficients, std::back_inserter(z.elements));
    std::ranges::copy(b.coefficients, std::back_inserter(z.elements));
    std::ranges::copy(c.coefficients, std::back_inserter(z.elements));

    using NTTAssigner = NTT<Z, R::dimension()>::Assigner<Builder::degree()>;
    NTTAssigner ntt_assigner(z.elements);
    ntt_assigner.cooley_tukey(a.coefficients);
    ntt_assigner.cooley_tukey(b.coefficients);
    R c_assigned;
    ntt_assigner.convolute(c_assigned.coefficients, a.coefficients, b.coefficients);
    ntt_assigner.gentleman_sande(c_assigned.coefficients);
    BOOST_TEST(c == c_assigned);
    BOOST_TEST(r1cs.isSatisfied(z));
}

BOOST_AUTO_TEST_SUITE_END()

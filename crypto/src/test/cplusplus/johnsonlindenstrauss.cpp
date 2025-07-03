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
#include <random>

#include "circuitbuilder.h"
#include "fastrng.h"
#include "johnsonlindenstrauss.h"
#include "lm62.h"
#include "matrixdense.h"
#include "poseidon2lm62.h"
#include "r1cs.h"
#include "vectordense.h"

using namespace blacknet::crypto;

using Z = LM62Ring;
using JL = JohnsonLindenstrauss<Z>;

BOOST_AUTO_TEST_SUITE(JohnsonLindenstrauss_Plain)

using NumericType = Z::NumericType;

static FastDRG rng;

BOOST_AUTO_TEST_CASE(test) {
    NumericType b = 128;
    std::size_t n = 16;
    std::size_t k = 32;
    NumericType slack_inf = 6;
    NumericType slack_ecd = 3;

    std::uniform_int_distribution<NumericType> dst{-b+1, +b-1};
    MatrixDense<Z> map = JL::random(rng, n, k);
    VectorDense<Z> high = VectorDense<Z>::random(rng, dst, k);
    VectorDense<Z> low = JL::project(map, high);

    BOOST_TEST(map * high == low);
    BOOST_TEST(low.checkInfinityNorm(b * slack_inf));
    BOOST_TEST(low.euclideanNorm() < high.euclideanNorm() * slack_ecd);
}

BOOST_AUTO_TEST_SUITE_END()

BOOST_AUTO_TEST_SUITE(JohnsonLindenstrauss_Circuit)

using Sponge = Poseidon2LM62Sponge<{0, 1, 1, 0}>;

BOOST_AUTO_TEST_CASE(distribution) {
    using Builder = CircuitBuilder<Z, 2>;
    Builder circuit;
    using SpongeCircuit = Sponge::Circuit<Builder>;
    SpongeCircuit sponge_circuit(circuit);
    using DistributionCircuit = JL::DistributionSponge<Sponge>::Circuit<Builder>;
    DistributionCircuit distribution_circuit(circuit);
    using VectorDenseCircuit = VectorDense<Z>::Circuit<Builder>;
    VectorDenseCircuit v_circuit(circuit, 32);
    std::ranges::generate(v_circuit, [&] { return distribution_circuit(sponge_circuit); });

    R1CS<Z> r1cs(circuit.r1cs());
    VectorDense<Z> z = r1cs.assigment();

    using SpongeAssigner = Sponge::Assigner<Builder::degree()>;
    SpongeAssigner sponge_assigner(z.elements);
    using DistributionAssigner = JL::DistributionSponge<Sponge>::Assigner<Builder::degree()>;
    DistributionAssigner distribution_assigner(z.elements);
    VectorDense<Z> v_assigned(32);
    std::ranges::generate(v_assigned, [&] { return distribution_assigner(sponge_assigner); });
    BOOST_TEST(r1cs.isSatisfied(z));
}

BOOST_AUTO_TEST_SUITE_END()

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
#include <algorithm>
#include <array>
#include <cstdint>
#include <limits>

#include "binaryuniformdistribution.h"
#include "circuitbuilder.h"
#include "poseidon2lm62.h"
#include "r1cs.h"

using namespace blacknet::crypto;

BOOST_AUTO_TEST_SUITE(BinaryUniformDistribution_Plain)

struct FixedGenerator {
    using result_type = std::uint16_t;

    result_type i{1234};

    result_type operator () () {
        return i++;
    }

    consteval static result_type min() {
        return std::numeric_limits<result_type>::min();
    }
    consteval static result_type max() {
        return std::numeric_limits<result_type>::max();
    }
};

BOOST_AUTO_TEST_CASE(Reproducible) {
    FixedGenerator g;
    BinaryUniformDistributionRNG<uint8_t, FixedGenerator> bud;
    const std::array<uint8_t, 16> a{0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0};
    std::array<uint8_t, 16> b;
    std::ranges::generate(b, [&]() { return bud(g); });
    BOOST_TEST(a == b);
}

BOOST_AUTO_TEST_SUITE_END()

BOOST_AUTO_TEST_SUITE(BinaryUniformDistribution_Circuit)

using Z = LM62Ring;
using Sponge = Poseidon2LM62Sponge<{32, 33, 34, 35}>;

BOOST_AUTO_TEST_CASE(test) {
    Sponge sponge;
    BinaryUniformDistributionSponge<Sponge> bud;
    std::array<Z, Z::bits() - 1> a;
    std::ranges::generate(a, [&] { return bud(sponge); });

    using Builder = CircuitBuilder<Z, 2>;
    Builder circuit;
    using SpongeCircuit = Sponge::Circuit<Builder>;
    SpongeCircuit sponge_circuit(circuit);
    using Circuit = BinaryUniformDistributionSponge<Sponge>::Circuit<Builder>;
    Circuit bud_circuit(circuit);
    for (std::size_t i = 0; i < a.size(); ++i)
        bud_circuit(sponge_circuit);

    R1CS<Z> r1cs(circuit.r1cs());
    Vector<Z> z = r1cs.assigment();

    using SpongeTracer = Sponge::Tracer<Builder::degree()>;
    SpongeTracer sponge_tracer(z.elements);
    using Tracer = BinaryUniformDistributionSponge<Sponge>::Tracer<Builder::degree()>;
    Tracer bud_tracer(z.elements);
    std::array<Z, a.size()> a_traced;
    std::ranges::generate(a_traced, [&] { return bud_tracer(sponge_tracer); });
    BOOST_TEST(a == a_traced);
    BOOST_TEST(r1cs.isSatisfied(z));
}

BOOST_AUTO_TEST_SUITE_END()

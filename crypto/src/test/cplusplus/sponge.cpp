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

#include "ccsbuilder.h"
#include "circuitry.h"
#include "customizableconstraintsystem.h"
#include "poseidon2solinas62.h"
#include "sponge.h"

using namespace blacknet::crypto;

BOOST_AUTO_TEST_SUITE(Sponges)

BOOST_AUTO_TEST_CASE(BlacknetPlain) {
    using Z = uint8_t;
    using B = std::array<Z, 4>;
    struct F {
        consteval static std::size_t width() {
            return 4;
        }
        constexpr static void permute(B& b) {
            for (auto& e : b) e += 1;
        }
    };
    using S = Sponge<Z, 2, 2, {10, 0}, F, SpongeMode::Overwrite>;

    S sponge1;
    BOOST_TEST((B{0, 0, 10, 0} == sponge1.state));
    sponge1.absorb(2);
    BOOST_TEST((B{2, 0, 10, 0} == sponge1.state));
    sponge1.absorb(4);
    BOOST_TEST((B{2, 4, 10, 0} == sponge1.state));
    sponge1.absorb(6);
    BOOST_TEST((B{6, 5, 11, 1} == sponge1.state));

    S sponge2(sponge1);
    sponge2.absorb(8);
    BOOST_TEST((B{6, 8, 11, 1} == sponge2.state));

    BOOST_TEST((Z{7} == sponge1.squeeze()));
    BOOST_TEST((B{7, 2, 12, 4} == sponge1.state));
    BOOST_TEST((Z{2} == sponge1.squeeze()));
    BOOST_TEST((B{7, 2, 12, 4} == sponge1.state));
    BOOST_TEST((Z{8} == sponge1.squeeze()));
    BOOST_TEST((B{8, 3, 13, 5} == sponge1.state));
    BOOST_CHECK_THROW(sponge1.absorb(0), SpongeException);

    sponge2.absorb(10);
    BOOST_TEST((B{10, 9, 12, 2} == sponge2.state));
    sponge2.absorb(12);
    BOOST_TEST((B{10, 12, 12, 2} == sponge2.state));
    BOOST_TEST((Z{11} == sponge2.squeeze()));
    BOOST_TEST((B{11, 13, 13, 4} == sponge2.state));

    S sponge3;
    BOOST_TEST((Z{2} == sponge3.squeeze()));
    BOOST_TEST((B{2, 1, 11, 3} == sponge3.state));
}

BOOST_AUTO_TEST_CASE(BlacknetCircuit) {
    using Sponge = Poseidon2Solinas62Sponge<{33, 34, 35, 36}>;
    using E = Solinas62Ring;
    const std::size_t T = 12;
    Sponge sponge;
    const std::array<E, T> b{
        0x0000000000000000,
        0x0000000000000001,
        0x0000000000000002,
        0x0000000000000003,
        0x0000000000000004,
        0x0000000000000005,
        0x0000000000000006,
        0x0000000000000007,
        0x0000000000000008,
        0x0000000000000009,
        0x000000000000000a,
        0x000000000000000b,
    };
    std::array<E, T> c;

    using Circuit = CCSBuilder<E, 3>;
    Circuit circuit;
    using Gadget = Sponge::Gadget<Circuit>;
    Gadget gadget(circuit);
    std::array<typename Circuit::LinearCombination, T> inputs;
    std::ranges::generate(inputs, [&]{ return circuit.input(); });
    std::array<typename Circuit::LinearCombination, T> outputs;
    gadget.absorb(inputs);
    gadget.squeeze(outputs);
    for (std::size_t i = 0; i < T; ++i) {
        auto v = circuit.auxiliary();
        circuit(v == outputs[i]);
    }

    CustomizableConstraintSystem<E> ccs(circuit.ccs());
    Vector<E> z;
    z.elements.reserve(ccs.variables());
    z.elements.emplace_back(E(1));
    std::ranges::copy(b, std::back_inserter(z.elements));
    using Tracer = Sponge::Tracer<Circuit::degree()>;
    Tracer tracer(sponge, z.elements);
    tracer.absorb(b);
    tracer.squeeze(c);
    std::ranges::copy(c, std::back_inserter(z.elements));
    test::circuitry(ccs, z);

    Sponge spng;
    for (const auto& i : b) spng.absorb(i);
    for (const auto& i : c) BOOST_TEST(i == spng.squeeze());
}

BOOST_AUTO_TEST_SUITE_END()

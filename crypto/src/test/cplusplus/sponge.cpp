/*
 * Copyright (c) 2024 Pavel Vasin
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
#include "poseidon2solinas62.h"
#include "sponge.h"

BOOST_AUTO_TEST_SUITE(Sponges)

BOOST_AUTO_TEST_CASE(test) {
    using Z = uint8_t;
    using B = std::array<Z, 4>;
    struct F {
        constexpr static void permute(B& b) {
            for (auto& e : b) e += 1;
        }
    };
    using S = Sponge<Z, 2, 2, F>;

    S sponge1;
    BOOST_TEST((B{0, 0, 0, 0} == sponge1.state));
    sponge1.absorb(2);
    BOOST_TEST((B{2, 0, 0, 0} == sponge1.state));
    sponge1.absorb(4);
    BOOST_TEST((B{2, 4, 0, 0} == sponge1.state));
    sponge1.absorb(6);
    BOOST_TEST((B{6, 5, 1, 1} == sponge1.state));

    S sponge2(sponge1);
    sponge2.absorb(8);
    BOOST_TEST((B{6, 8, 1, 1} == sponge2.state));

    BOOST_TEST((Z{7} == sponge1.squeeze()));
    BOOST_TEST((B{7, 2, 2, 4} == sponge1.state));
    BOOST_TEST((Z{2} == sponge1.squeeze()));
    BOOST_TEST((B{7, 2, 2, 4} == sponge1.state));
    BOOST_TEST((Z{8} == sponge1.squeeze()));
    BOOST_TEST((B{8, 3, 3, 5} == sponge1.state));
    BOOST_CHECK_THROW(sponge1.absorb(0), SpongeException);

    sponge2.absorb(10);
    BOOST_TEST((B{10, 9, 2, 2} == sponge2.state));
    sponge2.absorb(12);
    BOOST_TEST((B{10, 12, 2, 2} == sponge2.state));
    BOOST_TEST((Z{11} == sponge2.squeeze()));
    BOOST_TEST((B{11, 13, 3, 4} == sponge2.state));

    S sponge3;
    BOOST_TEST((Z{2} == sponge3.squeeze()));
    BOOST_TEST((B{2, 1, 1, 3} == sponge3.state));
}

BOOST_AUTO_TEST_CASE(circuit) {
    using Sponge = Poseidon2Solinas62;
    using E = Solinas62Ring;
    const std::size_t T = 12;
    std::array<E, T> a;
    a.fill(E(0));
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
    std::array<typename Circuit::LinearCombination, T> state;
    std::array<typename Circuit::LinearCombination, T> absorb;
    std::ranges::generate(absorb, [&]{ return circuit.input(); });
    std::array<typename Circuit::LinearCombination, T> squeeze;
    Sponge::circuit<Circuit>::fixed(circuit, 0, state, absorb, squeeze);
    for (std::size_t i = 0; i < T; ++i) {
        auto v = circuit.auxiliary();
        circuit(v == squeeze[i]);
    }

    CustomizableConstraintSystem<E> ccs(circuit.ccs());
    Vector<E> z;
    z.elements.reserve(ccs.variables());
    z.elements.emplace_back(E(1));
    std::ranges::copy(b, std::back_inserter(z.elements));
    Sponge::trace<Circuit::degree()>::fixed(0, a, b, c, z.elements);
    std::ranges::copy(c, std::back_inserter(z.elements));
    BOOST_TEST(ccs.variables() == z.size());
    BOOST_TEST(ccs.isSatisfied(z));
    for (std::size_t i = 1; i < z.size(); ++i) {
        z[i] += E(1);
        BOOST_TEST(!ccs.isSatisfied(z));
        z[i] -= E(1);
    }

    Sponge sponge;
    for (const auto& i : b) sponge.absorb(i);
    for (const auto& i : c) BOOST_TEST(i == sponge.squeeze());
}

BOOST_AUTO_TEST_SUITE_END()

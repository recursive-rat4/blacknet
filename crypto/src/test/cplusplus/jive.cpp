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

#include "ccsbuilder.h"
#include "jive.h"
#include "poseidon2pervushin.h"
#include "r1cs.h"

BOOST_AUTO_TEST_SUITE(Jives)

BOOST_AUTO_TEST_CASE(test) {
    using Z = uint8_t;
    using B = std::array<Z, 4>;
    struct F {
        constexpr static void permute(B& b) {
            for (std::size_t i = 0; i < b.size(); ++i) b[i] += i + 1;
        }
    };
    using J = Jive<Z, 2, F>;
    using H = J::Hash;

    BOOST_TEST((H{52, 58} == J::compress(H{11, 12}, H{13, 14})));
}

BOOST_AUTO_TEST_CASE(circuit) {
    using Jive = Poseidon2PervushinJive;
    using E = PervushinRing;
    const std::size_t T = 4;
    const std::array<E, T> a{
        0x0000000000000000,
        0x0000000000000001,
        0x0000000000000002,
        0x0000000000000003,
    };
    const std::array<E, T> b{
        0x0000000000000010,
        0x0000000000000011,
        0x0000000000000012,
        0x0000000000000013,
    };
    std::array<E, T> c;

    using Circuit = CCSBuilder<E, 2>;
    Circuit circuit;
    std::array<typename Circuit::LinearCombination, T> x0;
    std::ranges::generate(x0, [&]{ return circuit.input(); });
    std::array<typename Circuit::LinearCombination, T> x1;
    std::ranges::generate(x1, [&]{ return circuit.input(); });
    std::array<typename Circuit::LinearCombination, T> hash;
    Jive::circuit<Circuit>::compress(circuit, x0, x1, hash);
    for (std::size_t i = 0; i < T; ++i) {
        auto v = circuit.auxiliary();
        circuit(v == hash[i]);
    }

    R1CS<E> r1cs(circuit.r1cs());
    Vector<E> z;
    z.elements.reserve(r1cs.variables());
    z.elements.emplace_back(E(1));
    std::ranges::copy(a, std::back_inserter(z.elements));
    std::ranges::copy(b, std::back_inserter(z.elements));
    Jive::trace<Circuit::degree()>::compress(a, b, c, z.elements);
    std::ranges::copy(c, std::back_inserter(z.elements));
    BOOST_TEST(r1cs.variables() == z.size());
    BOOST_TEST(r1cs.isSatisfied(z));
    for (std::size_t i = 1; i < z.size(); ++i) {
        z[i] += E(1);
        BOOST_TEST(!r1cs.isSatisfied(z));
        z[i] -= E(1);
    }

    BOOST_TEST(c == Jive::compress(a, b));
}

BOOST_AUTO_TEST_SUITE_END()

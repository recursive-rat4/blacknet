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
#include "jive.h"
#include "poseidon2pervushin.h"
#include "r1cs.h"

using namespace blacknet::crypto;

BOOST_AUTO_TEST_SUITE(Jives)

BOOST_AUTO_TEST_CASE(plain) {
    using Z = uint8_t;
    using B = std::array<Z, 4>;
    struct F {
        consteval static std::size_t width() {
            return 4;
        }
        constexpr static void permute(B& b) {
            for (std::size_t i = 0; i < b.size(); ++i) b[i] += i + 1;
        }
    };
    using J = Jive<Z, 2, F, 2>;
    using H = J::Hash;

    BOOST_TEST((H{52, 58} == J::compress(H{11, 12}, H{13, 14})));
}

BOOST_AUTO_TEST_CASE(circuit) {
    using Jive = Poseidon2PervushinJive;
    using Hash = Jive::Hash;
    using E = PervushinRing;
    const Hash a{
        0x0000000000000000,
        0x0000000000000001,
        0x0000000000000002,
        0x0000000000000003,
    };
    const Hash b{
        0x0000000000000010,
        0x0000000000000011,
        0x0000000000000012,
        0x0000000000000013,
    };
    Hash c;

    using Builder = CircuitBuilder<E, 2>;
    Builder circuit;
    using HashCircuit = Jive::HashCircuit<Builder>;
    HashCircuit x0;
    std::ranges::generate(x0, [&]{ return circuit.input(); });
    HashCircuit x1;
    std::ranges::generate(x1, [&]{ return circuit.input(); });
    HashCircuit hash = Jive::Circuit<Builder>::compress(circuit, x0, x1);
    for (std::size_t i = 0; i < hash.size(); ++i) {
        auto v = circuit.auxiliary();
        circuit(v == hash[i]);
    }

    R1CS<E> r1cs(circuit.r1cs());
    VectorDense<E> z = r1cs.assigment();
    std::ranges::copy(a, std::back_inserter(z.elements));
    std::ranges::copy(b, std::back_inserter(z.elements));
    c = Jive::Assigner<Builder::degree()>::compress(a, b, z.elements);
    std::ranges::copy(c, std::back_inserter(z.elements));
    BOOST_TEST(r1cs.isSatisfied(z));

    BOOST_TEST(c == Jive::compress(a, b));
}

BOOST_AUTO_TEST_SUITE_END()

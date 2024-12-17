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

#include "poseidon2pasta.h"
#include "poseidon2pervushin.h"
#include "poseidon2solinas62.h"
#include "r1csbuilder.h"

BOOST_AUTO_TEST_SUITE(Poseidons)

BOOST_AUTO_TEST_CASE(Pallas) {
    using E = PallasField;
    using Params = Poseidon2PallasParams;
    std::array<E, 3> a{
        0,
        1,
        2,
    };
    std::array<E, 3> b(a);
    std::array<E, 3> c{
        E("1a9b54c7512a914dd778282c44b3513fea7251420b9d95750baae059b2268d7a"),
        E("1c48ea0994a7d7984ea338a54dbf0c8681f5af883fe988d59ba3380c9f7901fc"),
        E("079ddd0a80a3e9414489b526a2770448964766685f4c4842c838f8a23120b401"),
    };
    poseidon2::permute<Params>(a);
    BOOST_TEST(c == a);

    using Circuit = R1CSBuilder<E>;
    Circuit circuit;
    std::array<typename Circuit::Variable, Params::t> x;
    std::ranges::generate(x, [&]{ return circuit.input(); });
    poseidon2::circuit::permute<Params, Circuit>(circuit, x);
    R1CS<E> r1cs(circuit.r1cs());
    Vector<E> z;
    z.elements.reserve(r1cs.variables());
    z.elements.emplace_back(E(1));
    std::ranges::copy(b, std::back_inserter(z.elements));
    poseidon2::trace::permute<Params, Circuit::degree()>(b, z.elements);
    BOOST_TEST(r1cs.variables() == z.size());
    BOOST_TEST(r1cs.isSatisfied(z));
    for (std::size_t i = 1; i < z.size(); ++i) {
        z[i] += E(1);
        BOOST_TEST(!r1cs.isSatisfied(z));
        z[i] -= E(1);
    }
}

BOOST_AUTO_TEST_CASE(Solinas62) {
    using E = Solinas62Ring;
    using Params = Poseidon2Solinas62Params;
    std::array<E, 12> a{
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
    std::array<E, 12> b(a);
    std::array<E, 12> c{
        0x367dbec705769f69,
        0x12b6981be17dd745,
        0x1452150cc1c0ac4e,
        0x3daf481da739b2c7,
        0x260239f977f3729f,
        0x3e9ec143319375c2,
        0x16e5963a9ff7fde6,
        0x08a35acef1bc9fb7,
        0x2bb9c91efc879f55,
        0x059393d79ef06150,
        0x121111905f948bd4,
        0x0bce9d1ef0e19aeb,
    };
    poseidon2::permute<Params>(a);
    BOOST_TEST(c == a);

    using Circuit = R1CSBuilder<E>;
    Circuit circuit;
    std::array<typename Circuit::Variable, Params::t> x;
    std::ranges::generate(x, [&]{ return circuit.input(); });
    poseidon2::circuit::permute<Params, Circuit>(circuit, x);
    R1CS<E> r1cs(circuit.r1cs());
    Vector<E> z;
    z.elements.reserve(r1cs.variables());
    z.elements.emplace_back(E(1));
    std::ranges::copy(b, std::back_inserter(z.elements));
    poseidon2::trace::permute<Params, Circuit::degree()>(b, z.elements);
    BOOST_TEST(r1cs.variables() == z.size());
    BOOST_TEST(r1cs.isSatisfied(z));
    for (std::size_t i = 1; i < z.size(); ++i) {
        z[i] += E(1);
        BOOST_TEST(!r1cs.isSatisfied(z));
        z[i] -= E(1);
    }
}

BOOST_AUTO_TEST_CASE(Pervushin) {
    using E = PervushinRing;
    using Params = Poseidon2PervushinParams;
    std::array<E, 12> a{
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
    std::array<E, 12> b(a);
    std::array<E, 12> c{
        0x14ad43d6b732aa1f,
        0x02fbf1c807dd0281,
        0x13e01fc66d9b3d03,
        0x11a1f9de5bad75f6,
        0x18fda95519465f5e,
        0x1a5e99d9a41fe4ce,
        0x1e16576275d7822c,
        0x1106a6eafa54ef7f,
        0x1d5353e179478d6d,
        0x09312dc75ae6f2b3,
        0x1d00514d0694390a,
        0x03f39f82fb43ef6c,
    };
    poseidon2::permute<Params>(a);
    BOOST_TEST(c == a);

    using Circuit = R1CSBuilder<E>;
    Circuit circuit;
    std::array<typename Circuit::Variable, Params::t> x;
    std::ranges::generate(x, [&]{ return circuit.input(); });
    poseidon2::circuit::permute<Params, Circuit>(circuit, x);
    R1CS<E> r1cs(circuit.r1cs());
    Vector<E> z;
    z.elements.reserve(r1cs.variables());
    z.elements.emplace_back(E(1));
    std::ranges::copy(b, std::back_inserter(z.elements));
    poseidon2::trace::permute<Params, Circuit::degree()>(b, z.elements);
    BOOST_TEST(r1cs.variables() == z.size());
    BOOST_TEST(r1cs.isSatisfied(z));
    for (std::size_t i = 1; i < z.size(); ++i) {
        z[i] += E(1);
        BOOST_TEST(!r1cs.isSatisfied(z));
        z[i] -= E(1);
    }
}

BOOST_AUTO_TEST_SUITE_END()

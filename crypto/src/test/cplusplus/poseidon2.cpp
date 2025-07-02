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
#include "poseidon2lm62.h"
#include "poseidon2pasta.h"
#include "poseidon2pervushin.h"
#include "poseidon2solinas62.h"

using namespace blacknet::crypto;

BOOST_AUTO_TEST_SUITE(Poseidons)

BOOST_AUTO_TEST_CASE(Pallas_3) {
    using E = PallasField;
    using Poseidon2 = Poseidon2<Poseidon2PallasSpongeParams>;
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
    Poseidon2::permute(a);
    BOOST_TEST(c == a);

    using Builder = CircuitBuilder<E, 2>;
    Builder circuit;
    std::array<typename Builder::LinearCombination, Poseidon2::width()> x;
    std::ranges::generate(x, [&]{ return circuit.input(); });
    Poseidon2::Circuit<Builder>::permute(circuit, x);
    for (std::size_t i = 0; i < Poseidon2::width(); ++i) {
        auto v = circuit.auxiliary();
        circuit(v == x[i]);
    }
    CustomizableConstraintSystem<E> ccs(circuit.ccs());
    Vector<E> z = ccs.assigment();
    std::ranges::copy(b, std::back_inserter(z.elements));
    Poseidon2::Assigner<Builder::degree()>::permute(b, z.elements);
    std::ranges::copy(b, std::back_inserter(z.elements));
    BOOST_TEST(ccs.isSatisfied(z));
}

BOOST_AUTO_TEST_CASE(Solinas62_12) {
    using E = Solinas62Ring;
    using Poseidon2 = Poseidon2<Poseidon2Solinas62SpongeParams>;
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
    Poseidon2::permute(a);
    BOOST_TEST(c == a);

    using Builder = CircuitBuilder<E, 3>;
    Builder circuit;
    std::array<typename Builder::LinearCombination, Poseidon2::width()> x;
    std::ranges::generate(x, [&]{ return circuit.input(); });
    Poseidon2::Circuit<Builder>::permute(circuit, x);
    for (std::size_t i = 0; i < Poseidon2::width(); ++i) {
        auto v = circuit.auxiliary();
        circuit(v == x[i]);
    }
    CustomizableConstraintSystem<E> ccs(circuit.ccs());
    Vector<E> z = ccs.assigment();
    std::ranges::copy(b, std::back_inserter(z.elements));
    Poseidon2::Assigner<Builder::degree()>::permute(b, z.elements);
    std::ranges::copy(b, std::back_inserter(z.elements));
    BOOST_TEST(ccs.isSatisfied(z));
}

BOOST_AUTO_TEST_CASE(Pervushin_12) {
    using E = PervushinRing;
    using Poseidon2 = Poseidon2<Poseidon2PervushinSpongeParams>;
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
    Poseidon2::permute(a);
    BOOST_TEST(c == a);

    using Builder = CircuitBuilder<E, 3>;
    Builder circuit;
    std::array<typename Builder::LinearCombination, Poseidon2::width()> x;
    std::ranges::generate(x, [&]{ return circuit.input(); });
    Poseidon2::Circuit<Builder>::permute(circuit, x);
    for (std::size_t i = 0; i < Poseidon2::width(); ++i) {
        auto v = circuit.auxiliary();
        circuit(v == x[i]);
    }
    CustomizableConstraintSystem<E> ccs(circuit.ccs());
    Vector<E> z = ccs.assigment();
    std::ranges::copy(b, std::back_inserter(z.elements));
    Poseidon2::Assigner<Builder::degree()>::permute(b, z.elements);
    std::ranges::copy(b, std::back_inserter(z.elements));
    BOOST_TEST(ccs.isSatisfied(z));
}

BOOST_AUTO_TEST_CASE(Pervushin_8) {
    using E = PervushinRing;
    using Poseidon2 = Poseidon2<Poseidon2PervushinJiveParams>;
    std::array<E, 8> a{
        0x0000000000000000,
        0x0000000000000001,
        0x0000000000000002,
        0x0000000000000003,
        0x0000000000000004,
        0x0000000000000005,
        0x0000000000000006,
        0x0000000000000007,
    };
    std::array<E, 8> b(a);
    std::array<E, 8> c{
        0x1a8775be9bdb5c86,
        0x084e734b4eba7e69,
        0x0bcf6bc15f7f1390,
        0x165e2e00b93ba0e8,
        0x03bc7c10d705afaa,
        0x05a6da6c5b1c7a16,
        0x0aab068f99aec08b,
        0x1d231eb4c9e7dcdd,
    };
    Poseidon2::permute(a);
    BOOST_TEST(c == a);

    using Builder = CircuitBuilder<E, 17>;
    Builder circuit;
    std::array<typename Builder::LinearCombination, Poseidon2::width()> x;
    std::ranges::generate(x, [&]{ return circuit.input(); });
    Poseidon2::Circuit<Builder>::permute(circuit, x);
    for (std::size_t i = 0; i < Poseidon2::width(); ++i) {
        auto v = circuit.auxiliary();
        circuit(v == x[i]);
    }
    CustomizableConstraintSystem<E> ccs(circuit.ccs());
    Vector<E> z = ccs.assigment();
    std::ranges::copy(b, std::back_inserter(z.elements));
    Poseidon2::Assigner<Builder::degree()>::permute(b, z.elements);
    std::ranges::copy(b, std::back_inserter(z.elements));
    BOOST_TEST(ccs.isSatisfied(z));
}

BOOST_AUTO_TEST_CASE(LM62_12) {
    using E = LM62Ring;
    using Poseidon2 = Poseidon2<Poseidon2LM62SpongeParams>;
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
        0x1e579782b480f300,
        0x1a9c54ec71b6d22c,
        0x1802ab1232ff1575,
        0x1b32d4b7bf3a14ff,
        0x0317612d81c3ebdd,
        0x25a4a86020895493,
        0x0c60db52b0367dd3,
        0x11bd8ef8519c5e43,
        0x18d65f6aa07a8421,
        0x1bf0a06fea49a2ef,
        0x01a2a3f7ae42036b,
        0x01c0693258f141c7,
    };
    Poseidon2::permute(a);
    BOOST_TEST(c == a);

    using Builder = CircuitBuilder<E, 3>;
    Builder circuit;
    std::array<typename Builder::LinearCombination, Poseidon2::width()> x;
    std::ranges::generate(x, [&]{ return circuit.input(); });
    Poseidon2::Circuit<Builder>::permute(circuit, x);
    for (std::size_t i = 0; i < Poseidon2::width(); ++i) {
        auto v = circuit.auxiliary();
        circuit(v == x[i]);
    }
    CustomizableConstraintSystem<E> ccs(circuit.ccs());
    Vector<E> z = ccs.assigment();
    std::ranges::copy(b, std::back_inserter(z.elements));
    Poseidon2::Assigner<Builder::degree()>::permute(b, z.elements);
    std::ranges::copy(b, std::back_inserter(z.elements));
    BOOST_TEST(ccs.isSatisfied(z));
}

BOOST_AUTO_TEST_CASE(LM62_8) {
    using E = LM62Ring;
    using Poseidon2 = Poseidon2<Poseidon2LM62JiveParams>;
    std::array<E, 8> a{
        0x0000000000000000,
        0x0000000000000001,
        0x0000000000000002,
        0x0000000000000003,
        0x0000000000000004,
        0x0000000000000005,
        0x0000000000000006,
        0x0000000000000007,
    };
    std::array<E, 8> b(a);
    std::array<E, 8> c{
        0x14cb404ab36b8a85,
        0x0a7850e39ca55475,
        0x1e3c06dd7b33c7be,
        0x1fdecbc1077bce9c,
        0x096043ac97bdb167,
        0x0f8700d5c0f443d0,
        0x1c175762aaea4839,
        0x2269d4cce9947ff6,
    };
    Poseidon2::permute(a);
    BOOST_TEST(c == a);

    using Builder = CircuitBuilder<E, 3>;
    Builder circuit;
    std::array<typename Builder::LinearCombination, Poseidon2::width()> x;
    std::ranges::generate(x, [&]{ return circuit.input(); });
    Poseidon2::Circuit<Builder>::permute(circuit, x);
    for (std::size_t i = 0; i < Poseidon2::width(); ++i) {
        auto v = circuit.auxiliary();
        circuit(v == x[i]);
    }
    CustomizableConstraintSystem<E> ccs(circuit.ccs());
    Vector<E> z = ccs.assigment();
    std::ranges::copy(b, std::back_inserter(z.elements));
    Poseidon2::Assigner<Builder::degree()>::permute(b, z.elements);
    std::ranges::copy(b, std::back_inserter(z.elements));
    BOOST_TEST(ccs.isSatisfied(z));
}

BOOST_AUTO_TEST_SUITE_END()

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
#include "fs.h"
#include "pervushin.h"
#include "pervushinfield.h"
#include "poseidon2pervushin.h"
#include "r1cs.h"

using namespace blacknet::crypto;

static FastDRG rng;

BOOST_AUTO_TEST_SUITE(FSs)

using F = PervushinRing;

BOOST_AUTO_TEST_CASE(test) {
    using Circuit = CircuitBuilder<F, 2>;
    using CS = R1CS<F>;
    using RO = Poseidon2PervushinSponge<{67,68,69,70}>;
    using FS = FS<CS, RO>;

    Circuit circuit;
    {
        auto a = circuit.input();
        auto b = circuit.auxiliary();
        auto c = circuit.auxiliary();
        auto d = circuit.auxiliary();
        circuit(a + b == c * d);
    }
    R1CS<F> r1cs = circuit.r1cs();
    FS fs(r1cs);

    Vector<F> z, e;
    const Vector<F> e_init(r1cs.constraints(), F(0));
    Vector<F> z1{ F(1), F(6), F(-2), F(2), F(2) };
    Vector<F> z2{ F(1), F(20), F(-4), F(4), F(4) };
    Vector<F> z3{ F(1), F(12), F(-4), F(4), F(4) };
    BOOST_TEST(r1cs.isSatisfied(z1));
    BOOST_TEST(r1cs.isSatisfied(z2));
    BOOST_TEST(!r1cs.isSatisfied(z3));

    fs.fold(z, e, z1, e_init, z2, e_init);
    BOOST_TEST(r1cs.isSatisfied(z, e));

    fs.randomize(rng, z, e, z, e);
    BOOST_TEST(r1cs.isSatisfied(z, e));

    fs.fold(z, e, z, e, z3, e_init);
    BOOST_TEST(!r1cs.isSatisfied(z, e));
}

BOOST_AUTO_TEST_SUITE_END()

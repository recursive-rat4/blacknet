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

#include "circuitry.h"
#include "fastrng.h"
#include "matrix.h"
#include "pervushin.h"
#include "pervushinfield.h"
#include "r1cs.h"

using namespace blacknet::crypto;

static FastDRG rng;

BOOST_AUTO_TEST_SUITE(R1CSs)

using E = PervushinRing;
using EE = PervushinRingDegree2;

BOOST_AUTO_TEST_CASE(Satisfaction) {
    // Sixte with riposte
    Matrix<E> a(3, 5, {
        E(0), E(0), E(1), E(0), E(0),
        E(0), E(0), E(0), E(1), E(0),
        E(0), E(0), E(0), E(0), E(1),
    });
    Matrix<E> b(3, 5, {
        E(0), E(0), E(0), E(1), E(0),
        E(0), E(0), E(0), E(1), E(0),
        E(0), E(0), E(0), E(0), E(1),
    });
    Matrix<E> c(3, 5, {
        E(4), E(1), E(0), E(0), E(0),
        E(0), E(0), E(1), E(0), E(0),
        E(0), E(0), E(0), E(1), E(0),
    });
    Vector<E> z{ E(1), E(60), E(16), E(4), E(2) };

    R1CS<E> r1cs{
        MatrixSparse<E>(a),
        MatrixSparse<E>(b),
        MatrixSparse<E>(c),
    };
    test::circuitry(r1cs, z);

    Vector<EE> z_morphed(z.homomorph<EE>());
    test::circuitry(r1cs, z_morphed);

    const Vector<EE> e_init(r1cs.constraints(), EE(0));
    Vector<EE> e_folded(e_init);
    Vector<EE> z_folded(z_morphed);
    BOOST_TEST(r1cs.isSatisfied(z_folded, e_folded));

    EE r1{E(11), E(31)};
    r1cs.fold(r1, z_folded, e_folded, z_folded, e_folded, z_morphed, e_init);
    BOOST_TEST(r1cs.isSatisfied(z_folded, e_folded));

    Vector<EE> z_other{ EE(1), EE(725), EE(81), EE(9), EE(3) };
    BOOST_TEST(r1cs.isSatisfied(z_other));
    r1cs.fold(r1, z_folded, e_folded, z_folded, e_folded, z_other, e_init);
    BOOST_TEST(r1cs.isSatisfied(z_folded, e_folded));
    BOOST_TEST(e_init != e_folded);

    EE r2{E(-13), E(-3)};
    r1cs.fold(r2, z_folded, e_folded, z_folded, e_folded, z_other, e_init);
    BOOST_TEST(r1cs.isSatisfied(z_folded, e_folded));

    r1cs.fold(r2, z_folded, e_folded, z_folded, e_folded, z_folded, e_folded);
    BOOST_TEST(r1cs.isSatisfied(z_folded, e_folded));

    auto rr = EE::random(rng);
    auto [zr, er] = r1cs.random<EE>(rng);
    BOOST_TEST(r1cs.isSatisfied(zr, er));
    r1cs.fold(rr, z_folded, e_folded, z_folded, e_folded, zr, er);
    BOOST_TEST(r1cs.isSatisfied(z_folded, e_folded));
}

BOOST_AUTO_TEST_SUITE_END()

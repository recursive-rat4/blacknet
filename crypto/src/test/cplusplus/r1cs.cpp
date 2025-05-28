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

#include "fastrng.h"
#include "matrix.h"
#include "pervushin.h"
#include "pervushinfield.h"
#include "r1cs.h"

using namespace blacknet::crypto;

static FastDRG rng;

BOOST_AUTO_TEST_SUITE(R1CSs)

using Z = PervushinRing;
using R = PervushinRingDegree2;

BOOST_AUTO_TEST_CASE(Satisfaction) {
    // Sixte with riposte
    Matrix<R> a(3, 5, {
        R(0), R(0), R(1), R(0), R(0),
        R(0), R(0), R(0), R(1), R(0),
        R(0), R(0), R(0), R(0), R(1),
    });
    Matrix<R> b(3, 5, {
        R(0), R(0), R(0), R(1), R(0),
        R(0), R(0), R(0), R(1), R(0),
        R(0), R(0), R(0), R(0), R(1),
    });
    Matrix<R> c(3, 5, {
        R(4), R(1), R(0), R(0), R(0),
        R(0), R(0), R(1), R(0), R(0),
        R(0), R(0), R(0), R(1), R(0),
    });
    Vector<R> z{ R(1), R(60), R(16), R(4), R(2) };

    R1CS<R> r1cs{
        MatrixSparse<R>(a),
        MatrixSparse<R>(b),
        MatrixSparse<R>(c),
    };
    BOOST_TEST(r1cs.isSatisfied(z));

    const Vector<R> e_init(r1cs.constraints(), R(0));
    Vector<R> e_folded(e_init);
    Vector<R> z_folded(z);
    BOOST_TEST(r1cs.isSatisfied(z_folded, e_folded));

    R r1{Z(11), Z(31)};
    r1cs.fold(r1, z_folded, e_folded, z_folded, e_folded, z, e_init);
    BOOST_TEST(r1cs.isSatisfied(z_folded, e_folded));

    Vector<R> z_other{ R(1), R(725), R(81), R(9), R(3) };
    BOOST_TEST(r1cs.isSatisfied(z_other));
    r1cs.fold(r1, z_folded, e_folded, z_folded, e_folded, z_other, e_init);
    BOOST_TEST(r1cs.isSatisfied(z_folded, e_folded));
    BOOST_TEST(e_init != e_folded);

    R r2{Z(-13), Z(-3)};
    r1cs.fold(r2, z_folded, e_folded, z_folded, e_folded, z_other, e_init);
    BOOST_TEST(r1cs.isSatisfied(z_folded, e_folded));

    r1cs.fold(r2, z_folded, e_folded, z_folded, e_folded, z_folded, e_folded);
    BOOST_TEST(r1cs.isSatisfied(z_folded, e_folded));

    auto rr = R::random(rng);
    auto [zr, er] = r1cs.random(rng);
    BOOST_TEST(r1cs.isSatisfied(zr, er));
    r1cs.fold(rr, z_folded, e_folded, z_folded, e_folded, zr, er);
    BOOST_TEST(r1cs.isSatisfied(z_folded, e_folded));
}

BOOST_AUTO_TEST_SUITE_END()

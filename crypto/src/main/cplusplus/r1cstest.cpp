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

#include "pervushin.h"
#include "r1cs.h"

BOOST_AUTO_TEST_SUITE(R1CSs)

using E = PervushinRing;
using EE = PervushinRingDegree2;

BOOST_AUTO_TEST_CASE(Satisfaction) {
    // Sixte
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
        E(0), E(1), E(0), E(0), E(0),
        E(0), E(0), E(1), E(0), E(0),
        E(0), E(0), E(0), E(1), E(0),
    });
    Vector<E> z{ E(1), E(64), E(16), E(4), E(2) };

    R1CS<E> r1cs{
        MatrixSparse<E>(a),
        MatrixSparse<E>(b),
        MatrixSparse<E>(c),
    };
    BOOST_TEST(r1cs.isSatisfied(z));
    for (std::size_t i = 1; i < z.size(); ++i) {
        z[i] += E(1);
        BOOST_TEST(!r1cs.isSatisfied(z));
        z[i] -= E(1);
    }

    Vector<EE> z_morphed(z.homomorph<EE>());
    BOOST_TEST(r1cs.isSatisfied(z_morphed));
    for (std::size_t i = 1; i < z_morphed.size(); ++i) {
        z_morphed[i] += EE(1);
        BOOST_TEST(!r1cs.isSatisfied(z_morphed));
        z_morphed[i] -= EE(1);
    }
}

BOOST_AUTO_TEST_CASE(Building) {
    Matrix<E> m1(2, 3, {
        E(10), E(11), E(12),
        E(13), E(14), E(15),
    });
    Vector<E> v1{ E(00), E(00) };
    Matrix<E> m2(3, 2, {
        E(16), E(17),
        E(18), E(19),
        E(20), E(21),
    });
    Vector<E> v2{ E(00), E(04), E(00) };
    Matrix<E> m3(5, 6, {
        E(00), E(10), E(11), E(12), E(00), E(00),
        E(00), E(13), E(14), E(15), E(00), E(00),
        E(00), E(00), E(00), E(00), E(16), E(17),
        E(04), E(00), E(00), E(00), E(18), E(19),
        E(00), E(00), E(00), E(00), E(20), E(21),
    });
    MatrixSparse<E> ms(m3);

    R1CS<E>::Builder builder;
    builder.append(m1, v1, m1, v1, m1, v1);
    builder.append(m2, v2, m2, v2, m2, v2);
    BOOST_TEST(R1CS(ms, ms, ms) == builder.build());
}

BOOST_AUTO_TEST_SUITE_END()

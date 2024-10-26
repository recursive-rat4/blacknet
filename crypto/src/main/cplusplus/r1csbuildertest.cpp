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
#include "r1csbuilder.h"

BOOST_AUTO_TEST_SUITE(R1CSBuilders)

using E = PervushinRing;

BOOST_AUTO_TEST_CASE(Eqs) {
    Matrix<E> am(4, 4, {
        E(0), E(1), E(0), E(-1),
        E(0), E(0), E(-1), E(1),
        E(4), E(0), E(0), E(-1),
        E(4), E(0), E(0), E(-1),
    });
    Matrix<E> bm(4, 4, {
        E(1), E(0), E(0), E(0),
        E(1), E(0), E(0), E(0),
        E(1), E(0), E(0), E(0),
        E(1), E(0), E(0), E(0),
    });
    Matrix<E> cm(4, 4, {
        E(0), E(0), E(0), E(0),
        E(0), E(0), E(0), E(0),
        E(0), E(0), E(0), E(0),
        E(0), E(0), E(0), E(0),
    });
    R1CS<E> r1cs{
        MatrixSparse<E>(am),
        MatrixSparse<E>(bm),
        MatrixSparse<E>(cm),
    };

    R1CSBuilder<E> circuit;
    auto c = E(4);
    auto x = circuit.input();
    auto y = circuit.input();
    auto w = circuit.auxiliary();

    circuit(x == w);
    circuit(w == y);
    circuit(w == c);
    circuit(c == w);

    BOOST_TEST(r1cs == circuit.r1cs());

    Vector<E> z{ E(1), E(4), E(4), E(4) };
    BOOST_TEST(r1cs.isSatisfied(z));
    for (std::size_t i = 1; i < z.size(); ++i) {
        z[i] += E(1);
        BOOST_TEST(!r1cs.isSatisfied(z));
        z[i] -= E(1);
    }
}

BOOST_AUTO_TEST_CASE(Adds) {
    Matrix<E> am(5, 4, {
        E(0), E(0), E(0), E(2),
        E(0), E(0), E(2), E(0),
        E(0), E(0), E(2), E(0),
        E(4), E(0), E(0), E(1),
        E(4), E(0), E(0), E(1),
    });
    Matrix<E> bm(5, 4, {
        E(1), E(0), E(0), E(0),
        E(1), E(0), E(0), E(0),
        E(1), E(0), E(0), E(0),
        E(1), E(0), E(0), E(0),
        E(1), E(0), E(0), E(0),
    });
    Matrix<E> cm(5, 4, {
        E(0), E(1), E(0), E(0),
        E(0), E(0), E(0), E(1),
        E(4), E(0), E(0), E(0),
        E(0), E(1), E(0), E(0),
        E(0), E(1), E(0), E(0),
    });
    R1CS<E> r1cs{
        MatrixSparse<E>(am),
        MatrixSparse<E>(bm),
        MatrixSparse<E>(cm),
    };

    R1CSBuilder<E> circuit;
    auto c = E(4);
    auto x = circuit.input();
    auto y = circuit.input();
    auto w = circuit.auxiliary();

    circuit(x == w + w);
    circuit(w == y + y);
    circuit(c == y + y);
    circuit(x == w + c);
    circuit(x == c + w);

    BOOST_TEST(r1cs == circuit.r1cs());

    Vector<E> z{ E(1), E(8), E(2), E(4) };
    BOOST_TEST(r1cs.isSatisfied(z));
    for (std::size_t i = 1; i < z.size(); ++i) {
        z[i] += E(1);
        BOOST_TEST(!r1cs.isSatisfied(z));
        z[i] -= E(1);
    }
}

BOOST_AUTO_TEST_CASE(Muls) {
    Matrix<E> am(5, 4, {
        E(0), E(0), E(0), E(1),
        E(0), E(0), E(1), E(0),
        E(0), E(0), E(1), E(0),
        E(0), E(0), E(0), E(4),
        E(0), E(0), E(0), E(4),
    });
    Matrix<E> bm(5, 4, {
        E(0), E(0), E(0), E(1),
        E(0), E(0), E(1), E(0),
        E(0), E(0), E(1), E(0),
        E(1), E(0), E(0), E(0),
        E(1), E(0), E(0), E(0),
    });
    Matrix<E> cm(5, 4, {
        E(0), E(1), E(0), E(0),
        E(0), E(0), E(0), E(1),
        E(4), E(0), E(0), E(0),
        E(0), E(1), E(0), E(0),
        E(0), E(1), E(0), E(0),
    });
    R1CS<E> r1cs{
        MatrixSparse<E>(am),
        MatrixSparse<E>(bm),
        MatrixSparse<E>(cm),
    };

    R1CSBuilder<E> circuit;
    auto c = E(4);
    auto x = circuit.input();
    auto y = circuit.input();
    auto w = circuit.auxiliary();

    circuit(x == w * w);
    circuit(w == y * y);
    circuit(c == y * y);
    circuit(x == w * c);
    circuit(x == c * w);

    BOOST_TEST(r1cs == circuit.r1cs());

    Vector<E> z{ E(1), E(16), E(2), E(4) };
    BOOST_TEST(r1cs.isSatisfied(z));
    for (std::size_t i = 1; i < z.size(); ++i) {
        z[i] += E(1);
        BOOST_TEST(!r1cs.isSatisfied(z));
        z[i] -= E(1);
    }
}

BOOST_AUTO_TEST_CASE(Boards) {
    Matrix<E> am(8, 5, {
        E(0), E(1), E(1), E(0), E(0),
        E(0), E(0), E(0), E(1), E(0),
        E(4), E(1), E(1), E(1), E(0),
        E(8), E(0), E(1), E(1), E(0),
        E(0), E(0), E(2), E(2), E(0),
        E(0), E(6), E(0), E(0), E(0),
        E(8), E(2), E(0), E(0), E(0),
        E(8), E(2), E(0), E(0), E(0),
    });
    Matrix<E> bm(8, 5, {
        E(0), E(0), E(0), E(1), E(1),
        E(0), E(0), E(0), E(1), E(0),
        E(1), E(0), E(0), E(0), E(0),
        E(1), E(0), E(0), E(0), E(0),
        E(1), E(0), E(0), E(0), E(0),
        E(1), E(0), E(0), E(0), E(0),
        E(1), E(0), E(0), E(0), E(0),
        E(1), E(0), E(0), E(0), E(0),
    });
    Matrix<E> cm(8, 5, {
        E(160), E(0), E(0), E(0), E(0),
        E(0), E(4), E(0), E(0), E(0),
        E(0), E(0), E(0), E(0), E(1),
        E(0), E(0), E(0), E(0), E(1),
        E(0), E(0), E(0), E(0), E(1),
        E(24), E(0), E(0), E(0), E(0),
        E(0), E(0), E(0), E(0), E(1),
        E(0), E(0), E(0), E(0), E(1),
    });
    R1CS<E> r1cs{
        MatrixSparse<E>(am),
        MatrixSparse<E>(bm),
        MatrixSparse<E>(cm),
    };

    R1CSBuilder<E> circuit;
    auto a = E(160);
    auto b = E(2);
    auto c = E(4);
    auto d = E(24);
    auto x = circuit.input();
    auto y = circuit.input();
    auto z = circuit.input();
    auto w = circuit.auxiliary();

    circuit(a == (x + y) * (z + w));
    circuit(x * c == z * z);
    circuit(w == x + y + z + c);
    circuit(w == c + y + z + c);
    circuit(w == b * y + z * b);
    circuit(d == b * x + x * c);
    circuit(w == c + b * (x + b));
    circuit(w == b * (x + c));

    BOOST_TEST(r1cs == circuit.r1cs());

    Vector<E> zv{ E(1), E(4), E(4), E(4), E(16) };
    BOOST_TEST(r1cs.isSatisfied(zv));
    for (std::size_t i = 1; i < zv.size(); ++i) {
        zv[i] += E(1);
        BOOST_TEST(!r1cs.isSatisfied(zv));
        zv[i] -= E(1);
    }
}

BOOST_AUTO_TEST_SUITE_END()

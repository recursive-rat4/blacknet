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
#include "pervushin.h"
#include "r1cs.h"

using namespace blacknet::crypto;

BOOST_AUTO_TEST_SUITE(CircuitBuilders)

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

    CircuitBuilder<E, 2> circuit;
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

    CircuitBuilder<E, 2> circuit;
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

    CircuitBuilder<E, 2> circuit;
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

    CircuitBuilder<E, 2> circuit;
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
}

BOOST_AUTO_TEST_CASE(Cubism) {
    Matrix<E> am(2, 5, {
        E(0), E(1), E(0), E(0), E(0),
        E(0), E(1), E(1), E(0), E(0),
    });
    Matrix<E> bm(2, 5, {
        E(0), E(1), E(0), E(0), E(0),
        E(0), E(1), E(0), E(1), E(0),
    });
    Matrix<E> cm(2, 5, {
        E(0), E(1), E(0), E(0), E(0),
        E(0), E(1), E(0), E(0), E(1),
    });
    Matrix<E> dm(2, 5, {
        E(0), E(0), E(0), E(0), E(1),
        E(350), E(0), E(0), E(0), E(0),
    });
    CustomizableConstraintSystem<E> ccs{
        2, 5,
        {am, bm, cm, dm},
        {{0, 1, 2}, {3}},
        {E(1), E(-1)}
    };

    CircuitBuilder<E, 3> circuit;
    auto c = E(350);
    auto x = circuit.input();
    auto y = circuit.input();
    auto z = circuit.input();
    auto w = circuit.auxiliary();

    circuit(w == x * x * x);
    circuit(c == (x + y) * (x + z) * (x + w));

    BOOST_TEST(ccs == circuit.ccs());

    Vector<E> zv{ E(1), E(2), E(3), E(5), E(8) };
    BOOST_TEST(ccs.isSatisfied(zv));
}

BOOST_AUTO_TEST_SUITE_END()

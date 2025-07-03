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
#include "matrixdense.h"
#include "pervushin.h"
#include "r1cs.h"
#include "vectordense.h"

using namespace blacknet::crypto;

using R = PervushinRing;

BOOST_AUTO_TEST_SUITE(Vector_Plain)

BOOST_AUTO_TEST_CASE(HadamardSummation) {
    VectorDense<R> a{
        R(0),
        R(4),
        R(2),
    };
    VectorDense<R> b{
        R(7),
        R(3),
        R(5),
    };
    VectorDense<R> c{
        R(7),
        R(7),
        R(7),
    };
    BOOST_TEST(c == a + b);
    BOOST_TEST(c == b + a);
}

BOOST_AUTO_TEST_CASE(HadamardProduct) {
    VectorDense<R> a{
        R(2),
        R(2),
        R(2),
    };
    VectorDense<R> b{
        R(1),
        R(2),
        R(4),
    };
    VectorDense<R> c{
        R(2),
        R(4),
        R(8),
    };
    BOOST_TEST(c == a * b);
    BOOST_TEST(c == b * a);
}

BOOST_AUTO_TEST_CASE(ScalarProduct) {
    VectorDense<R> a{
        R(4),
        R(5),
        R(6),
    };
    R b(2);
    VectorDense<R> c{
        R(8),
        R(10),
        R(12),
    };
    BOOST_TEST(c == a * b);
    BOOST_TEST(c == b * a);
}

BOOST_AUTO_TEST_CASE(HadamardSubtraction) {
    VectorDense<R> a{
        R(8),
        R(5),
        R(1),
    };
    VectorDense<R> b{
        R(7),
        R(3),
        R(0),
    };
    VectorDense<R> c{
        R(1),
        R(2),
        R(1),
    };
    BOOST_TEST(c == a - b);
}

BOOST_AUTO_TEST_CASE(Negation) {
    VectorDense<R> a{
        R(7),
        R(0),
        R(-1),
    };
    VectorDense<R> b{
        R(-7),
        R(0),
        R(1),
    };
    BOOST_TEST(b == -a);
    BOOST_TEST(a == -(-a));
}

BOOST_AUTO_TEST_CASE(Concatectation) {
    VectorDense<R> a{
        R(0), R(1),
    };
    VectorDense<R> b{
        R(2), R(3), R(4),
    };
    VectorDense<R> c{
        R(0), R(1), R(2), R(3), R(4),
    };
    VectorDense<R> d{
        R(2), R(3), R(4), R(0), R(1),
    };
    BOOST_TEST(c == (a || b));
    BOOST_TEST(d == (b || a));
}

BOOST_AUTO_TEST_CASE(DotProduct) {
    VectorDense<R> a{
        R(1),
        R(3),
        R(-5),
    };
    VectorDense<R> b{
        R(4),
        R(-2),
        R(-1),
    };
    R c(3);
    R d(35);
    BOOST_TEST(c == a.dot(b));
    BOOST_TEST(c == b.dot(a));
    BOOST_TEST(d == a.dot(a));
}

BOOST_AUTO_TEST_CASE(TensorProduct) {
    VectorDense<R> a{
        R(0),
        R(1),
        R(2),
    };
    VectorDense<R> b{
        R(3),
        R(4),
    };
    MatrixDense<R> c{3, 2, {
        R(0), R(0),
        R(3), R(4),
        R(6), R(8),
    }};
    MatrixDense<R> d{2, 3, {
        R(0), R(3), R(6),
        R(0), R(4), R(8),
    }};
    BOOST_TEST(c == a.tensor(b));
    BOOST_TEST(d == b.tensor(a));
}

BOOST_AUTO_TEST_SUITE_END()

BOOST_AUTO_TEST_SUITE(Vector_Circuit)

BOOST_AUTO_TEST_CASE(DotProduct) {
    VectorDense<R> a{
        R(1),
        R(3),
        R(-5),
    };
    VectorDense<R> b{
        R(4),
        R(-2),
        R(-1),
    };
    R c(3);

    using Builder = CircuitBuilder<R, 2>;
    Builder circuit;
    using VectorDenseCircuit = VectorDense<R>::Circuit<Builder>;
    VectorDenseCircuit a_circuit(circuit, Builder::Variable::Type::Input, 3);
    VectorDenseCircuit b_circuit(circuit, Builder::Variable::Type::Input, 3);
    auto c_var = circuit.input();
    circuit(c_var == a_circuit.dot(b_circuit));

    R1CS<R> r1cs(circuit.r1cs());
    VectorDense<R> z = r1cs.assigment();
    std::ranges::copy(a.elements, std::back_inserter(z.elements));
    std::ranges::copy(b.elements, std::back_inserter(z.elements));
    z.elements.push_back(c);

    using Assigner = VectorDense<R>::Assigner<Builder::degree()>;
    Assigner a_assigner(a, z.elements);
    Assigner b_assigner(b, z.elements);
    BOOST_TEST(c == a_assigner.dot(b_assigner));
    BOOST_TEST(r1cs.isSatisfied(z));
}

BOOST_AUTO_TEST_SUITE_END()

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
#include "matrix.h"
#include "pervushin.h"
#include "r1cs.h"
#include "vector.h"

using namespace blacknet::crypto;

using R = PervushinRing;

BOOST_AUTO_TEST_SUITE(Vector_Plain)

BOOST_AUTO_TEST_CASE(HadamardSummation) {
    Vector<R> a{
        R(0),
        R(4),
        R(2),
    };
    Vector<R> b{
        R(7),
        R(3),
        R(5),
    };
    Vector<R> c{
        R(7),
        R(7),
        R(7),
    };
    BOOST_TEST(c == a + b);
    BOOST_TEST(c == b + a);
}

BOOST_AUTO_TEST_CASE(HadamardProduct) {
    Vector<R> a{
        R(2),
        R(2),
        R(2),
    };
    Vector<R> b{
        R(1),
        R(2),
        R(4),
    };
    Vector<R> c{
        R(2),
        R(4),
        R(8),
    };
    BOOST_TEST(c == a * b);
    BOOST_TEST(c == b * a);
}

BOOST_AUTO_TEST_CASE(ScalarProduct) {
    Vector<R> a{
        R(4),
        R(5),
        R(6),
    };
    R b(2);
    Vector<R> c{
        R(8),
        R(10),
        R(12),
    };
    BOOST_TEST(c == a * b);
    BOOST_TEST(c == b * a);
}

BOOST_AUTO_TEST_CASE(HadamardSubtraction) {
    Vector<R> a{
        R(8),
        R(5),
        R(1),
    };
    Vector<R> b{
        R(7),
        R(3),
        R(0),
    };
    Vector<R> c{
        R(1),
        R(2),
        R(1),
    };
    BOOST_TEST(c == a - b);
}

BOOST_AUTO_TEST_CASE(Negation) {
    Vector<R> a{
        R(7),
        R(0),
        R(-1),
    };
    Vector<R> b{
        R(-7),
        R(0),
        R(1),
    };
    BOOST_TEST(b == -a);
    BOOST_TEST(a == -(-a));
}

BOOST_AUTO_TEST_CASE(Concatectation) {
    Vector<R> a{
        R(0), R(1),
    };
    Vector<R> b{
        R(2), R(3), R(4),
    };
    Vector<R> c{
        R(0), R(1), R(2), R(3), R(4),
    };
    Vector<R> d{
        R(2), R(3), R(4), R(0), R(1),
    };
    BOOST_TEST(c == (a || b));
    BOOST_TEST(d == (b || a));
}

BOOST_AUTO_TEST_CASE(DotProduct) {
    Vector<R> a{
        R(1),
        R(3),
        R(-5),
    };
    Vector<R> b{
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
    Vector<R> a{
        R(0),
        R(1),
        R(2),
    };
    Vector<R> b{
        R(3),
        R(4),
    };
    Matrix<R> c{3, 2, {
        R(0), R(0),
        R(3), R(4),
        R(6), R(8),
    }};
    Matrix<R> d{2, 3, {
        R(0), R(3), R(6),
        R(0), R(4), R(8),
    }};
    BOOST_TEST(c == a.tensor(b));
    BOOST_TEST(d == b.tensor(a));
}

BOOST_AUTO_TEST_SUITE_END()

BOOST_AUTO_TEST_SUITE(Vector_Circuit)

BOOST_AUTO_TEST_CASE(DotProduct) {
    Vector<R> a{
        R(1),
        R(3),
        R(-5),
    };
    Vector<R> b{
        R(4),
        R(-2),
        R(-1),
    };
    R c(3);

    using Builder = CircuitBuilder<R, 2>;
    Builder circuit;
    using VectorCircuit = Vector<R>::Circuit<Builder>;
    VectorCircuit a_circuit(circuit, Builder::Variable::Type::Input, 3);
    VectorCircuit b_circuit(circuit, Builder::Variable::Type::Input, 3);
    auto c_var = circuit.input();
    circuit(c_var == a_circuit.dot(b_circuit));

    R1CS<R> r1cs(circuit.r1cs());
    Vector<R> z = r1cs.assigment();
    std::ranges::copy(a.elements, std::back_inserter(z.elements));
    std::ranges::copy(b.elements, std::back_inserter(z.elements));
    z.elements.push_back(c);

    using Tracer = Vector<R>::Tracer;
    Tracer a_tracer(a, z.elements);
    Tracer b_tracer(b, z.elements);
    BOOST_TEST(c == a_tracer.dot(b_tracer));
    BOOST_TEST(r1cs.isSatisfied(z));
}

BOOST_AUTO_TEST_SUITE_END()

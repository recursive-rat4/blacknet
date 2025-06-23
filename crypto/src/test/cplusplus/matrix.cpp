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

BOOST_AUTO_TEST_SUITE(Matrix_Plain)

BOOST_AUTO_TEST_CASE(Add) {
    Matrix<R> a(3, 2, {
        R(1), R(3),
        R(1), R(0),
        R(1), R(2),
    });
    Matrix<R> b{3, 2, {
        R(0), R(0),
        R(7), R(5),
        R(2), R(1),
    }};
    Matrix<R> c{3, 2, {
        R(1), R(3),
        R(8), R(5),
        R(3), R(3),
    }};
    BOOST_TEST(c == a + b);
    BOOST_TEST(c == b + a);
}

BOOST_AUTO_TEST_CASE(Mul) {
    Matrix<R> a(4, 3, {
        R(1), R(0), R(1),
        R(2), R(1), R(1),
        R(0), R(1), R(1),
        R(1), R(1), R(2),
    });
    Matrix<R> b{3, 3, {
        R(1), R(2), R(1),
        R(2), R(3), R(1),
        R(4), R(2), R(2),
    }};
    Matrix<R> c{4, 3, {
        R(5), R(4), R(3),
        R(8), R(9), R(5),
        R(6), R(5), R(3),
        R(11), R(9), R(6),
    }};
    BOOST_TEST(c == a * b);
}

BOOST_AUTO_TEST_CASE(VectorProduct) {
    Matrix<R> a(3, 2, {
        R(17), R(18),
        R(33), R(34),
        R(49), R(50),
    });
    Vector<R> b{
        R(2),
        R(3),
    };
    Vector<R> c{
        R(88),
        R(168),
        R(248),
    };
    Vector<R> d{
        R(19192),
        R(19696),
    };
    BOOST_TEST(c == a * b);
    BOOST_TEST(d == c * a);
}

BOOST_AUTO_TEST_CASE(Concatectation) {
    Matrix<R> a(3, 2, {
        R(1), R(3),
        R(1), R(0),
        R(1), R(2),
    });
    Matrix<R> b{3, 2, {
        R(0), R(0),
        R(7), R(5),
        R(2), R(1),
    }};
    Matrix<R> c{3, 4, {
        R(1), R(3), R(0), R(0),
        R(1), R(0), R(7), R(5),
        R(1), R(2), R(2), R(1),
    }};
    BOOST_TEST(c == (a || b));
}

BOOST_AUTO_TEST_CASE(Transposition) {
    Matrix<R> a(3, 2, {
        R(1), R(2),
        R(3), R(4),
        R(5), R(6),
    });
    Matrix<R> b{2, 3, {
        R(1), R(3), R(5),
        R(2), R(4), R(6),
    }};
    BOOST_TEST(b == a.transpose());
    BOOST_TEST(a == a.transpose().transpose());
}

BOOST_AUTO_TEST_CASE(InfinityNorm) {
    Matrix<R> a(2, 2, {
        R(0), R(1),
        R(2), R(3),
    });
    int64_t nb = 3;
    int64_t ng = 4;
    BOOST_TEST(!a.checkInfinityNorm(nb));
    BOOST_TEST(a.checkInfinityNorm(ng));
}

BOOST_AUTO_TEST_SUITE_END()

BOOST_AUTO_TEST_SUITE(Matrix_Circuit)

BOOST_AUTO_TEST_CASE(VectorProduct) {
    const Matrix<R> a(3, 2, {
        R(17), R(18),
        R(33), R(34),
        R(49), R(50),
    });
    const Vector<R> b{
        R(2),
        R(3),
    };
    const Vector<R> c{
        R(88),
        R(168),
        R(248),
    };

    using Builder = CircuitBuilder<R, 2>;
    Builder circuit;
    using MatrixCircuit = Matrix<R>::Circuit<Builder>;
    MatrixCircuit a_circuit(circuit, Builder::Variable::Type::Input, a.rows, a.columns);
    using VectorCircuit = Vector<R>::Circuit<Builder>;
    VectorCircuit b_circuit(circuit, Builder::Variable::Type::Input, b.size());
    auto c_circuit = a_circuit * b_circuit;

    R1CS<R> r1cs(circuit.r1cs());
    Vector<R> z = r1cs.assigment();
    std::ranges::copy(a.elements, std::back_inserter(z.elements));
    std::ranges::copy(b.elements, std::back_inserter(z.elements));

    using MatrixTracer = Matrix<R>::Tracer;
    MatrixTracer a_tracer(a, z.elements);
    using VectorTracer = Vector<R>::Tracer;
    VectorTracer b_tracer(b, z.elements);
    auto c_traced = a_tracer * b_tracer;
    BOOST_TEST(c == c_traced.vector);
    BOOST_TEST(r1cs.isSatisfied(z));
}

BOOST_AUTO_TEST_SUITE_END()

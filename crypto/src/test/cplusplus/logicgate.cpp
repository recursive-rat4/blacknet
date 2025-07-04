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
#include <algorithm>
#include <array>
#include <tuple>

#include "circuitbuilder.h"
#include "logicgate.h"
#include "pervushinextension.h"
#include "r1cs.h"
#include "vectordense.h"

using namespace blacknet::crypto;

BOOST_AUTO_TEST_SUITE(LogicGate_Plain)

using R = PervushinRingDegree2;
using LogicGate = LogicGate<R>;

BOOST_AUTO_TEST_CASE(xors) {
    const std::array<std::tuple<R,R,R>, 4> truth{
        std::make_tuple(R(0), R(0), R(0)),
        std::make_tuple(R(0), R(1), R(1)),
        std::make_tuple(R(1), R(0), R(1)),
        std::make_tuple(R(1), R(1), R(0))
    };
    for (auto&& row : truth) {
        auto&& [a, b, c] = row;
        BOOST_TEST(c == LogicGate().Xor(a, b));
    }
}

BOOST_AUTO_TEST_CASE(ands) {
    const std::array<std::tuple<R,R,R>, 4> truth{
        std::make_tuple(R(0), R(0), R(0)),
        std::make_tuple(R(0), R(1), R(0)),
        std::make_tuple(R(1), R(0), R(0)),
        std::make_tuple(R(1), R(1), R(1))
    };
    for (auto&& row : truth) {
        auto&& [a, b, c] = row;
        BOOST_TEST(c == LogicGate().And(a, b));
    }
}

BOOST_AUTO_TEST_CASE(ors) {
    const std::array<std::tuple<R,R,R>, 4> truth{
        std::make_tuple(R(0), R(0), R(0)),
        std::make_tuple(R(0), R(1), R(1)),
        std::make_tuple(R(1), R(0), R(1)),
        std::make_tuple(R(1), R(1), R(1))
    };
    for (auto&& row : truth) {
        auto&& [a, b, c] = row;
        BOOST_TEST(c == LogicGate().Or(a, b));
    }
}

BOOST_AUTO_TEST_CASE(nots) {
    const std::array<std::tuple<R,R>, 2> truth{
        std::make_tuple(R(0), R(1)),
        std::make_tuple(R(1), R(0)),
    };
    for (auto&& row : truth) {
        auto&& [a, b] = row;
        BOOST_TEST(b == LogicGate().Not(a));
    }
}

BOOST_AUTO_TEST_SUITE_END()

BOOST_AUTO_TEST_SUITE(LogicGate_Circuit)

using R = PervushinRing;
using LogicGate = LogicGate<R>;

BOOST_AUTO_TEST_CASE(less_or_equal_checks) {
    const VectorDense<R> a{0,1,0,0};
    const VectorDense<R> b{0,0,1,0};

    using Builder = CircuitBuilder<R, 2>;
    Builder circuit;
    using VectorDenseCircuit = VectorDense<R>::Circuit<Builder>;
    VectorDenseCircuit a_circuit(&circuit, Builder::Variable::Type::Input, a.size());
    using Circuit = LogicGate::Circuit<Builder>;
    Circuit logic_gate_circuit(&circuit);
    logic_gate_circuit.LessOrEqualCheck(a_circuit, b);

    R1CS<R> r1cs(circuit.r1cs());
    VectorDense<R> z = r1cs.assigment();
    std::ranges::copy(a, std::back_inserter(z.elements));

    using Assigner = LogicGate::Assigner<Builder::degree()>;
    Assigner logic_gate_assigner(&z.elements);
    logic_gate_assigner.LessOrEqualCheck(a, b);
    BOOST_TEST(r1cs.isSatisfied(z));
}

BOOST_AUTO_TEST_CASE(xors) {
    const R a(1), b(1), c(0);

    using Builder = CircuitBuilder<R, 2>;
    Builder circuit;
    auto a_var = circuit.input();
    auto b_var = circuit.input();
    using Circuit = LogicGate::Circuit<Builder>;
    Circuit logic_gate_circuit(&circuit);
    auto c_lc = logic_gate_circuit.Xor(a_var, b_var);

    R1CS<R> r1cs(circuit.r1cs());
    VectorDense<R> z = r1cs.assigment();
    z.elements.push_back(a);
    z.elements.push_back(b);

    using Assigner = LogicGate::Assigner<Builder::degree()>;
    Assigner logic_gate_assigner(&z.elements);
    BOOST_TEST(c == logic_gate_assigner.Xor(a, b));
    BOOST_TEST(r1cs.isSatisfied(z));
}

BOOST_AUTO_TEST_CASE(ands) {
    const R a(1), b(1), c(1);

    using Builder = CircuitBuilder<R, 2>;
    Builder circuit;
    auto a_var = circuit.input();
    auto b_var = circuit.input();
    using Circuit = LogicGate::Circuit<Builder>;
    Circuit logic_gate_circuit(&circuit);
    auto c_lc = logic_gate_circuit.And(a_var, b_var);

    R1CS<R> r1cs(circuit.r1cs());
    VectorDense<R> z = r1cs.assigment();
    z.elements.push_back(a);
    z.elements.push_back(b);

    using Assigner = LogicGate::Assigner<Builder::degree()>;
    Assigner logic_gate_assigner(&z.elements);
    BOOST_TEST(c == logic_gate_assigner.And(a, b));
    BOOST_TEST(r1cs.isSatisfied(z));
}

BOOST_AUTO_TEST_CASE(ors) {
    const R a(1), b(1), c(1);

    using Builder = CircuitBuilder<R, 2>;
    Builder circuit;
    auto a_var = circuit.input();
    auto b_var = circuit.input();
    using Circuit = LogicGate::Circuit<Builder>;
    Circuit logic_gate_circuit(&circuit);
    auto c_lc = logic_gate_circuit.Or(a_var, b_var);

    R1CS<R> r1cs(circuit.r1cs());
    VectorDense<R> z = r1cs.assigment();
    z.elements.push_back(a);
    z.elements.push_back(b);

    using Assigner = LogicGate::Assigner<Builder::degree()>;
    Assigner logic_gate_assigner(&z.elements);
    BOOST_TEST(c == logic_gate_assigner.Or(a, b));
    BOOST_TEST(r1cs.isSatisfied(z));
}

BOOST_AUTO_TEST_CASE(nots) {
    const R a(1), b(0);

    using Builder = CircuitBuilder<R, 2>;
    Builder circuit;
    auto a_var = circuit.input();
    using Circuit = LogicGate::Circuit<Builder>;
    Circuit logic_gate_circuit(&circuit);
    auto b_lc = logic_gate_circuit.Not(a_var);

    R1CS<R> r1cs(circuit.r1cs());
    VectorDense<R> z = r1cs.assigment();
    z.elements.push_back(a);

    using Assigner = LogicGate::Assigner<Builder::degree()>;
    Assigner logic_gate_assigner(&z.elements);
    BOOST_TEST(b == logic_gate_assigner.Not(a));
    BOOST_TEST(r1cs.isSatisfied(z));
}

BOOST_AUTO_TEST_SUITE_END()

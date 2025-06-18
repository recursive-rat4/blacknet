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
#include <array>
#include <tuple>

#include "circuitbuilder.h"
#include "logicgate.h"
#include "pervushinextension.h"
#include "r1cs.h"

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

BOOST_AUTO_TEST_CASE(xors) {
    const R a(1), b(1), c(0);

    using Builder = CircuitBuilder<R, 2>;
    Builder circuit;
    auto a_var = circuit.input();
    auto b_var = circuit.input();
    using Circuit = LogicGate::Circuit<Builder>;
    Circuit logic_gate_circuit(circuit);
    auto c_lc = logic_gate_circuit.Xor(a_var, b_var);

    R1CS<R> r1cs(circuit.r1cs());
    Vector<R> z = r1cs.assigment();
    z.elements.push_back(a);
    z.elements.push_back(b);

    using Tracer = LogicGate::Tracer;
    Tracer logic_gate_tracer(z.elements);
    BOOST_TEST(c == logic_gate_tracer.Xor(a, b));
    BOOST_TEST(r1cs.isSatisfied(z));
}

BOOST_AUTO_TEST_CASE(ands) {
    const R a(1), b(1), c(1);

    using Builder = CircuitBuilder<R, 2>;
    Builder circuit;
    auto a_var = circuit.input();
    auto b_var = circuit.input();
    using Circuit = LogicGate::Circuit<Builder>;
    Circuit logic_gate_circuit(circuit);
    auto c_lc = logic_gate_circuit.And(a_var, b_var);

    R1CS<R> r1cs(circuit.r1cs());
    Vector<R> z = r1cs.assigment();
    z.elements.push_back(a);
    z.elements.push_back(b);

    using Tracer = LogicGate::Tracer;
    Tracer logic_gate_tracer(z.elements);
    BOOST_TEST(c == logic_gate_tracer.And(a, b));
    BOOST_TEST(r1cs.isSatisfied(z));
}

BOOST_AUTO_TEST_CASE(ors) {
    const R a(1), b(1), c(1);

    using Builder = CircuitBuilder<R, 2>;
    Builder circuit;
    auto a_var = circuit.input();
    auto b_var = circuit.input();
    using Circuit = LogicGate::Circuit<Builder>;
    Circuit logic_gate_circuit(circuit);
    auto c_lc = logic_gate_circuit.Or(a_var, b_var);

    R1CS<R> r1cs(circuit.r1cs());
    Vector<R> z = r1cs.assigment();
    z.elements.push_back(a);
    z.elements.push_back(b);

    using Tracer = LogicGate::Tracer;
    Tracer logic_gate_tracer(z.elements);
    BOOST_TEST(c == logic_gate_tracer.Or(a, b));
    BOOST_TEST(r1cs.isSatisfied(z));
}

BOOST_AUTO_TEST_CASE(nots) {
    const R a(1), b(0);

    using Builder = CircuitBuilder<R, 2>;
    Builder circuit;
    auto a_var = circuit.input();
    using Circuit = LogicGate::Circuit<Builder>;
    Circuit logic_gate_circuit(circuit);
    auto b_lc = logic_gate_circuit.Not(a_var);

    R1CS<R> r1cs(circuit.r1cs());
    Vector<R> z = r1cs.assigment();
    z.elements.push_back(a);

    using Tracer = LogicGate::Tracer;
    Tracer logic_gate_tracer(z.elements);
    BOOST_TEST(b == logic_gate_tracer.Not(a));
    BOOST_TEST(r1cs.isSatisfied(z));
}

BOOST_AUTO_TEST_SUITE_END()

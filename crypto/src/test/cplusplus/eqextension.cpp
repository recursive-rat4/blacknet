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
#include <ranges>

#include "circuitbuilder.h"
#include "customizableconstraintsystem.h"
#include "eqextension.h"
#include "hypercube.h"
#include "point.h"
#include "solinas62.h"
#include "util.h"

using namespace blacknet::crypto;

using E = Solinas62Ring;

BOOST_AUTO_TEST_SUITE(EqExtension_Plain)

BOOST_AUTO_TEST_CASE(meta) {
    std::vector<E> a{E(1), E(0), E(0)};
    EqExtension eq(a);
    BOOST_TEST(1 == eq.degree());
    BOOST_TEST(3 == eq.variables());
}

BOOST_AUTO_TEST_CASE(mul) {
    EqExtension<E> a({E(2), E(3), E(5), E(7)});
    E b(11);
    Point<E> r{E(13), E(17), E(23), E(27)};
    BOOST_TEST(a(r) * b == (a * b)(r));
}

BOOST_AUTO_TEST_CASE(bind) {
    EqExtension<E> eq1({E(2), E(3), E(4)});
    Point<E> r1{E(5), E(6), E(7)};
    EqExtension<E> eq2(eq1);
    eq2.bind(E(5));
    Point<E> r2{E(6), E(7)};
    EqExtension<E> eq3(eq2);
    eq3.bind(E(6));
    Point<E> r3{E(7)};
    BOOST_TEST(eq1(r1) == eq2(r2));
    BOOST_TEST(eq1(r1) == eq3(r3));

    std::vector<E> evaluations(4);
    EqExtension<E> eq = eq1;
    eq.bind(E(-2));
    eq1.bind<E(-2), util::Assign<E>>(evaluations);
    BOOST_TEST(eq() == evaluations);
    eq = eq1;
    eq.bind(E(-1));
    eq1.bind<E(-1), util::Assign<E>>(evaluations);
    BOOST_TEST(eq() == evaluations);
    eq = eq1;
    eq.bind(E(0));
    eq1.bind<E(0), util::Assign<E>>(evaluations);
    BOOST_TEST(eq() == evaluations);
    eq = eq1;
    eq.bind(E(1));
    eq1.bind<E(1), util::Assign<E>>(evaluations);
    BOOST_TEST(eq() == evaluations);
    eq = eq1;
    eq.bind(E(2));
    eq1.bind<E(2), util::Assign<E>>(evaluations);
    BOOST_TEST(eq() == evaluations);
    eq = eq1;
    eq.bind(E(3));
    eq1.bind<E(3), util::Assign<E>>(evaluations);
    BOOST_TEST(eq() == evaluations);
    eq = eq1;
    eq.bind(E(4));
    eq1.bind<E(4), util::Assign<E>>(evaluations);
    BOOST_TEST(eq() == evaluations);

    std::vector<E> pis(eq2());
    Hypercube<E> hc(eq2.variables());
    for (const auto& [index, b] : std::views::zip(
            std::ranges::subrange(hc.composedBegin(), hc.composedEnd()),
            std::ranges::subrange(hc.decomposedBegin(), hc.decomposedEnd())
        )) {
        BOOST_TEST(eq2(b) == pis[index]);
    };
}

BOOST_AUTO_TEST_CASE(point) {
    Hypercube<E> hc(3);
    Point<E> a{E(1), E(0), E(0)};
    EqExtension eq(a.coordinates);
    std::ranges::for_each(hc.decomposedBegin(), hc.decomposedEnd(), [&](const Point<E>& b) {
        if (a == b)
            BOOST_TEST(E(1) == eq(b));
        else
            BOOST_TEST(E(0) == eq(b));
    });
}

BOOST_AUTO_TEST_CASE(hypercube) {
    Hypercube<E> hc(3);
    std::vector<E> a{E(1), E(0), E(0)};
    EqExtension eq(a);
    std::vector<E> pis(eq());
    std::ranges::for_each(hc.composedBegin(), hc.composedEnd(), [&](const std::size_t& i) {
        if (i == 4)
            BOOST_TEST(E(1) == pis[i]);
        else
            BOOST_TEST(E(0) == pis[i]);
    });
}

BOOST_AUTO_TEST_SUITE_END()

BOOST_AUTO_TEST_SUITE(EqExtension_Circuit)

BOOST_AUTO_TEST_CASE(points) {
    EqExtension<E> eq({E(2), E(3), E(5)});
    Point<E> x{E(7), E(11), E(13)};

    using Builder = CircuitBuilder<E, 2>;
    Builder circuit;
    using EqCircuit = EqExtension<E>::Circuit<Builder>;
    EqCircuit eq_circuit(circuit, Builder::Variable::Type::Input, 3);
    using PointCircuit = Point<E>::Circuit<Builder>;
    PointCircuit x_circuit(circuit, Builder::Variable::Type::Input, 3);
    eq_circuit(x_circuit);

    CustomizableConstraintSystem<E> ccs(circuit.ccs());
    VectorDense<E> z = ccs.assigment();
    std::ranges::copy(eq.coefficients, std::back_inserter(z.elements));
    std::ranges::copy(x, std::back_inserter(z.elements));

    using EqAssigner = EqExtension<E>::Assigner<Builder::degree()>;
    EqAssigner eq_assigner(z.elements);
    BOOST_TEST(eq(x) == eq_assigner.point(eq, x));
    BOOST_TEST(ccs.isSatisfied(z));
}

BOOST_AUTO_TEST_CASE(hypercubes) {
    EqExtension<E> eq({E(29), E(31), E(37)});

    using Builder = CircuitBuilder<E, 2>;
    Builder circuit;
    using EqCircuit = EqExtension<E>::Circuit<Builder>;
    EqCircuit eq_circuit(circuit, Builder::Variable::Type::Input, 3);
    EqCircuit::hypercube(circuit, eq_circuit.coefficients);

    CustomizableConstraintSystem<E> ccs(circuit.ccs());
    VectorDense<E> z = ccs.assigment();
    std::ranges::copy(eq.coefficients, std::back_inserter(z.elements));

    using EqAssigner = EqExtension<E>::Assigner<Builder::degree()>;
    EqAssigner eq_assigner(z.elements);
    BOOST_TEST(eq() == eq_assigner.hypercube(eq.coefficients));
    BOOST_TEST(ccs.isSatisfied(z));
}

BOOST_AUTO_TEST_SUITE_END()

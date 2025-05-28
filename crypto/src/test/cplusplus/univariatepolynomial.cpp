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
#include "solinas62.h"
#include "univariatepolynomial.h"

using namespace blacknet::crypto;

BOOST_AUTO_TEST_SUITE(UnivariatePolynomials)

using E = Solinas62Ring;

BOOST_AUTO_TEST_CASE(meta) {
    UnivariatePolynomial up{E(2), E(3), E(4), E(5)};
    BOOST_TEST(3 == up.degree());
    BOOST_TEST(1 == up.variables());
}

BOOST_AUTO_TEST_CASE(point) {
    UnivariatePolynomial a{E(2), E(3), E(4), E(5)};
    UnivariatePolynomial b{E(2), E(3), E(4)};
    UnivariatePolynomial c{E(2), E(3)};
    UnivariatePolynomial d{E(2)};

    BOOST_TEST(E(398) == a(E(4)));
    BOOST_TEST(E(78) == b(E(4)));
    BOOST_TEST(E(14) == c(E(4)));
    BOOST_TEST(E(2) == d(E(4)));

    BOOST_TEST(E(16) == a.at_0_plus_1());
    BOOST_TEST(E(11) == b.at_0_plus_1());
    BOOST_TEST(E(7) == c.at_0_plus_1());
    BOOST_TEST(E(4) == d.at_0_plus_1());
}

BOOST_AUTO_TEST_CASE(circuit) {
    UnivariatePolynomial<E> p{E(2), E(3), E(4), E(5), E(6)};
    E x(7);

    using Circuit = CircuitBuilder<E, 2>;
    Circuit circuit;
    using Gadget = UnivariatePolynomial<E>::Gadget<Circuit>;
    Gadget gadget(circuit, Circuit::Variable::Type::Input, p.degree());
    typename Circuit::LinearCombination x_var(circuit.input());
    auto y_lc = gadget(x_var);
    typename Circuit::Variable y_var(circuit.auxiliary());
    circuit(y_var == y_lc);
    CustomizableConstraintSystem<E> ccs(circuit.ccs());
    Vector<E> z = ccs.assigment();
    std::ranges::copy(p.coefficients, std::back_inserter(z.elements));
    z.elements.push_back(x);
    using Tracer = UnivariatePolynomial<E>::Tracer;
    Tracer tracer(p, z.elements);
    z.elements.emplace_back(
        tracer(x)
    );
    BOOST_TEST(p(x) == z.elements.back());
    BOOST_TEST(ccs.isSatisfied(z));
}

BOOST_AUTO_TEST_SUITE_END()

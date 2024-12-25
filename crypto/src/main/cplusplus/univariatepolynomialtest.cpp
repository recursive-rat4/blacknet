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
#include <ranges>

#include "ccsbuilder.h"
#include "solinas62.h"
#include "solinas62field.h"
#include "univariatepolynomial.h"

BOOST_AUTO_TEST_SUITE(UnivariatePolynomials)

using E = Solinas62Ring;
using EE = Solinas62RingDegree2;

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
}

BOOST_AUTO_TEST_CASE(homomorphism) {
    UnivariatePolynomial<E> p1({E(20), E(21), E(22), E(23)});
    UnivariatePolynomial<EE> p2 = p1.homomorph<EE>();
    BOOST_TEST(EE(p1(E(24))) == p2(EE(24)));
}

BOOST_AUTO_TEST_CASE(circuit) {
    UnivariatePolynomial<E> p{E(2), E(3), E(4), E(5), E(6)};
    E x(7);

    using Circuit = CCSBuilder<E, 2>;
    Circuit circuit;
    std::array<typename Circuit::LinearCombination, 5> c_vars;
    std::ranges::generate(c_vars, [&]{ return circuit.input(); });
    typename Circuit::LinearCombination x_var(circuit.input());
    auto y_lc = UnivariatePolynomial<E>::circuit<Circuit>::point(circuit, c_vars, x_var);
    typename Circuit::Variable y_var(circuit.auxiliary());
    circuit(y_var == y_lc);
    CustomizableConstraintSystem<E> ccs(circuit.ccs());
    Vector<E> z;
    z.elements.reserve(ccs.variables());
    z.elements.emplace_back(E(1));
    std::ranges::copy(p.coefficients, std::back_inserter(z.elements));
    z.elements.push_back(x);
    z.elements.emplace_back(
        UnivariatePolynomial<E>::trace::point(p, x, z.elements)
    );
    BOOST_TEST(p(x) == z.elements.back());
    BOOST_TEST(ccs.variables() == z.size());
    BOOST_TEST(ccs.isSatisfied(z));
    for (std::size_t i = 1; i < z.size(); ++i) {
        z[i] += E(1);
        BOOST_TEST(!ccs.isSatisfied(z));
        z[i] -= E(1);
    }
}

BOOST_AUTO_TEST_SUITE_END()

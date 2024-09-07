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

#include "solinas62.h"
#include "univariatepolynomial.h"

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
}

BOOST_AUTO_TEST_CASE(interpolation) {
    UnivariatePolynomial lp{E(7), E(5)};
    E lp0(7);
    E lp1(12);
    BOOST_TEST(lp == UnivariatePolynomial<E>::interpolate(lp0, lp1));
}

BOOST_AUTO_TEST_SUITE_END()

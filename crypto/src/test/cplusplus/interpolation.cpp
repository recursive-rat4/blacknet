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

#include "interpolation.h"
#include "pervushin.h"
#include "pervushinfield.h"

BOOST_AUTO_TEST_SUITE(Interpolations)

using Z = PervushinRing;
using F = PervushinRingDegree2;

BOOST_AUTO_TEST_CASE(balance) {
    using Interpolation = Interpolation<Z, F>;
    UnivariatePolynomial<F> p1{F(2), F(3)};
    UnivariatePolynomial<F> p2{F(2), F(3), F(5)};
    UnivariatePolynomial<F> p4{F(2), F(3), F(5), F(7), F(11)};
    UnivariatePolynomial<F> p5{F(2), F(3), F(5), F(7), F(11), F(13)};
    BOOST_TEST(p1 == Interpolation::balanced(F(2), F(5)));
    BOOST_TEST(p2 == Interpolation::balanced(F(4), F(2), F(10)));
    BOOST_TEST(p4 == Interpolation::balanced(F(136), F(8), F(2), F(28), F(260)));
    BOOST_TEST(p5 == Interpolation::balanced(F(-280), F(-5), F(2), F(41), F(676), F(4295)));
}

BOOST_AUTO_TEST_SUITE_END()

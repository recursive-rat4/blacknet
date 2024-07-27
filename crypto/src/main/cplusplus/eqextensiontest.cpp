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

#include "eqextension.h"
#include "hypercube.h"
#include "pastacurves.h"

BOOST_AUTO_TEST_SUITE(EqExtensions)

BOOST_AUTO_TEST_CASE(point) {
    using E = PallasField;
    Hypercube<E> hc(3);
    std::vector<E> a{E(1), E(0), E(0)};
    EqExtension eq(a);
    std::ranges::for_each(hc.decomposedBegin(), hc.decomposedEnd(), [&](const std::vector<E>& b) {
        if (a == b)
            BOOST_TEST(E(1) == eq(b));
        else
            BOOST_TEST(E(0) == eq(b));
    });
}

BOOST_AUTO_TEST_CASE(hypercube) {
    using E = PallasField;
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

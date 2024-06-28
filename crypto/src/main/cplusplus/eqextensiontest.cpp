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
#include "pastacurves.h"

BOOST_AUTO_TEST_SUITE(EqExtensions)

BOOST_AUTO_TEST_CASE(evaluate) {
    using E = PallasField;
    std::vector<E> a{E(1), E(0), E(0)};
    std::vector<E> b(3);
    EqExtension eq(a);
    for (std::size_t i = 0; i <= 1; ++i) {
        for (std::size_t j = 0; j <= 1; ++j) {
            for (std::size_t k = 0; k <= 1; ++k) {
                b[0] = i ? E(1) : E(0);
                b[1] = j ? E(1) : E(0);
                b[2] = k ? E(1) : E(0);
                if (a == b)
                    BOOST_TEST(E(1) == eq(b));
                else
                    BOOST_TEST(E(0) == eq(b));
            }
        }
    }
}

BOOST_AUTO_TEST_SUITE_END()

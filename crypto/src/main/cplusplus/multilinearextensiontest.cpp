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

#include "matrix.h"
#include "multilinearextension.h"
#include "pastacurves.h"
#include "vector.h"

BOOST_AUTO_TEST_SUITE(MultilinearExtensions)

BOOST_AUTO_TEST_CASE(matrix) {
    using E = PallasField;
    Matrix<E> a(2, 4, {
        E(30), E(31), E(32), E(33),
        E(43), E(44), E(45), E(46),
    });
    std::vector<E> b(3);
    MultilinearExtension mle(a);
    for (std::size_t i = 0; i <= 1; ++i) {
        for (std::size_t j = 0; j <= 1; ++j) {
            for (std::size_t k = 0; k <= 1; ++k) {
                std::size_t row = i;
                std::size_t column = j << 1 | k;
                b[0] = i ? E(1) : E(0);
                b[1] = j ? E(1) : E(0);
                b[2] = k ? E(1) : E(0);
                BOOST_TEST((a[row, column] == mle(b)));
            }
        }
    }
}

BOOST_AUTO_TEST_CASE(vector) {
    using E = PallasField;
    Vector<E> a{E(63), E(64), E(65), E(66), E(67), E(68), E(69), E(70)};
    std::vector<E> b(3);
    MultilinearExtension mle(a);
    for (std::size_t i = 0; i <= 1; ++i) {
        for (std::size_t j = 0; j <= 1; ++j) {
            for (std::size_t k = 0; k <= 1; ++k) {
                std::size_t index = i << 2 | j << 1 | k;
                b[0] = i ? E(1) : E(0);
                b[1] = j ? E(1) : E(0);
                b[2] = k ? E(1) : E(0);
                BOOST_TEST(a[index] == mle(b));
            }
        }
    }
}

BOOST_AUTO_TEST_SUITE_END()

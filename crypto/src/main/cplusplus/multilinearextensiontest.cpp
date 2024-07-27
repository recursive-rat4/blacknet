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

#include "hypercube.h"
#include "matrix.h"
#include "multilinearextension.h"
#include "pastacurves.h"
#include "vector.h"

BOOST_AUTO_TEST_SUITE(MultilinearExtensions)

BOOST_AUTO_TEST_CASE(matrix) {
    using E = PallasField;
    Hypercube<E> hc(3);
    Matrix<E> a(2, 4, {
        E(30), E(31), E(32), E(33),
        E(43), E(44), E(45), E(46),
    });
    MultilinearExtension mle(a);
    for (std::tuple<const std::pair<std::size_t, std::size_t>&, const std::vector<E>&> i : std::views::zip(
            std::ranges::subrange(hc.splittedBegin(2, 4), hc.splittedEnd()),
            std::ranges::subrange(hc.decomposedBegin(), hc.decomposedEnd())
        )) {
        const std::size_t& row = std::get<0>(i).first;
        const std::size_t& column = std::get<0>(i).second;
        const std::vector<E>& b = std::get<1>(i);
        BOOST_TEST((a[row, column] == mle(b)));
    };
}

BOOST_AUTO_TEST_CASE(vector) {
    using E = PallasField;
    Hypercube<E> hc(3);
    Vector<E> a{E(63), E(64), E(65), E(66), E(67), E(68), E(69), E(70)};
    MultilinearExtension mle(a);
    for (std::tuple<const std::size_t&, const std::vector<E>&> i : std::views::zip(
            std::ranges::subrange(hc.composedBegin(), hc.composedEnd()),
            std::ranges::subrange(hc.decomposedBegin(), hc.decomposedEnd())
        )) {
        const std::size_t& index = std::get<0>(i);
        const std::vector<E>& b = std::get<1>(i);
        BOOST_TEST(a[index] == mle(b));
    };
}

BOOST_AUTO_TEST_SUITE_END()

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

#include "hypercube.h"
#include "multilinearextension.h"
#include "solinas62.h"

using namespace blacknet::crypto;

BOOST_AUTO_TEST_SUITE(Hypercubes)

BOOST_AUTO_TEST_CASE(composed) {
    using V = std::size_t;
    Hypercube<bool> hypercube(4);
    V vertices[16] = { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15 };
    BOOST_TEST(std::ranges::distance(vertices) == std::ranges::distance(hypercube.composedBegin(), hypercube.composedEnd()));
    for (const auto& [a, b] : std::views::zip(
            std::ranges::subrange{std::ranges::begin(vertices), std::ranges::end(vertices)},
            std::ranges::subrange(hypercube.composedBegin(), hypercube.composedEnd())
        )) {
        BOOST_TEST(a == b);
    };
}

BOOST_AUTO_TEST_CASE(decomposed) {
    using V = std::vector<bool>;
    Hypercube<bool> hypercube(3);
    std::vector<V> vertices{
        {0, 0, 0},
        {0, 0, 1},
        {0, 1, 0},
        {0, 1, 1},
        {1, 0, 0},
        {1, 0, 1},
        {1, 1, 0},
        {1, 1, 1},
    };
    BOOST_TEST(std::ranges::distance(vertices) == std::ranges::distance(hypercube.decomposedBegin(), hypercube.decomposedEnd()));
    for (const auto& [a, b] : std::views::zip(
            std::ranges::subrange{std::ranges::begin(vertices), std::ranges::end(vertices)},
            std::ranges::subrange(hypercube.decomposedBegin(), hypercube.decomposedEnd())
        )) {
        BOOST_TEST(a == b);
    };
}

BOOST_AUTO_TEST_CASE(splitted) {
    using V = std::pair<std::size_t, std::size_t>;
    Hypercube<bool> hypercube(3);
    V vertices[8] = {
        {0, 0}, {0, 1},
        {1, 0}, {1, 1},
        {2, 0}, {2, 1},
        {3, 0}, {3, 1},
    };
    BOOST_TEST(std::ranges::distance(vertices) == std::ranges::distance(hypercube.splittedBegin(4, 2), hypercube.splittedEnd()));
    for (const auto& [a, b] : std::views::zip(
            std::ranges::subrange{std::ranges::begin(vertices), std::ranges::end(vertices)},
            std::ranges::subrange(hypercube.splittedBegin(4, 2), hypercube.splittedEnd())
        )) {
        BOOST_TEST((a == b));
    };
}

BOOST_AUTO_TEST_CASE(sum) {
    using R = Solinas62Ring;
    MultilinearExtension<R> p0{R(1), R(2), R(3), R(-6)};
    MultilinearExtension<R> p1{R(0), R(1), R(0), R(-0)};
    BOOST_TEST(R(0) == Hypercube<R>::sum(p0));
    BOOST_TEST(R(1) == Hypercube<R>::sum(p1));
}

BOOST_AUTO_TEST_SUITE_END()

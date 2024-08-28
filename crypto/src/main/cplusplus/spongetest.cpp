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

#include "sponge.h"

BOOST_AUTO_TEST_SUITE(Sponges)

BOOST_AUTO_TEST_CASE(test) {
    using Z = uint8_t;
    using B = std::array<Z, 4>;
    using S = Sponge<Z, 2, 2, [](B& b){ for (auto& e : b) e += 1; }>;

    S sponge1;
    BOOST_TEST((B{0, 0, 0, 0} == sponge1.state));
    sponge1.absorb(2);
    BOOST_TEST((B{2, 0, 0, 0} == sponge1.state));
    sponge1.absorb(4);
    BOOST_TEST((B{2, 4, 0, 0} == sponge1.state));
    sponge1.absorb(6);
    BOOST_TEST((B{6, 5, 1, 1} == sponge1.state));

    S sponge2(sponge1);
    sponge2.absorb(8);
    BOOST_TEST((B{6, 8, 1, 1} == sponge2.state));

    BOOST_TEST((Z{7} == sponge1.squeeze()));
    BOOST_TEST((B{7, 2, 2, 4} == sponge1.state));
    BOOST_TEST((Z{2} == sponge1.squeeze()));
    BOOST_TEST((B{7, 2, 2, 4} == sponge1.state));
    BOOST_TEST((Z{8} == sponge1.squeeze()));
    BOOST_TEST((B{8, 3, 3, 5} == sponge1.state));
    BOOST_CHECK_THROW(sponge1.absorb(0), SpongeException);

    sponge2.absorb(10);
    BOOST_TEST((B{10, 9, 2, 2} == sponge2.state));
    sponge2.absorb(12);
    BOOST_TEST((B{10, 12, 2, 2} == sponge2.state));
    BOOST_TEST((Z{11} == sponge2.squeeze()));
    BOOST_TEST((B{11, 13, 3, 4} == sponge2.state));

    S sponge3;
    BOOST_TEST((Z{2} == sponge3.squeeze()));
    BOOST_TEST((B{2, 1, 1, 3} == sponge3.state));
}

BOOST_AUTO_TEST_SUITE_END()

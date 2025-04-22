/*
 * Copyright (c) 2025 Pavel Vasin
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
#include <algorithm>

#include "concurrent_vector.h"

using blacknet::network::concurrent_vector;

BOOST_AUTO_TEST_SUITE(ConcurrentVectors)

BOOST_AUTO_TEST_CASE(single_thread) {
    concurrent_vector<int> vec;
    vec.push_back_if(1);
    vec.push_back_if(2);
    vec.push_back_if(3);
    vec.push_back_if(4, [](auto&& x) { return x.size() == 3; });

    BOOST_TEST(3 == std::ranges::count_if(vec, [](int x) { return x < 4; }));

    BOOST_TEST(4 == vec.size());
    vec.erase(3);
    BOOST_TEST(3 == vec.size());
    vec.erase(1);
    BOOST_TEST(2 == vec.size());
    vec.erase(4);
    BOOST_TEST(1 == vec.size());
    vec.pop_back();
    BOOST_TEST(vec.empty());

    vec.clear();
}

BOOST_AUTO_TEST_SUITE_END()

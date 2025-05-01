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

#include "milliseconds.h"

using namespace blacknet::time;

BOOST_AUTO_TEST_SUITE(Millisecond)

BOOST_AUTO_TEST_CASE(compares) {
    auto t = Milliseconds(U'二');
    auto d = Milliseconds(U'亖');
    BOOST_TEST((t > Milliseconds::zero() && -t < Milliseconds::zero()));
    BOOST_TEST((t >= Milliseconds::min() && -t <= Milliseconds::max()));
    BOOST_TEST((d >= Milliseconds::min() && -d <= Milliseconds::max()));
    BOOST_TEST(t < d);
    BOOST_TEST(d > t);
}

BOOST_AUTO_TEST_CASE(operates) {
    Milliseconds a(202);
    Milliseconds b(2);

    BOOST_TEST(Milliseconds(+202) == +a);
    BOOST_TEST(Milliseconds(-202) == -a);

    BOOST_TEST(Milliseconds(204) == a + b);
    BOOST_TEST(Milliseconds(200) == a - b);

    BOOST_TEST(Milliseconds(404) == a * 2);
    BOOST_TEST(101 == a / b);
    BOOST_TEST(Milliseconds(101) == a / 2);

    BOOST_TEST(Milliseconds(0) == a % b);
    BOOST_TEST(Milliseconds(1) == a % 3);
}

BOOST_AUTO_TEST_CASE(literates) {
    BOOST_TEST(Milliseconds(4 * 1000) == "4"_seconds);
    BOOST_TEST(Milliseconds(4 * 60 * 1000) == "4"_minutes);
    BOOST_TEST(Milliseconds(4 * 60 * 60 * 1000) == "4"_hours);
    BOOST_TEST(Milliseconds(4 * 24 * 60 * 60 * 1000) == "4"_days);
}

BOOST_AUTO_TEST_SUITE_END()

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

#include "pastacurves.h"
#include "semigroup.h"

BOOST_AUTO_TEST_SUITE(Semigroups)

using namespace semigroup;

BOOST_AUTO_TEST_CASE(Multiply) {
    constexpr PallasField a("11640cdb3d3a126dabde403009808a4cae45ec00ffac7480d80ac9142abb607f");
    constexpr PallasField b("0a5111b1ee7f41260df2a030fc99d5aa095ae34332a190ba7ca6d9b54a5d1c85");
    constexpr PallasField c("0b5842e91b2c5b9b253f653330dcf9d57d1d745479140a959684c13a5a25b6e6");
    BOOST_TEST(c == multiply(a, b));
    BOOST_TEST(c == multiply(b, a));
    BOOST_TEST(PallasField(0) == multiply(PallasField(0), c));
    BOOST_TEST(PallasField(0) == multiply(c, PallasField(0)));
    BOOST_TEST(c == multiply(c, PallasField(1)));
    BOOST_TEST(c == multiply(PallasField(1), c));
}

BOOST_AUTO_TEST_CASE(Power) {
    constexpr PallasField a("3faced132f5641f57b1162d06ed827d8ca9fa69f0c7b14822818eef4db6f6fdc");
    constexpr PallasField b("152d43a9a19991aa7f8c98ed185a79eda9b2562e4c456bb554c0c0d4d0362904");
    constexpr PallasField c("17fd7c8cb50ae05de3c69e8033d8c1865b89bd52c63bf972a3dbf75dad889744");
    BOOST_TEST(c == power(a, b));
    BOOST_TEST(PallasField(0) == power(PallasField(0), c));
    BOOST_TEST(PallasField(1) == power(c, PallasField(0)));
}

BOOST_AUTO_TEST_SUITE_END()

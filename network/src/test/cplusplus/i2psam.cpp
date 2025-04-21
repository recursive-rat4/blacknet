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

#include "byte.h"
#include "i2psam.h"

using namespace blacknet;
using namespace blacknet::network::i2p;

BOOST_AUTO_TEST_SUITE(I2PSAMs)

BOOST_AUTO_TEST_CASE(values) {
    const Answer newlined{"HELLO REPLY RESULT=OK VERSION=3.3\n"};
    BOOST_TEST(newlined.get("VERSION").value() == "3.3");

    const Answer quoted{"HELLO REPLY RESULT=I2P_ERROR MESSAGE=\"Must start with HELLO VERSION\"\n"};
    BOOST_TEST(quoted.get("MESSAGE").value() == "Must start with HELLO VERSION");
}

BOOST_AUTO_TEST_CASE(oks) {
    const Answer yay{"HELLO REPLY RESULT=OK VERSION=3.3\n"};
    yay.ok();

    const Answer nay{"HELLO REPLY RESULT=I2P_ERROR MESSAGE=\"Must start with HELLO VERSION\"\n"};
    BOOST_CHECK_THROW(nay.ok(), Exception);
}

BOOST_AUTO_TEST_CASE(destinations) {
    BOOST_TEST(
        compat::byte::arrayU<32>({ 81, 169, 239, 153, 149, 11, 34, 49, 163, 77, 41, 180, 244, 162, 252, 194, 49, 92, 204, 43, 2, 56, 105, 63, 140, 102, 235, 132, 22, 244, 63, 19 })
        ==
        Answer::hash("EpnubNeLyuhb86IFUJkUUClggJOhQyV59n5kzG2bbXcSme5s14vK6FvzogVQmRRQKWCAk6FDJXn2fmTMbZttdxKZ7mzXi8roW~OiBVCZFFApYICToUMlefZ-ZMxtm213EpnubNeLyuhb86IFUJkUUClggJOhQyV59n5kzG2bbXcSme5s14vK6FvzogVQmRRQKWCAk6FDJXn2fmTMbZttdxKZ7mzXi8roW~OiBVCZFFApYICToUMlefZ-ZMxtm213EpnubNeLyuhb86IFUJkUUClggJOhQyV59n5kzG2bbXcSme5s14vK6FvzogVQmRRQKWCAk6FDJXn2fmTMbZttdxKZ7mzXi8roW~OiBVCZFFApYICToUMlefZ-ZMxtm213EpnubNeLyuhb86IFUJkUUClggJOhQyV59n5kzG2bbXcSme5s14vK6FvzogVQmRRQKWCAk6FDJXn2fmTMbZttd3bv4RZ3HHk0U1v2T5r8N6TFmPNsTli1XzmB20yGQHW4BQAEAAcAAA==")
    );
}

BOOST_AUTO_TEST_SUITE_END()

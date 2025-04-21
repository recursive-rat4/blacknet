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
#include "endpoint.h"

using namespace blacknet;
using namespace blacknet::network::endpoint;

BOOST_AUTO_TEST_SUITE(Endpoints)

BOOST_AUTO_TEST_CASE(I2Ps) {
    const std::string_view string
    {"y45f23mb2apgywmftrjmfg35oynzfwjed7rxs2mh76pbdeh4fatq.b32.i2p:28453"};
    const I2P endpoint{28453, compat::byte::arrayU<32>({
        0xC7, 0x3A, 0x5D, 0x6D, 0x81, 0xD0, 0x1E, 0x6C,
        0x59, 0x85, 0x9C, 0x52, 0xC2, 0x9B, 0x7D, 0x76,
        0x1B, 0x92, 0xD9, 0x24, 0x1F, 0xE3, 0x79, 0x69,
        0x87, 0xFF, 0x9E, 0x11, 0x90, 0xFC, 0x28, 0x27
    })};

    BOOST_TEST(!endpoint.is_local());
    BOOST_TEST(!endpoint.is_private());
    BOOST_CHECK_THROW(endpoint.to_boost(), Exception);
    BOOST_TEST(endpoint.to_log(true) == string);

    //TODO parse
}

BOOST_AUTO_TEST_SUITE_END()

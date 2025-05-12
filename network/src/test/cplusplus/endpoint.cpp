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
#include <array>
#include <initializer_list>
#include <string_view>
#include <tuple>

#include "byte.h"
#include "endpoint.h"
#include "size_output_stream.h"
#include "span_input_stream.h"
#include "span_output_stream.h"

using namespace blacknet;
using namespace blacknet::network::endpoint;

BOOST_AUTO_TEST_SUITE(Endpoints)

BOOST_AUTO_TEST_CASE(IPv4s) {
    const std::initializer_list<std::tuple<std::string_view, bool, bool>> data = {
        std::make_tuple("0.0.0.0", true, false),
        std::make_tuple("100.64.0.0", false, true),
        std::make_tuple("100.128.0.0", false, false),
        std::make_tuple("127.0.1.4", true, false),
        std::make_tuple("255.255.255.255", false, false),
    };
    for (auto [string, is_local, is_private] : data) {
        auto endpoint = parse(string, 28453);
        BOOST_TEST_REQUIRE(endpoint);
        BOOST_TEST((Enum::IPv4 == endpoint->ordinal()));
        BOOST_TEST(!endpoint->is_permissionless());
        BOOST_TEST(is_local == endpoint->is_local());
        BOOST_TEST(is_private == endpoint->is_private());
        endpoint->to_boost();
        BOOST_TEST(endpoint->to_host() == string);
    }
}

BOOST_AUTO_TEST_CASE(IPv6s) {
    const std::initializer_list<std::tuple<std::string_view, bool, bool>> data = {
        std::make_tuple("1001:1001:1001:1001:1001:1001:1001:1001", false, false),
        std::make_tuple("2001:8db8:8558:8888:1331:8aa8:3789:7337", false, false),
        std::make_tuple("f00f:f00f:f00f:f00f:f00f:f00f:f00f:f00f", false, false),
    };
    for (auto [string, is_local, is_private] : data) {
        auto endpoint = parse(string, 28453);
        BOOST_TEST_REQUIRE(endpoint);
        BOOST_TEST((Enum::IPv6 == endpoint->ordinal()));
        BOOST_TEST(!endpoint->is_permissionless());
        BOOST_TEST(is_local == endpoint->is_local());
        BOOST_TEST(is_private == endpoint->is_private());
        endpoint->to_boost();
        BOOST_TEST(endpoint->to_host() == string);
    }
}

BOOST_AUTO_TEST_CASE(TORv3s) {
    const std::initializer_list<std::string_view> data = {
        "pg6mmjiyjmcrsslvykfwnntlaru7p5svn6y2ymmju6nubxndf4pscryd.onion",
        "sp3k262uwy4r2k3ycr5awluarykdpag6a7y33jxop4cs2lu5uz5sseqd.onion",
        "xa4r2iadxm55fbnqgwwi5mymqdcofiu3w6rpbtqn7b2dyn7mgwj64jyd.onion",
    };
    for (auto string : data) {
        auto endpoint = parse(string, 28453);
        BOOST_TEST_REQUIRE(endpoint);
        BOOST_TEST((Enum::TORv3 == endpoint->ordinal()));
        BOOST_TEST(endpoint->is_permissionless());
        BOOST_TEST(!endpoint->is_local());
        BOOST_TEST(!endpoint->is_private());
        BOOST_CHECK_THROW(endpoint->to_boost(), Exception);
        BOOST_TEST(endpoint->to_host() == string);
    }
}

BOOST_AUTO_TEST_CASE(I2Ps) {
    const std::initializer_list<std::string_view> data = {
        "y45f23mb2apgywmftrjmfg35oynzfwjed7rxs2mh76pbdeh4fatq.b32.i2p",
    };
    for (auto string : data) {
        auto endpoint = parse(string, 28453);
        BOOST_TEST_REQUIRE(endpoint);
        BOOST_TEST((Enum::I2P == endpoint->ordinal()));
        BOOST_TEST(endpoint->is_permissionless());
        BOOST_TEST(!endpoint->is_local());
        BOOST_TEST(!endpoint->is_private());
        BOOST_CHECK_THROW(endpoint->to_boost(), Exception);
        BOOST_TEST(endpoint->to_host() == string);
    }
}

BOOST_AUTO_TEST_CASE(Compares) {
    auto a = parse("127.0.0.1", 12345);
    auto b = parse("127.0.0.2", 12345);
    auto c = parse("mzgt4svgc72euhvkpfdow7aiiivziqwhsl2fdzgiwkqeronnjjtq.b32.i2p", 0);
    auto d = parse("mzgt4svgc72euhvkpfdow7aiiivziqwhsl2fdzgiwkqeronnjjtq.b32.i2p", 0);

    BOOST_TEST_REQUIRE(a);
    BOOST_TEST_REQUIRE(b);
    BOOST_TEST_REQUIRE(c);
    BOOST_TEST_REQUIRE(d);

    BOOST_TEST((*a != *b));
    BOOST_TEST((*b != *c));
    BOOST_TEST((*c == *d));
    BOOST_TEST((*d != *a));
}

BOOST_AUTO_TEST_CASE(Serializes) {
    const auto endpoint = parse("127.0.0.4", 258);
    const auto bytes = compat::byte::arrayU<7>({
        0x80,
        0x01, 0x02,
        0x7F, 0x00, 0x00, 0x04
    });
    {
        io::span_input_stream<std::endian::big> sis(bytes);
        endpoint_ptr deserialized = deserialize(sis);
        BOOST_TEST_REQUIRE(deserialized);
        BOOST_TEST((*endpoint == *deserialized));
    } {
        io::size_output_stream sos;
        endpoint->serialize(sos);
        BOOST_TEST_REQUIRE(bytes.size() == sos.size);
    } {
        std::array<std::byte, bytes.size()> serialized;
        io::span_output_stream<std::endian::big> sos(serialized);
        endpoint->serialize(sos);
        BOOST_TEST(bytes == serialized);
    }
}

BOOST_AUTO_TEST_SUITE_END()

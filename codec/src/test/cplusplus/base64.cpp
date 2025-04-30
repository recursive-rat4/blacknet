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

#include <cstddef>
#include <algorithm>
#include <array>
#include <span>
#include <string_view>

#include "base64.h"

using namespace blacknet::codec;
using namespace blacknet::compat;

BOOST_AUTO_TEST_SUITE(Base64s)

BOOST_AUTO_TEST_CASE(RFC4648s) {
    using base64 = base64::codec<base64::rfc4648>;

    const std::array<std::pair<std::string_view, std::string_view>, 7> vectors{
        std::pair{ "", "" },
        std::pair{ "f", "Zg==" },
        std::pair{ "fo", "Zm8=" },
        std::pair{ "foo", "Zm9v" },
        std::pair{ "foob", "Zm9vYg==" },
        std::pair{ "fooba", "Zm9vYmE=" },
        std::pair{ "foobar", "Zm9vYmFy" },
    };

    for (auto [bytestring, string] : vectors) {
        auto bytes = std::span<const std::byte>(reinterpret_cast<const std::byte*>(bytestring.data()), bytestring.size());

        auto encoded = base64::encode(bytes);
        auto decoded = base64::decode(string);

        BOOST_TEST(string == encoded);
        BOOST_TEST(std::ranges::equal(bytes, decoded));
    }
}

BOOST_AUTO_TEST_SUITE_END()

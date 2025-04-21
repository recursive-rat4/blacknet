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
#include <array>
#include <string_view>
#include <utility>

#include "base32.h"
#include "boost-print.h"

using namespace blacknet::codec;
using namespace blacknet::compat;

BOOST_AUTO_TEST_SUITE(Base32s)

BOOST_AUTO_TEST_CASE(RFC4648s) {
    using base32 = base32::codec<base32::rfc4648>;

    const std::array<std::pair<std::string_view, std::string_view>, 7> vectors{
        std::pair{ "", "" },
        std::pair{ "f", "MY======" },
        std::pair{ "fo", "MZXQ====" },
        std::pair{ "foo", "MZXW6===" },
        std::pair{ "foob", "MZXW6YQ=" },
        std::pair{ "fooba", "MZXW6YTB" },
        std::pair{ "foobar", "MZXW6YTBOI======" },
    };

    for (auto [bytestring, string] : vectors) {
        auto bytes = std::span<const std::byte>(reinterpret_cast<const std::byte*>(bytestring.data()), bytestring.size());

        auto encoded = base32::encode(bytes);
        auto decoded = base32::decode(string);

        BOOST_TEST(string == encoded);
        BOOST_CHECK_EQUAL_COLLECTIONS(bytes.begin(), bytes.end(), decoded.begin(), decoded.end());
    }
}

BOOST_AUTO_TEST_SUITE_END()

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

#ifndef BLACKNET_CRYPTO_CIRCUITRY_H
#define BLACKNET_CRYPTO_CIRCUITRY_H

#include <boost/test/unit_test.hpp>

namespace test {
    template<typename CS, typename A>
    constexpr void circuitry(const CS& cs, A& a) {
        BOOST_TEST_REQUIRE(cs.variables() == a.size());
        BOOST_TEST(cs.isSatisfied(a));
        // Test that all variables (except constant) have at least one constraint
        // May result in false positives
        for (std::size_t i = 1; i < a.size(); ++i) {
            a[i] += typename A::ElementType(1);
            BOOST_TEST(!cs.isSatisfied(a));
            a[i] -= typename A::ElementType(1);
        }
    }
}

#endif

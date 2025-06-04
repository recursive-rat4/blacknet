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

#include <boost/test/unit_test.hpp>

#include "ajtaicommitment.h"
#include "poseidon2solinas62.h"
#include "solinas62.h"
#include "solinas62extension.h"

using namespace blacknet::crypto;

BOOST_AUTO_TEST_SUITE(AjtaiCommitments)

using Z = Solinas62Ring;
using R = Solinas62RingDegree64NTT;

BOOST_AUTO_TEST_CASE(Zq) {
    using CS = AjtaiCommitment<Z, NormP::Infinity>;
    auto drg = Poseidon2Solinas62Sponge<{1, 2, 3, 4}>();
    CS cs(CS::setup(drg, 2, 2), 8);
    Z z1(1);
    Z z2(2);
    Z z3(3);
    Z z4(4);
    auto c1 = cs.commit({z1, z2});
    auto c2 = cs.commit({z3, z4});
    BOOST_TEST(cs.open(c1, {z1, z2}), "Opening");
    BOOST_TEST(!cs.open(c2, {z1, z2}), "Binding");
    BOOST_TEST(!cs.open(c1, {z2, z1}), "Positional binding");
    BOOST_TEST(cs.open(c1 + c2, {z1 + z3, z2 + z4}), "Homomorphism");
}

BOOST_AUTO_TEST_CASE(Rq) {
    using CS = AjtaiCommitment<R, NormP::Infinity>;
    auto drg = Poseidon2Solinas62Sponge<{5, 6, 7, 8}>();
    CS cs(CS::setup(drg, 2, 2), 16);
    R r1({1, 2});
    R r2({3, 4});
    R r3({5, 6});
    R r4({7, 8});
    auto c1 = cs.commit({r1, r2});
    auto c2 = cs.commit({r3, r4});
    BOOST_TEST(cs.open(c1, {r1, r2}), "Opening");
    BOOST_TEST(!cs.open(c2, {r1, r2}), "Binding");
    BOOST_TEST(!cs.open(c1, {r2, r1}), "Positional binding");
    BOOST_TEST(cs.open(c1 + c2, {r1 + r3, r2 + r4}), "Homomorphism");
}

BOOST_AUTO_TEST_SUITE_END()

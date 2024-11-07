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
#include <boost/random/mersenne_twister.hpp>

#include "bfv.h"
#include "fermat.h"
#include "solinas62.h"

static boost::random::mt19937 rng;

BOOST_AUTO_TEST_SUITE(BFVs)

BOOST_AUTO_TEST_CASE(Tests) {
    using BFV = BFV<FermatRing, Solinas62Ring, 4>;
    BFV bfv;
    auto sk = bfv.generateSecretKey(rng);
    auto pk = bfv.generatePublicKey(rng, sk);
    BFV::PlainText pt{ 1, 2, 3, 4 };
    auto ct = bfv.encrypt(rng, pk, pt);
    BOOST_TEST(pt == bfv.decrypt(sk, ct), "Decryption");
}

BOOST_AUTO_TEST_SUITE_END()

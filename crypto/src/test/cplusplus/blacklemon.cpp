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
#include <boost/random/mersenne_twister.hpp>

#include "blacklemon.h"

static boost::random::mt19937 rng;

BOOST_AUTO_TEST_SUITE(BlackLemons)

BOOST_AUTO_TEST_CASE(Tests) {
    using Zq = BlackLemon::Zq;
    using Rq = BlackLemon::Rq;
    BlackLemon bl;
    auto sk = bl.generateSecretKey(rng);
    auto pk = bl.generatePublicKey(rng, sk);
    BlackLemon::PlainText pt{Zq(0), Zq(0), Zq(1), Zq(1)};
    auto ct = bl.encrypt(rng, pk, pt);
    BOOST_TEST(pt == bl.decrypt(sk, ct), "Decryption");

    auto snakeEye = BlackLemon::CipherText{ Rq(1), Rq(0) };
    BOOST_TEST(!bl.detect(sk, snakeEye).has_value(), "Snake-eye resistance");

    auto sk2 = bl.generateSecretKey(rng);
    BOOST_TEST(!bl.detect(sk2, ct).has_value(), "δ-snake-eye resistance");
}

BOOST_AUTO_TEST_SUITE_END()

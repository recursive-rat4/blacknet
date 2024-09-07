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
#include "poseidon2pasta.h"
#include "sumcheck.h"

BOOST_AUTO_TEST_SUITE(SumChecks)

using R = PallasField;
using SumCheck = SumCheck<R, Poseidon2Pallas>;

BOOST_AUTO_TEST_CASE(mle) {
    MultilinearExtension p1{R(7), R(7), R(7), R(0)};
    MultilinearExtension p2{R(7), R(7), R(7), R(7)};
    MultilinearExtension p3{R(7), R(7), R(0), R(7)};
    R s1(21);
    R s2(28);
    auto proof = SumCheck::prove(p1);
    BOOST_TEST(SumCheck::verify(p1, s1, proof));
    BOOST_TEST(!SumCheck::verify(p1, s2, proof));
    BOOST_TEST(!SumCheck::verify(p2, s1, proof));
    BOOST_TEST(!SumCheck::verify(p2, s2, proof));
    BOOST_TEST(!SumCheck::verify(p3, s1, proof));
    proof.claims[1].coefficients[1] += 1;
    BOOST_TEST(!SumCheck::verify(p1, s1, proof));
}

BOOST_AUTO_TEST_SUITE_END()

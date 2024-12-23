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

#include "customizableconstraintsystem.h"
#include "eqextension.h"
#include "multilinearextension.h"
#include "poseidon2solinas62.h"
#include "powextension.h"
#include "solinas62.h"
#include "solinas62field.h"
#include "sumcheck.h"

BOOST_AUTO_TEST_SUITE(SumChecks)

using Z = Solinas62Ring;
using F = Solinas62RingDegree2;
using RO = Poseidon2Solinas62;

BOOST_AUTO_TEST_CASE(interpolation) {
    using SumCheck = SumCheck<Z, F, UnivariatePolynomial, RO>;
    UnivariatePolynomial<F> p1{F(2), F(3)};
    UnivariatePolynomial<F> p2{F(2), F(3), F(5)};
    UnivariatePolynomial<F> p4{F(2), F(3), F(5), F(7), F(11)};
    UnivariatePolynomial<F> p5{F(2), F(3), F(5), F(7), F(11), F(13)};
    BOOST_TEST(p1 == SumCheck::interpolate(F(2), F(5)));
    BOOST_TEST(p2 == SumCheck::interpolate(F(4), F(2), F(10)));
    BOOST_TEST(p4 == SumCheck::interpolate(F(136), F(8), F(2), F(28), F(260)));
    BOOST_TEST(p5 == SumCheck::interpolate(F(-280), F(-5), F(2), F(41), F(676), F(4295)));
}

BOOST_AUTO_TEST_CASE(mle) {
    using SumCheck = SumCheck<Z, F, MultilinearExtension, RO>;
    MultilinearExtension p1{Z(7), Z(7), Z(7), Z(0)};
    MultilinearExtension p2{Z(7), Z(7), Z(7), Z(7)};
    MultilinearExtension p3{Z(7), Z(7), Z(0), Z(7)};
    Z s1(21);
    Z s2(28);

    auto proof = SumCheck::prove(p1, s1);
    BOOST_TEST(SumCheck::verify(p1, s1, proof));
    BOOST_TEST(!SumCheck::verify(p1, s2, proof));
    BOOST_TEST(!SumCheck::verify(p2, s1, proof));
    BOOST_TEST(!SumCheck::verify(p2, s2, proof));
    BOOST_TEST(!SumCheck::verify(p3, s1, proof));
    proof.claims[1].coefficients[1].coefficients[1] += Z(1);
    BOOST_TEST(!SumCheck::verify(p1, s1, proof));

    auto proof2 = SumCheck::prove(p1, s2);
    BOOST_TEST(!SumCheck::verify(p1, s1, proof2));
    BOOST_TEST(!SumCheck::verify(p1, s2, proof2));
}

BOOST_AUTO_TEST_CASE(eq) {
    using SumCheck = SumCheck<Z, F, EqExtension, RO>;
    EqExtension<Z> p1({Z(45), Z(46), Z(47), Z(48)});
    EqExtension<Z> p2({Z(45), Z(46), Z(48), Z(48)});
    Z s1(1);
    Z s2(2);

    auto proof = SumCheck::prove(p1, s1);
    BOOST_TEST(SumCheck::verify(p1, s1, proof));
    BOOST_TEST(!SumCheck::verify(p1, s2, proof));
    BOOST_TEST(!SumCheck::verify(p2, s1, proof));
    BOOST_TEST(!SumCheck::verify(p2, s2, proof));
    proof.claims[3].coefficients[1].coefficients[1] += Z(1);
    BOOST_TEST(!SumCheck::verify(p1, s1, proof));

    auto proof2 = SumCheck::prove(p1, s2);
    BOOST_TEST(!SumCheck::verify(p1, s1, proof2));
    BOOST_TEST(!SumCheck::verify(p1, s2, proof2));
}

BOOST_AUTO_TEST_CASE(ccs) {
    using CCS = CustomizableConstraintSystem<Z>;
    using SumCheck = SumCheck<Z, F, CCS::Polynomial, RO>;
    CCS::Polynomial<Z> ccs(1, 2, {{Z(7), Z(7), Z(7), Z(0)}}, {{0}}, {Z(1)});
    Z s(21);

    auto proof = SumCheck::prove(ccs, s);
    BOOST_TEST(SumCheck::verify(ccs, s, proof));
}

BOOST_AUTO_TEST_CASE(pow_early_stop) {
    using SumCheck = SumCheck<Z, F, PowExtension, RO>;
    PowExtension<Z> p1(Z(2), 4);
    PowExtension<Z> p2(Z(4), 4);
    Z s1(1);
    Z s2(2);

    auto proof = SumCheck::proveEarlyStopping(p1, s1);
    BOOST_TEST(SumCheck::verifyEarlyStopping(p1, s1, proof));
    BOOST_TEST(!SumCheck::verifyEarlyStopping(p1, s2, proof));
    BOOST_TEST(!SumCheck::verifyEarlyStopping(p2, s2, proof));

    proof.claim.coefficients[1] += Z(1);
    BOOST_TEST(!SumCheck::verifyEarlyStopping(p1, s1, proof));
    proof.claim.coefficients[1] -= Z(1);

    proof.challenge += Z(1);
    BOOST_TEST(!SumCheck::verifyEarlyStopping(p1, s1, proof));
    proof.challenge -= Z(1);

    auto proof2 = SumCheck::proveEarlyStopping(p1, s2);
    BOOST_TEST(!SumCheck::verifyEarlyStopping(p1, s1, proof2));
}

BOOST_AUTO_TEST_SUITE_END()

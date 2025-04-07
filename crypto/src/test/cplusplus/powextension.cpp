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
#include <ranges>

#include "ccsbuilder.h"
#include "circuitry.h"
#include "customizableconstraintsystem.h"
#include "hypercube.h"
#include "powextension.h"
#include "solinas62.h"
#include "solinas62field.h"
#include "util.h"

using namespace blacknet::crypto;

BOOST_AUTO_TEST_SUITE(PowExtensions)

using E = Solinas62Ring;
using EE = Solinas62RingDegree2;

BOOST_AUTO_TEST_CASE(meta) {
    E tau(7);
    std::size_t ell(3);
    PowExtension pow(tau, ell);
    BOOST_TEST(1 == pow.degree());
    BOOST_TEST(3 == pow.variables());
}

BOOST_AUTO_TEST_CASE(mul) {
    PowExtension<E> a(E(7), 4);
    E b(11);
    std::vector<E> r{E(13), E(17), E(23), E(27)};
    BOOST_TEST(a(r) * b == (a * b)(r));
}

BOOST_AUTO_TEST_CASE(bind) {
    PowExtension<E> pow1(E(4), 3);
    std::vector<E> r1{E(5), E(6), E(7)};
    PowExtension<E> pow2(pow1);
    pow2.bind(E(5));
    std::vector<E> r2{E(6), E(7)};
    PowExtension<E> pow3(pow2);
    pow3.bind(E(6));
    std::vector<E> r3{E(7)};
    BOOST_TEST(pow1(r1) == pow2(r2));
    BOOST_TEST(pow1(r1) == pow3(r3));

    std::vector<E> evaluations(4);
    PowExtension<E> pow = pow1;
    pow.bind(E(-2));
    pow1.bind<E(-2), util::Assign<E>>(evaluations);
    BOOST_TEST(pow() == evaluations);
    pow = pow1;
    pow.bind(E(-1));
    pow1.bind<E(-1), util::Assign<E>>(evaluations);
    BOOST_TEST(pow() == evaluations);
    pow = pow1;
    pow.bind(E(0));
    pow1.bind<E(0), util::Assign<E>>(evaluations);
    BOOST_TEST(pow() == evaluations);
    pow = pow1;
    pow.bind(E(1));
    pow1.bind<E(1), util::Assign<E>>(evaluations);
    BOOST_TEST(pow() == evaluations);
    pow = pow1;
    pow.bind(E(2));
    pow1.bind<E(2), util::Assign<E>>(evaluations);
    BOOST_TEST(pow() == evaluations);
    pow = pow1;
    pow.bind(E(3));
    pow1.bind<E(3), util::Assign<E>>(evaluations);
    BOOST_TEST(pow() == evaluations);
    pow = pow1;
    pow.bind(E(4));
    pow1.bind<E(4), util::Assign<E>>(evaluations);
    BOOST_TEST(pow() == evaluations);

    evaluations = pow2();
    Hypercube<E> hc(pow2.variables());
    for (const auto& [index, b] : std::views::zip(
            std::ranges::subrange(hc.composedBegin(), hc.composedEnd()),
            std::ranges::subrange(hc.decomposedBegin(), hc.decomposedEnd())
        )) {
        BOOST_TEST(pow2(b) == evaluations[index]);
    };
}

BOOST_AUTO_TEST_CASE(homomorphism) {
    PowExtension<E> pow1(E(5), 3);
    std::vector<E> r1{E(11), E(12), E(13)};
    PowExtension<EE> pow2 = pow1.homomorph<EE>();
    std::vector<EE> r2{EE(11), EE(12), EE(13)};
    BOOST_TEST(EE(pow1(r1)) == pow2(r2));
}

BOOST_AUTO_TEST_CASE(circuit) {
    E tau(4);
    constexpr std::size_t ell(3);

    using Circuit = CCSBuilder<E, 2>;
    Circuit circuit;
    auto tau_var = circuit.input();
    PowExtension<E>::circuit<Circuit>::powers<ell>(circuit, tau_var);
    CustomizableConstraintSystem<E> ccs(circuit.ccs());
    Vector<E> z;
    z.elements.reserve(ccs.variables());
    z.elements.emplace_back(E(1));
    z.elements.push_back(tau);
    BOOST_TEST(PowExtension<E>::powers(tau, ell) == PowExtension<E>::trace::powers(tau, ell, z.elements));
    test::circuitry(ccs, z);
}

BOOST_AUTO_TEST_SUITE_END()

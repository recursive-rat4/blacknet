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
#include <ranges>

#include "hypercube.h"
#include "matrix.h"
#include "multilinearextension.h"
#include "polynomialring.h"
#include "solinas62.h"
#include "vector.h"
#include "util.h"

BOOST_AUTO_TEST_SUITE(MultilinearExtensions)

using E = Solinas62Ring;
using EE = Solinas62RingDegree2;

BOOST_AUTO_TEST_CASE(meta) {
    MultilinearExtension mle{E(1), E(2), E(3), E(4), E(5), E(6), E(7), E(8)};
    BOOST_TEST(1 == mle.degree());
    BOOST_TEST(3 == mle.variables());
}

BOOST_AUTO_TEST_CASE(add) {
    MultilinearExtension a{E(1), E(2), E(3), E(4)};
    MultilinearExtension b{E(5), E(6), E(7), E(8)};
    MultilinearExtension c{E(6), E(8), E(10), E(12)};
    BOOST_TEST(c == a + b);
    BOOST_TEST(c == b + a);
}

BOOST_AUTO_TEST_CASE(mul) {
    MultilinearExtension a{E(1), E(2), E(3), E(4)};
    E b(3);
    MultilinearExtension c{E(3), E(6), E(9), E(12)};
    BOOST_TEST(c == a * b);
}

BOOST_AUTO_TEST_CASE(sub) {
    MultilinearExtension a{E(99), E(98), E(97), E(96)};
    E b(3);
    MultilinearExtension c{E(96), E(95), E(94), E(93)};
    BOOST_TEST(c == a - b);
}

BOOST_AUTO_TEST_CASE(bind) {
    MultilinearExtension a{E(1), E(2), E(3), E(4), E(5), E(6), E(7), E(8)};
    MultilinearExtension b{E(1), E(2), E(3), E(4)};
    MultilinearExtension c{E(3), E(4)};
    MultilinearExtension d{E(4)};

    MultilinearExtension mle(a);
    mle.bind(E(0));
    BOOST_TEST(b == mle);
    mle.bind(E(1));
    BOOST_TEST(c == mle);
    mle.bind(E(1));
    BOOST_TEST(d == mle);

    std::vector<E> evaluations(4);
    mle = a;
    mle.bind(E(0));
    a.bind<E(0), util::Assign<E>>(evaluations);
    BOOST_TEST(mle() == evaluations);
    mle = a;
    mle.bind(E(1));
    a.bind<E(1), util::Assign<E>>(evaluations);
    BOOST_TEST(mle() == evaluations);
    mle = a;
    mle.bind(E(2));
    a.bind<E(2), util::Assign<E>>(evaluations);
    BOOST_TEST(mle() == evaluations);
    mle = a;
    mle.bind(E(3));
    a.bind<E(3), util::Assign<E>>(evaluations);
    BOOST_TEST(mle() == evaluations);
    mle = a;
    mle.bind(E(4));
    a.bind<E(4), util::Assign<E>>(evaluations);
    BOOST_TEST(mle() == evaluations);
}

BOOST_AUTO_TEST_CASE(homomorphism) {
    MultilinearExtension<E> mle1({E(14), E(15), E(16), E(17)});
    std::vector<E> r1{E(18), E(19)};
    MultilinearExtension<EE> mle2 = mle1.homomorph<EE>();
    std::vector<EE> r2{EE(18), EE(19)};
    BOOST_TEST(EE(mle1(r1)) == mle2(r2));
}

BOOST_AUTO_TEST_CASE(matrix) {
    Hypercube<E> hc(3);
    Matrix<E> a(2, 4, {
        E(30), E(31), E(32), E(33),
        E(43), E(44), E(45), E(46),
    });
    MultilinearExtension mle(a);
    for (std::tuple<const std::pair<std::size_t, std::size_t>&, const std::vector<E>&> i : std::views::zip(
            std::ranges::subrange(hc.splittedBegin(2, 4), hc.splittedEnd()),
            std::ranges::subrange(hc.decomposedBegin(), hc.decomposedEnd())
        )) {
        const std::size_t& row = std::get<0>(i).first;
        const std::size_t& column = std::get<0>(i).second;
        const std::vector<E>& b = std::get<1>(i);
        BOOST_TEST((a[row, column] == mle(b)));
    };
}

BOOST_AUTO_TEST_CASE(polynomial) {
    using P = Solinas62RingDegree4;
    Hypercube<E> hc(2);
    P a{E(71), E(72), E(73), E(74)};
    MultilinearExtension<E> mle(a);
    for (std::tuple<const std::size_t&, const std::vector<E>&> i : std::views::zip(
            std::ranges::subrange(hc.composedBegin(), hc.composedEnd()),
            std::ranges::subrange(hc.decomposedBegin(), hc.decomposedEnd())
        )) {
        const std::size_t& index = std::get<0>(i);
        const std::vector<E>& b = std::get<1>(i);
        BOOST_TEST(a.coefficients[index] == mle(b));
    };
}

BOOST_AUTO_TEST_CASE(vector) {
    Hypercube<E> hc(3);
    Vector<E> a{E(63), E(64), E(65), E(66), E(67), E(68), E(69), E(70)};
    MultilinearExtension mle(a);
    for (std::tuple<const std::size_t&, const std::vector<E>&> i : std::views::zip(
            std::ranges::subrange(hc.composedBegin(), hc.composedEnd()),
            std::ranges::subrange(hc.decomposedBegin(), hc.decomposedEnd())
        )) {
        const std::size_t& index = std::get<0>(i);
        const std::vector<E>& b = std::get<1>(i);
        BOOST_TEST(a[index] == mle(b));
    };
}

BOOST_AUTO_TEST_CASE(ringvector) {
    Hypercube<E> hc(3);
    Vector<EE> a{
        EE{75, 76},
        EE{77, 78},
        EE{78, 79},
        EE{79, 80},
    };
    MultilinearExtension<E> mle(a);
    for (std::tuple<const std::pair<std::size_t, std::size_t>&, const std::vector<E>&> i : std::views::zip(
            std::ranges::subrange(hc.splittedBegin(4, 2), hc.splittedEnd()),
            std::ranges::subrange(hc.decomposedBegin(), hc.decomposedEnd())
        )) {
        const std::size_t& row = std::get<0>(i).first;
        const std::size_t& column = std::get<0>(i).second;
        const std::vector<E>& b = std::get<1>(i);
        BOOST_TEST(a.elements[row].coefficients[column] == mle(b));
    };
}

BOOST_AUTO_TEST_SUITE_END()

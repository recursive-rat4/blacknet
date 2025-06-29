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

#include "fermat.h"
#include "matrixring.h"
#include "module.h"

using namespace blacknet::crypto;

using Z = FermatRing;
using M = Module<Z, 2>;
using R = MatrixRing<Z, 2>;

BOOST_AUTO_TEST_SUITE(MatrixRings)

BOOST_AUTO_TEST_CASE(Add) {
    const R a{
        Z(1), Z(3),
        Z(1), Z(0),
    };
    const R b{
        Z(0), Z(0),
        Z(7), Z(5),
    };
    const R c{
        Z(1), Z(3),
        Z(8), Z(5),
    };
    BOOST_TEST(c == a + b);
    BOOST_TEST(c == b + a);
}

BOOST_AUTO_TEST_CASE(Mul) {
    const R a{
        Z(1), Z(0),
        Z(2), Z(1),
    };
    const R b{
        Z(1), Z(2),
        Z(2), Z(3),
    };
    const R c{
        Z(1), Z(2),
        Z(4), Z(7),
    };
    const R d{
        Z(5), Z(2),
        Z(8), Z(3),
    };
    BOOST_TEST(c == a * b);
    BOOST_TEST(d == b * a);
}

BOOST_AUTO_TEST_CASE(ModuleProduct) {
    const R a{
        Z(17), Z(18),
        Z(33), Z(34),
    };
    const M b{
        Z(2),
        Z(3),
    };
    const M c{
        Z(88),
        Z(168),
    };
    const M d{
        Z(133),
        Z(138),
    };
    BOOST_TEST(c == a * b);
    BOOST_TEST(d == b * a);
}

BOOST_AUTO_TEST_CASE(Transposition) {
    const R a({
        Z(1), Z(2),
        Z(3), Z(4),
    });
    const R b{{
        Z(1), Z(3),
        Z(2), Z(4),
    }};
    BOOST_TEST(b == a.transpose());
    BOOST_TEST(a == b.transpose());
}

BOOST_AUTO_TEST_CASE(InfinityNorm) {
    const R a({
        Z(0), Z(1),
        Z(2), Z(3),
    });
    Z::NumericType nb = 3;
    Z::NumericType ng = 4;
    BOOST_TEST(!a.checkInfinityNorm(nb));
    BOOST_TEST(a.checkInfinityNorm(ng));
}

BOOST_AUTO_TEST_SUITE_END()

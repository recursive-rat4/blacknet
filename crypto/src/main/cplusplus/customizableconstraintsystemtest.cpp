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
#include "solinas62.h"

using E = Solinas62Ring;

BOOST_AUTO_TEST_SUITE(CustomizableConstraintSystems)

BOOST_AUTO_TEST_CASE(ccs) {
    // Quarte
    Matrix<E> m1(1, 3, { E(0), E(0), E(1) });
    Matrix<E> m2(1, 3, { E(1), E(0), E(0) });
    Vector<E> z1{ E(81), E(1), E(2) };
    Vector<E> z2{ E(16), E(1), E(2) };

    std::vector<Matrix<E>> ms;
    ms.emplace_back(std::move(m1));
    ms.emplace_back(std::move(m2));

    CustomizableConstraintSystem<E> ccs(
        1, 3,
        std::move(ms),
        {{0, 0, 0, 0}, {1}},
        {E(1), E(-1)}
    );
    BOOST_TEST(!ccs.isSatisfied(z1));
    BOOST_TEST(ccs.isSatisfied(z2));
}

BOOST_AUTO_TEST_SUITE_END()

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
#include "pastacurves.h"

BOOST_AUTO_TEST_SUITE(CustomizableConstraintSystems)

BOOST_AUTO_TEST_CASE(r1cs) {
    // x = w²
    Matrix<PallasField> a(1, 3, { PallasField(0), PallasField(0), PallasField(1) });
    Matrix<PallasField> b(1, 3, { PallasField(0), PallasField(0), PallasField(1) });
    Matrix<PallasField> c(1, 3, { PallasField(1), PallasField(0), PallasField(0) });
    Vector<PallasField> z1{ PallasField(9), PallasField(1), PallasField(2) };
    Vector<PallasField> z2{ PallasField(4), PallasField(1), PallasField(2) };

    std::vector<Matrix<PallasField>> m;
    m.emplace_back(std::move(a));
    m.emplace_back(std::move(b));
    m.emplace_back(std::move(c));

    CustomizableConstraintSystem<PallasField> ccs(
        1, 3,
        std::move(m),
        {{0, 1}, {2}},
        {PallasField(1), -PallasField(1)}
    );
    BOOST_TEST(!ccs.isSatisfied(z1));
    BOOST_TEST(ccs.isSatisfied(z2));
}

BOOST_AUTO_TEST_SUITE_END()

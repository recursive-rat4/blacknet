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

#include "customizableconstraintsystem.h"
#include "hypercube.h"
#include "matrixdense.h"
#include "solinas62.h"

using namespace blacknet::crypto;

BOOST_AUTO_TEST_SUITE(CustomizableConstraintSystems)

using E = Solinas62Ring;

BOOST_AUTO_TEST_CASE(ccs) {
    // Quarte
    MatrixDense<E> m1(1, 3, { E(0), E(0), E(1) });
    MatrixDense<E> m2(1, 3, { E(0), E(1), E(0) });
    Vector<E> z{ E(1), E(16), E(2) };

    std::vector<MatrixSparse<E>> ms;
    ms.emplace_back(MatrixSparse(m1));
    ms.emplace_back(MatrixSparse(m2));

    CustomizableConstraintSystem<E> ccs(
        1, 3,
        std::move(ms),
        {{0, 0, 0, 0}, {1}},
        {E(1), E(-1)}
    );
    BOOST_TEST(ccs.isSatisfied(z));

    CustomizableConstraintSystem<E>::Polynomial poly(ccs.polynomial(z));
    BOOST_TEST(4 == poly.degree());
    BOOST_TEST(0 == poly.variables());
    BOOST_TEST(E(0) == Hypercube<E>::sum(poly));
    for (std::size_t i = 1; i < z.size(); ++i) {
        z[i] += E(1);
        poly = ccs.polynomial(z);
        BOOST_TEST(E(0) != Hypercube<E>::sum(poly));
        z[i] -= E(1);
    }
}

BOOST_AUTO_TEST_SUITE_END()

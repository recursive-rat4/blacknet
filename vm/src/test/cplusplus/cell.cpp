/*
 * Copyright (c) 2025 Pavel Vasin
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

#include "cell.h"
#include "circuitbuilder.h"
#include "poseidon2pervushin.h"
#include "r1cs.h"

using namespace blacknet::crypto;
using blacknet::vm::Cell;

BOOST_AUTO_TEST_SUITE(Cells)

BOOST_AUTO_TEST_CASE(plain) {
    using Z = PervushinRing;
    using Cell = Cell<Poseidon2PervushinJive>;
    using Hash = Poseidon2PervushinJive::Hash;

    const Hash hash0{Z(0), Z(0), Z(0), Z(0)};
    const Hash hash1{Z(1), Z(1), Z(1), Z(1)};
    auto cell0 = Cell(hash0);
    auto cell1 = Cell(hash1);
    auto cell2 = Cell::cons(cell1, cell0);
    auto cell3 = cell2.car(cell1, cell0);
    auto cell4 = cell2.cdr(cell1, cell0);

    BOOST_TEST(cell0 == Cell::null());
    BOOST_TEST(cell3 == cell1);
    BOOST_TEST(cell4 == cell0);
    BOOST_CHECK_THROW(cell2.car(cell0, cell1), std::runtime_error);
    BOOST_CHECK_THROW(cell2.cdr(cell0, cell1), std::runtime_error);
}

BOOST_AUTO_TEST_CASE(circuit) {
    using E = PervushinRing;
    using Jive = Poseidon2PervushinJive;
    using Cell = Cell<Jive>;

    using Circuit = CircuitBuilder<E, 2>;
    Circuit circuit;
    using Gadget = Cell::Gadget<Circuit>;
    auto null_gadget = Gadget::null(circuit);
    auto cons_gadget = Gadget::cons(circuit, null_gadget, null_gadget);
    auto car_gadget = cons_gadget.car(null_gadget, null_gadget);
    auto cdr_gadget = cons_gadget.cdr(null_gadget, null_gadget);
    R1CS<E> r1cs(circuit.r1cs());

    Vector<E> z = r1cs.assigment();
    using Tracer = Cell::Tracer<Circuit::degree()>;
    auto null = Cell::null();
    auto cons = Tracer::cons(null, null, z.elements);
    Tracer cons_tracer(cons, z.elements);
    auto car = cons_tracer.car(null, null);
    auto cdr = cons_tracer.cdr(null, null);
    BOOST_TEST(r1cs.isSatisfied(z));

    BOOST_TEST(null == car);
    BOOST_TEST(null == cdr);
}

BOOST_AUTO_TEST_SUITE_END()

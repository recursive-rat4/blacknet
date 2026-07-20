/*
 * Copyright (c) 2026 Pavel Vasin
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

use blacknet_crypto::algebra::IntegerModRing;
use blacknet_crypto::assigner::arithmeticgate::ArithmeticGate as Assigner;
use blacknet_crypto::circuit::arithmeticgate::ArithmeticGate as Circuit;
use blacknet_crypto::circuit::builder::{CircuitBuilder, LinearCombination};
use blacknet_crypto::constraintsystem::ConstraintSystem;
use core::array;
use core::assert_matches;
use core::iter::zip;

type Z = blacknet_crypto::gf2::GF2;

#[test]
fn wrapping_add() {
    let a = [1, 0, 1, 0, 0, 0, 1, 0].map(Z::new);
    let b = [1, 1, 0, 0, 0, 0, 1, 0].map(Z::new);
    let c = [0, 0, 0, 1, 0, 0, 0, 1].map(Z::new);

    let circuit = CircuitBuilder::<Z>::new(2);
    let arithmetic_gate_circuit = Circuit::<Z>::new(&circuit);
    let scope = circuit.scope("test");
    let a_lc: [LinearCombination<Z>; 8] = array::from_fn(|_| scope.public_input().into());
    let b_lc: [LinearCombination<Z>; 8] = array::from_fn(|_| scope.public_input().into());
    let c_lc: [LinearCombination<Z>; 8] = array::from_fn(|_| scope.public_input().into());
    let c_circuit = arithmetic_gate_circuit.wrapping_add(&a_lc, &b_lc);
    for (l, r) in zip(c_circuit, c_lc) {
        scope.constrain(l, r);
    }
    drop(scope);

    let r1cs = circuit.r1cs();
    let z = r1cs.assigment();
    z.extend(a);
    z.extend(b);
    z.extend(c);

    let assigner = Assigner::<Z>::new(&z);
    assigner.wrapping_add(&a, &b);

    assert_matches!(r1cs.is_satisfied(&z.finish()), Ok(()));
}

#[test]
fn rotate_right() {
    let a = [1, 0, 1, 0, 0, 0, 0, 0].map(Z::new);
    let b = 17;
    let c = [0, 1, 0, 0, 0, 0, 0, 1].map(Z::new);

    let circuit = CircuitBuilder::<Z>::new(2);
    let arithmetic_gate_circuit = Circuit::<Z>::new(&circuit);
    let scope = circuit.scope("test");
    let a_lc: [LinearCombination<Z>; 8] = array::from_fn(|_| scope.public_input().into());
    let c_lc: [LinearCombination<Z>; 8] = array::from_fn(|_| scope.public_input().into());
    let c_circuit = arithmetic_gate_circuit.rotate_right(&a_lc, b);
    for (l, r) in zip(c_circuit, c_lc) {
        scope.constrain(l, r);
    }
    drop(scope);

    let r1cs = circuit.r1cs();
    let z = r1cs.assigment();
    z.extend(a);
    z.extend(c);

    let assigner = Assigner::<Z>::new(&z);
    assigner.rotate_right(&a, b);

    assert_matches!(r1cs.is_satisfied(&z.finish()), Ok(()));
}

#[test]
fn bitxor() {
    let a = [0, 0, 1, 1].map(Z::new);
    let b = [0, 1, 0, 1].map(Z::new);
    let c = [0, 1, 1, 0].map(Z::new);

    let circuit = CircuitBuilder::<Z>::new(2);
    let arithmetic_gate_circuit = Circuit::<Z>::new(&circuit);
    let scope = circuit.scope("test");
    let a_lc: [LinearCombination<Z>; 4] = array::from_fn(|_| scope.public_input().into());
    let b_lc: [LinearCombination<Z>; 4] = array::from_fn(|_| scope.public_input().into());
    let c_lc: [LinearCombination<Z>; 4] = array::from_fn(|_| scope.public_input().into());
    let c_circuit = arithmetic_gate_circuit.bitxor(&a_lc, &b_lc);
    for (l, r) in zip(c_circuit, c_lc) {
        scope.constrain(l, r);
    }
    drop(scope);

    let r1cs = circuit.r1cs();
    let z = r1cs.assigment();
    z.extend(a);
    z.extend(b);
    z.extend(c);

    let assigner = Assigner::<Z>::new(&z);
    assigner.bitxor(&a, &b);

    assert_matches!(r1cs.is_satisfied(&z.finish()), Ok(()));
}

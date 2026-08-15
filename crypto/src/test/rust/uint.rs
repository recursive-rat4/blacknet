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
use blacknet_crypto::assigner::uint::UInt as Assigner;
use blacknet_crypto::circuit::builder::{CircuitBuilder, VariableKind};
use blacknet_crypto::circuit::uint::UInt as Circuit;
use blacknet_crypto::constraintsystem::ConstraintSystem;
use core::assert_matches;
use core::iter::zip;

type Z = blacknet_crypto::gf2::GF2;

#[test]
fn fused_add() {
    let a = [1, 0, 1, 0, 1, 1, 0, 0].map(Z::with_int);
    let b = [0, 0, 1, 1, 1, 0, 1, 1].map(Z::with_int);
    let c = [1, 1, 1, 0, 0, 1, 1, 0].map(Z::with_int);
    let d = [0, 0, 0, 1, 1, 1, 1, 0].map(Z::with_int);

    let circuit = CircuitBuilder::<Z>::r1cs();
    let scope = circuit.scope("test");
    let a_input = Circuit::<Z, 8>::allocate(&circuit, VariableKind::Public);
    let b_input = Circuit::<Z, 8>::allocate(&circuit, VariableKind::Public);
    let c_input = Circuit::<Z, 8>::allocate(&circuit, VariableKind::Public);
    let d_input = Circuit::<Z, 8>::allocate(&circuit, VariableKind::Public);
    circuit.lay_out();
    let d_output = a_input.fused_add(&b_input, &c_input);
    for (l, r) in zip(d_output, d_input) {
        scope.constrain(l, r);
    }
    drop(scope);

    let r1cs = circuit.to_r1cs();
    let z = r1cs.assigment();
    z.extend(a);
    z.extend(b);
    z.extend(c);
    z.extend(d);

    let a_assigner = Assigner::<Z, 8>::new(a, &z);
    let b_assigner = Assigner::<Z, 8>::new(b, &z);
    let c_assigner = Assigner::<Z, 8>::new(c, &z);
    a_assigner.fused_add(&b_assigner, &c_assigner);

    assert_matches!(r1cs.is_satisfied(&z.finish()), Ok(()));
}

#[test]
fn wrapping_add() {
    let a = [1, 0, 1, 0, 0, 0, 1, 0].map(Z::with_int);
    let b = [1, 1, 0, 0, 0, 0, 1, 0].map(Z::with_int);
    let c = [0, 0, 0, 1, 0, 0, 0, 1].map(Z::with_int);

    let circuit = CircuitBuilder::<Z>::r1cs();
    let scope = circuit.scope("test");
    let a_input = Circuit::<Z, 8>::allocate(&circuit, VariableKind::Public);
    let b_input = Circuit::<Z, 8>::allocate(&circuit, VariableKind::Public);
    let c_input = Circuit::<Z, 8>::allocate(&circuit, VariableKind::Public);
    circuit.lay_out();
    let c_output = a_input.wrapping_add(&b_input);
    for (l, r) in zip(c_output, c_input) {
        scope.constrain(l, r);
    }
    drop(scope);

    let r1cs = circuit.to_r1cs();
    let z = r1cs.assigment();
    z.extend(a);
    z.extend(b);
    z.extend(c);

    let a_assigner = Assigner::<Z, 8>::new(a, &z);
    let b_assigner = Assigner::<Z, 8>::new(b, &z);
    a_assigner.wrapping_add(&b_assigner);

    assert_matches!(r1cs.is_satisfied(&z.finish()), Ok(()));
}

#[test]
fn rotate_right() {
    let a = [1, 0, 1, 0, 0, 0, 0, 0].map(Z::with_int);
    let b = 17;
    let c = [0, 1, 0, 0, 0, 0, 0, 1].map(Z::with_int);

    let circuit = CircuitBuilder::<Z>::r1cs();
    let scope = circuit.scope("test");
    let a_input = Circuit::<Z, 8>::allocate(&circuit, VariableKind::Public);
    let c_input = Circuit::<Z, 8>::allocate(&circuit, VariableKind::Public);
    circuit.lay_out();
    let c_output = a_input.rotate_right(b);
    for (l, r) in zip(c_output, c_input) {
        scope.constrain(l, r);
    }
    drop(scope);

    let r1cs = circuit.to_r1cs();
    let z = r1cs.assigment();
    z.extend(a);
    z.extend(c);

    let a_assigner = Assigner::<Z, 8>::new(a, &z);
    a_assigner.rotate_right(b);

    assert_matches!(r1cs.is_satisfied(&z.finish()), Ok(()));
}

#[test]
fn bitxor() {
    let a = [0, 0, 1, 1].map(Z::with_int);
    let b = [0, 1, 0, 1].map(Z::with_int);
    let c = [0, 1, 1, 0].map(Z::with_int);

    let circuit = CircuitBuilder::<Z>::r1cs();
    let scope = circuit.scope("test");
    let a_input = Circuit::<Z, 4>::allocate(&circuit, VariableKind::Public);
    let b_input = Circuit::<Z, 4>::allocate(&circuit, VariableKind::Public);
    let c_input = Circuit::<Z, 4>::allocate(&circuit, VariableKind::Public);
    circuit.lay_out();
    let c_output = a_input.bitxor(&b_input);
    for (l, r) in zip(c_output, c_input) {
        scope.constrain(l, r);
    }
    drop(scope);

    let r1cs = circuit.to_r1cs();
    let z = r1cs.assigment();
    z.extend(a);
    z.extend(b);
    z.extend(c);

    let a_assigner = Assigner::<Z, 4>::new(a, &z);
    let b_assigner = Assigner::<Z, 4>::new(b, &z);
    a_assigner.bitxor(&b_assigner);

    assert_matches!(r1cs.is_satisfied(&z.finish()), Ok(()));
}

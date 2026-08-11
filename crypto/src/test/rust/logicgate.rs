/*
 * Copyright (c) 2025-2026 Pavel Vasin
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

use blacknet_crypto::assigner::logicgate::LogicGate as Assigner;
use blacknet_crypto::circuit::builder::{CircuitBuilder, Variable};
use blacknet_crypto::circuit::logicgate::LogicGate as Circuit;
use blacknet_crypto::constraintsystem::ConstraintSystem;
use core::{array, assert_matches};

type R = blacknet_crypto::uring::U32Ring;

#[test]
fn xor() {
    let truth: [(R, R, R); 4] = [
        [0, 0, 0].map(R::from).into(),
        [0, 1, 1].map(R::from).into(),
        [1, 0, 1].map(R::from).into(),
        [1, 1, 0].map(R::from).into(),
    ];

    let circuit = CircuitBuilder::<R>::r1cs();
    let scope = circuit.scope("test");
    let logic_gate_circuit = Circuit::<R>::new(&circuit);
    let inputs: [(Variable<R>, Variable<R>, Variable<R>); 4] =
        array::from_fn(|_| (scope.public(), scope.public(), scope.public()));
    circuit.lay_out();
    for (a, b, c) in inputs {
        let c_circuit = logic_gate_circuit.xor(&a.into(), &b.into());
        scope.constrain(c_circuit, c);
    }
    drop(scope);

    let r1cs = circuit.to_r1cs();
    let z = r1cs.assigment();
    for (a, b, c) in truth {
        z.push(a);
        z.push(b);
        z.push(c);
    }

    let assigner = Assigner::<R>::new(&z);
    for (a, b, _) in truth {
        assigner.xor(a, b);
    }

    assert_matches!(r1cs.is_satisfied(&z.finish()), Ok(()));
}

#[test]
fn and() {
    let truth: [(R, R, R); 4] = [
        [0, 0, 0].map(R::from).into(),
        [0, 1, 0].map(R::from).into(),
        [1, 0, 0].map(R::from).into(),
        [1, 1, 1].map(R::from).into(),
    ];

    let circuit = CircuitBuilder::<R>::r1cs();
    let scope = circuit.scope("test");
    let logic_gate_circuit = Circuit::<R>::new(&circuit);
    let inputs: [(Variable<R>, Variable<R>, Variable<R>); 4] =
        array::from_fn(|_| (scope.public(), scope.public(), scope.public()));
    circuit.lay_out();
    for (a, b, c) in inputs {
        let c_circuit = logic_gate_circuit.and(&a.into(), &b.into());
        scope.constrain(c_circuit, c);
    }
    drop(scope);

    let r1cs = circuit.to_r1cs();
    let z = r1cs.assigment();
    for (a, b, c) in truth {
        z.push(a);
        z.push(b);
        z.push(c);
    }

    let assigner = Assigner::<R>::new(&z);
    for (a, b, _) in truth {
        assigner.and(a, b);
    }

    assert_matches!(r1cs.is_satisfied(&z.finish()), Ok(()));
}

#[test]
fn or() {
    let truth: [(R, R, R); 4] = [
        [0, 0, 0].map(R::from).into(),
        [0, 1, 1].map(R::from).into(),
        [1, 0, 1].map(R::from).into(),
        [1, 1, 1].map(R::from).into(),
    ];

    let circuit = CircuitBuilder::<R>::r1cs();
    let scope = circuit.scope("test");
    let logic_gate_circuit = Circuit::<R>::new(&circuit);
    let inputs: [(Variable<R>, Variable<R>, Variable<R>); 4] =
        array::from_fn(|_| (scope.public(), scope.public(), scope.public()));
    circuit.lay_out();
    for (a, b, c) in inputs {
        let c_circuit = logic_gate_circuit.or(&a.into(), &b.into());
        scope.constrain(c_circuit, c);
    }
    drop(scope);

    let r1cs = circuit.to_r1cs();
    let z = r1cs.assigment();
    for (a, b, c) in truth {
        z.push(a);
        z.push(b);
        z.push(c);
    }

    let assigner = Assigner::<R>::new(&z);
    for (a, b, _) in truth {
        assigner.or(a, b);
    }

    assert_matches!(r1cs.is_satisfied(&z.finish()), Ok(()));
}

#[test]
fn not() {
    #[rustfmt::skip]
    let truth: [(R, R); 2] = [
        [0, 1].map(R::from).into(),
        [1, 0].map(R::from).into(),
    ];

    let circuit = CircuitBuilder::<R>::r1cs();
    let scope = circuit.scope("test");
    let logic_gate_circuit = Circuit::<R>::new(&circuit);
    let inputs: [(Variable<R>, Variable<R>); 2] =
        array::from_fn(|_| (scope.public(), scope.public()));
    circuit.lay_out();
    for (a, b) in inputs {
        let b_circuit = logic_gate_circuit.not(&a.into());
        scope.constrain(b_circuit, b);
    }
    drop(scope);

    let r1cs = circuit.to_r1cs();
    let z = r1cs.assigment();
    for (a, b) in truth {
        z.push(a);
        z.push(b);
    }

    let assigner = Assigner::<R>::new(&z);
    for (a, _) in truth {
        assigner.not(a);
    }

    assert_matches!(r1cs.is_satisfied(&z.finish()), Ok(()));
}

#[test]
fn check_less_or_equal() {
    let a = [0, 1, 0, 0].map(R::from);
    let b = [0, 0, 1, 0].map(R::from);

    let circuit = CircuitBuilder::<R>::r1cs();
    let logic_gate_circuit = Circuit::<R>::new(&circuit);
    let scope = circuit.scope("test");
    let a_circuit = (0..a.len())
        .map(|_| scope.public())
        .map(From::from)
        .collect::<Vec<_>>();
    circuit.lay_out();
    logic_gate_circuit.check_less_or_equal(&a_circuit, &b);
    drop(scope);

    let r1cs = circuit.to_r1cs();
    let z = r1cs.assigment();
    z.extend(a);

    let assigner = Assigner::<R>::new(&z);
    assigner.check_less_or_equal(&a, &b);

    assert_matches!(r1cs.is_satisfied(&z.finish()), Ok(()));
}

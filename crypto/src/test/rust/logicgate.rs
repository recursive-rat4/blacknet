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

use blacknet_crypto::assigner::logicgate::LogicGate as Assigner;
use blacknet_crypto::circuit::circuitbuilder::CircuitBuilder;
use blacknet_crypto::circuit::logicgate::LogicGate as Circuit;

type R = blacknet_crypto::pervushin::PervushinField;

#[test]
fn xor() {
    let truth: [(R, R, R); 4] = [
        [0, 0, 0].map(R::from).into(),
        [0, 1, 1].map(R::from).into(),
        [1, 0, 1].map(R::from).into(),
        [1, 1, 0].map(R::from).into(),
    ];

    let circuit = CircuitBuilder::<R>::new(2);
    let logic_gate_circuit = Circuit::<R>::new(&circuit);
    for _ in 0..truth.len() {
        let scope = circuit.scope("test");
        let a_var = scope.public_input();
        let b_var = scope.public_input();
        let c_var = scope.public_input();
        let c_circuit = logic_gate_circuit.xor(&a_var.into(), &b_var.into());
        scope.constrain(c_circuit, c_var);
    }

    let r1cs = circuit.r1cs();
    let z = r1cs.assigment();
    for (a, b, c) in truth {
        z.push(a);
        z.push(b);
        z.push(c);
    }

    let assigner = Assigner::new(&z);
    for (a, b, _) in truth {
        assigner.xor(a, b);
    }

    assert!(r1cs.is_satisfied(&z.finish()));
}

#[test]
fn and() {
    let truth: [(R, R, R); 4] = [
        [0, 0, 0].map(R::from).into(),
        [0, 1, 0].map(R::from).into(),
        [1, 0, 0].map(R::from).into(),
        [1, 1, 1].map(R::from).into(),
    ];

    let circuit = CircuitBuilder::<R>::new(2);
    let logic_gate_circuit = Circuit::<R>::new(&circuit);
    for _ in 0..truth.len() {
        let scope = circuit.scope("test");
        let a_var = scope.public_input();
        let b_var = scope.public_input();
        let c_var = scope.public_input();
        let c_circuit = logic_gate_circuit.and(&a_var.into(), &b_var.into());
        scope.constrain(c_circuit, c_var);
    }

    let r1cs = circuit.r1cs();
    let z = r1cs.assigment();
    for (a, b, c) in truth {
        z.push(a);
        z.push(b);
        z.push(c);
    }

    let assigner = Assigner::new(&z);
    for (a, b, _) in truth {
        assigner.and(a, b);
    }

    assert!(r1cs.is_satisfied(&z.finish()));
}

#[test]
fn or() {
    let truth: [(R, R, R); 4] = [
        [0, 0, 0].map(R::from).into(),
        [0, 1, 1].map(R::from).into(),
        [1, 0, 1].map(R::from).into(),
        [1, 1, 1].map(R::from).into(),
    ];

    let circuit = CircuitBuilder::<R>::new(2);
    let logic_gate_circuit = Circuit::<R>::new(&circuit);
    for _ in 0..truth.len() {
        let scope = circuit.scope("test");
        let a_var = scope.public_input();
        let b_var = scope.public_input();
        let c_var = scope.public_input();
        let c_circuit = logic_gate_circuit.or(&a_var.into(), &b_var.into());
        scope.constrain(c_circuit, c_var);
    }

    let r1cs = circuit.r1cs();
    let z = r1cs.assigment();
    for (a, b, c) in truth {
        z.push(a);
        z.push(b);
        z.push(c);
    }

    let assigner = Assigner::new(&z);
    for (a, b, _) in truth {
        assigner.or(a, b);
    }

    assert!(r1cs.is_satisfied(&z.finish()));
}

#[test]
fn not() {
    #[rustfmt::skip]
    let truth: [(R, R); 2] = [
        [0, 1].map(R::from).into(),
        [1, 0].map(R::from).into(),
    ];

    let circuit = CircuitBuilder::<R>::new(2);
    let logic_gate_circuit = Circuit::<R>::new(&circuit);
    for _ in 0..truth.len() {
        let scope = circuit.scope("test");
        let a_var = scope.public_input();
        let b_var = scope.public_input();
        let b_circuit = logic_gate_circuit.not(&a_var.into());
        scope.constrain(b_circuit, b_var);
    }

    let r1cs = circuit.r1cs();
    let z = r1cs.assigment();
    for (a, b) in truth {
        z.push(a);
        z.push(b);
    }

    let assigner = Assigner::new(&z);
    for (a, _) in truth {
        assigner.not(a);
    }

    assert!(r1cs.is_satisfied(&z.finish()));
}

#[test]
fn check_less_or_equal() {
    let a = [0, 1, 0, 0].map(R::from);
    let b = [0, 0, 1, 0].map(R::from);

    let circuit = CircuitBuilder::<R>::new(2);
    let logic_gate_circuit = Circuit::<R>::new(&circuit);
    let scope = circuit.scope("test");
    let a_circuit = (0..a.len())
        .map(|_| scope.public_input())
        .map(From::from)
        .collect::<Vec<_>>();
    logic_gate_circuit.check_less_or_equal(&a_circuit, &b);
    drop(scope);

    let r1cs = circuit.r1cs();
    let z = r1cs.assigment();
    z.extend(a);

    let assigner = Assigner::new(&z);
    assigner.check_less_or_equal(&a, &b);

    assert!(r1cs.is_satisfied(&z.finish()));
}

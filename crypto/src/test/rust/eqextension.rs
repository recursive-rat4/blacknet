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

use blacknet_compat::assert_ok;
use blacknet_crypto::assigner::eqextension::EqExtension as Assigner;
use blacknet_crypto::circuit::circuitbuilder::{CircuitBuilder, VariableKind};
use blacknet_crypto::circuit::eqextension::EqExtension as Circuit;
use blacknet_crypto::circuit::point::Point as PointCircuit;
use blacknet_crypto::constraintsystem::ConstraintSystem;
use blacknet_crypto::eqextension::EqExtension;
use blacknet_crypto::hypercube::Hypercube;
use blacknet_crypto::point::Point;
use blacknet_crypto::polynomial::Polynomial;
use blacknet_crypto::ring::{Ring, UnitalRing};

type R = blacknet_crypto::pervushin::PervushinField;

#[test]
fn meta() {
    let eq = EqExtension::from([1, 0, 0].map(R::from));
    assert_eq!(eq.degree(), 1);
    assert_eq!(eq.variables(), 3);
}

#[test]
fn mul() {
    let a = EqExtension::from([2, 3, 5, 7].map(R::from));
    let b = R::from(11);
    let r = Point::from([13, 17, 23, 27].map(R::from));
    assert_eq!(a.point(&r) * b, (a * b).point(&r));
}

#[test]
fn bind() {
    let eq1 = EqExtension::from([2, 3, 4].map(R::from));
    let r1 = Point::from([5, 6, 7].map(R::from));
    let mut eq2 = eq1.clone();
    eq2.bind(R::from(5));
    let r2 = Point::from([6, 7].map(R::from));
    let mut eq3 = eq2.clone();
    eq3.bind(R::from(6));
    let r3 = Point::from([7].map(R::from));
    assert_eq!(eq2.point(&r2), eq1.point(&r1));
    assert_eq!(eq3.point(&r3), eq1.point(&r1));

    let mut eq = eq1.clone();
    eq.bind(R::from(-2));
    let evaluations = eq1.hypercube_with_var::<-2>();
    assert_eq!(evaluations, eq.hypercube());
    eq = eq1.clone();
    eq.bind(R::from(-1));
    let evaluations = eq1.hypercube_with_var::<-1>();
    assert_eq!(evaluations, eq.hypercube());
    eq = eq1.clone();
    eq.bind(R::from(0));
    let evaluations = eq1.hypercube_with_var::<0>();
    assert_eq!(evaluations, eq.hypercube());
    eq = eq1.clone();
    eq.bind(R::from(1));
    let evaluations = eq1.hypercube_with_var::<1>();
    assert_eq!(evaluations, eq.hypercube());
    eq = eq1.clone();
    eq.bind(R::from(2));
    let evaluations = eq1.hypercube_with_var::<2>();
    assert_eq!(evaluations, eq.hypercube());
    eq = eq1.clone();
    eq.bind(R::from(3));
    let evaluations = eq1.hypercube_with_var::<3>();
    assert_eq!(evaluations, eq.hypercube());
    eq = eq1.clone();
    eq.bind(R::from(4));
    let evaluations = eq1.hypercube_with_var::<4>();
    assert_eq!(evaluations, eq.hypercube());
}

#[test]
fn point() {
    let hc = Hypercube::<R>::new(3);
    let a = Point::<R>::from([1, 0, 0].map(R::from));
    let eq = EqExtension::<R>::from(a.coordinates().clone());
    hc.iter_vertex().for_each(|b| {
        if a == b {
            assert_eq!(eq.point(&b), R::UNITY);
        } else {
            assert_eq!(eq.point(&b), R::ZERO);
        }
    });
}

#[test]
fn hypercube() {
    let hc = Hypercube::<R>::new(3);
    let a = Vec::<R>::from([1, 0, 0].map(R::from));
    let eq = EqExtension::<R>::from(a);
    let pis = eq.hypercube();
    hc.iter_index().for_each(|i| {
        if i == 4 {
            assert_eq!(pis[i], R::UNITY);
        } else {
            assert_eq!(pis[i], R::ZERO);
        }
    });
}

#[test]
fn circuit_point() {
    let coefficients_plain = [2, 3, 5].map(R::from);
    let eq_plain = EqExtension::<R>::from(coefficients_plain);
    let x_plain = Point::<R>::from([7, 11, 13].map(R::from));
    let y_plain = eq_plain.point(&x_plain);

    let circuit = CircuitBuilder::<R>::new(2);
    let scope = circuit.scope("test");
    let eq_circuit = Circuit::allocate(&circuit, VariableKind::PublicInput, eq_plain.variables());
    let x_circuit =
        PointCircuit::allocate(&circuit, VariableKind::PublicInput, x_plain.dimension());
    let _y_circuit = eq_circuit.point(&x_circuit);
    drop(scope);

    let r1cs = circuit.r1cs();
    let z = r1cs.assigment();
    z.extend(coefficients_plain);
    z.extend_from_slice(&x_plain);

    let eq_assigner = Assigner::new(coefficients_plain.into(), &z);
    let y_assigned = eq_assigner.point(&x_plain);

    assert_eq!(y_assigned, y_plain);
    assert_ok!(r1cs.is_satisfied(&z.finish()));
}

#[test]
fn circuit_hypercube() {
    let coefficients_plain = [29, 31, 37].map(R::from);
    let eq_plain = EqExtension::<R>::from(coefficients_plain);
    let y_plain = eq_plain.hypercube();

    let circuit = CircuitBuilder::<R>::new(2);
    let scope = circuit.scope("test");
    let eq_circuit = Circuit::allocate(&circuit, VariableKind::PublicInput, eq_plain.variables());
    let _y_circuit = eq_circuit.hypercube();
    drop(scope);

    let r1cs = circuit.r1cs();
    let z = r1cs.assigment();
    z.extend(coefficients_plain);

    let eq_assigner = Assigner::new(coefficients_plain.into(), &z);
    let y_assigned = eq_assigner.hypercube();

    assert_eq!(y_assigned, y_plain);
    assert_ok!(r1cs.is_satisfied(&z.finish()));
}

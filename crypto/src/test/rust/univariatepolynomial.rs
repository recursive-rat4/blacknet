/*
 * Copyright (c) 2024-2026 Pavel Vasin
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
use blacknet_crypto::algebra::{Double, Square, Zero};
use blacknet_crypto::assigner::polynomial::UnivariatePolynomial as Assigner;
use blacknet_crypto::circuit::builder::{CircuitBuilder, VariableKind};
use blacknet_crypto::circuit::polynomial::UnivariatePolynomial as Circuit;
use blacknet_crypto::constraintsystem::ConstraintSystem;
use blacknet_crypto::matrix::DenseVector;
use blacknet_crypto::polynomial::{InBasis, TensorBasis, UnivariatePolynomial};

type R = blacknet_crypto::pervushin::PervushinField;

#[test]
fn polynomial() {
    let p = UnivariatePolynomial::from([2, 3, 4, 5].map(R::from));
    assert_eq!(p.degree(), 3);
    assert_eq!(p.variables(), 1);
}

#[test]
fn add() {
    let a = UnivariatePolynomial::from([1, 2, 3, 4].map(R::from));
    let b = UnivariatePolynomial::from([5, 6, 1, 5].map(R::from));
    let c = UnivariatePolynomial::from([6, 8, 4, 9].map(R::from));
    assert_eq!(a + b, c);
}

#[test]
fn dbl() {
    let a = UnivariatePolynomial::from([1, 2, 3, 4].map(R::from));
    let b = UnivariatePolynomial::from([2, 4, 6, 8].map(R::from));
    assert_eq!(a.double(), b);
}

#[test]
fn mul() {
    let a = UnivariatePolynomial::from([2, 3].map(R::from));
    let b = UnivariatePolynomial::from([5, 7, 11].map(R::from));
    let c = UnivariatePolynomial::from([10, 29, 43, 33].map(R::from));
    assert_eq!(a * b, c);
}

#[test]
fn sqr() {
    let a = UnivariatePolynomial::from([2, 3, 5].map(R::from));
    let b = UnivariatePolynomial::from([4, 12, 29, 30, 25].map(R::from));
    assert_eq!(a.square(), b);
}

#[test]
fn evaluate() {
    let a = UnivariatePolynomial::from([2, 3, 4, 5].map(R::from));
    let b = UnivariatePolynomial::from([2, 3, 4].map(R::from));
    let c = UnivariatePolynomial::from([2, 3].map(R::from));
    let d = UnivariatePolynomial::from([2].map(R::from));
    let e = UnivariatePolynomial::<R>::default();

    assert_eq!(a.evaluate(R::from(4)), R::from(398));
    assert_eq!(b.evaluate(R::from(4)), R::from(78));
    assert_eq!(c.evaluate(R::from(4)), R::from(14));
    assert_eq!(d.evaluate(R::from(4)), R::from(2));
    assert_eq!(e.evaluate(R::from(4)), R::ZERO);

    assert_eq!(a.at_0_plus_1(), R::from(16));
    assert_eq!(b.at_0_plus_1(), R::from(11));
    assert_eq!(c.at_0_plus_1(), R::from(7));
    assert_eq!(d.at_0_plus_1(), R::from(4));
    assert_eq!(e.at_0_plus_1(), R::ZERO);
}

#[test]
fn basis() {
    let point = R::from(4);
    let p0 = UnivariatePolynomial::<R>::default();
    let p1 = UnivariatePolynomial::from([2].map(R::from));
    let p2 = UnivariatePolynomial::from([2, 3].map(R::from));
    let p3 = UnivariatePolynomial::from([2, 3, 4].map(R::from));
    let b0 = DenseVector::<R>::default();
    let b1 = DenseVector::from([1].map(R::from));
    let b2 = DenseVector::from([1, 4].map(R::from));
    let b3 = DenseVector::from([1, 4, 16].map(R::from));

    assert_eq!(p0.basis(&point), b0);
    assert_eq!(p1.basis(&point), b1);
    assert_eq!(p2.basis(&point), b2);
    assert_eq!(p3.basis(&point), b3);
}

#[test]
fn tensor_basis() {
    let point = R::from(2);
    let p = UnivariatePolynomial::from([1, 1, 1, 1].map(R::from));
    let l = DenseVector::from([1, 4].map(R::from));
    let r = DenseVector::from([1, 2].map(R::from));
    let (left, right) = p.tensor_basis(&point);

    assert_eq!(left, l);
    assert_eq!(right, r);
}

#[test]
fn circuit_evaluate() {
    let p_plain = UnivariatePolynomial::from([2, 3, 4, 5, 6].map(R::from));
    let x_plain = R::from(7);
    let y_plain = p_plain.evaluate(x_plain);

    let circuit = CircuitBuilder::<R>::new(2);
    let scope = circuit.scope("test");
    let p_circuit = Circuit::<R>::allocate(&circuit, VariableKind::PublicInput, p_plain.degree());
    let x_circuit = scope.public_input();
    let _y_circuit = p_circuit.evaluate(&x_circuit.into());
    drop(scope);

    let r1cs = circuit.r1cs();
    let z = r1cs.assigment();
    z.extend_from_slice(&p_plain);
    z.push(x_plain);

    let p_assigner = Assigner::new(p_plain.into(), &z);
    let y_assigned = p_assigner.evaluate(x_plain);

    assert_eq!(y_assigned, y_plain);
    assert_ok!(r1cs.is_satisfied(&z.finish()));
}

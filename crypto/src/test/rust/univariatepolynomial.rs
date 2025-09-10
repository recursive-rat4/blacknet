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

use blacknet_crypto::assigner::univariatepolynomial::UnivariatePolynomial as Assigner;
use blacknet_crypto::circuit::circuitbuilder::{CircuitBuilder, VariableKind};
use blacknet_crypto::circuit::univariatepolynomial::UnivariatePolynomial as Circuit;
use blacknet_crypto::constraintsystem::ConstraintSystem;
use blacknet_crypto::univariatepolynomial::UnivariatePolynomial;

type R = blacknet_crypto::pervushin::PervushinField;

#[test]
fn polynomial() {
    let p = UnivariatePolynomial::from([2, 3, 4, 5].map(R::from));
    assert_eq!(p.degree(), 3);
    assert_eq!(p.variables(), 1);
}

#[test]
fn plain_evaluate() {
    let a = UnivariatePolynomial::from([2, 3, 4, 5].map(R::from));
    let b = UnivariatePolynomial::from([2, 3, 4].map(R::from));
    let c = UnivariatePolynomial::from([2, 3].map(R::from));
    let d = UnivariatePolynomial::from([2].map(R::from));

    assert_eq!(a.evaluate(R::from(4)), R::from(398));
    assert_eq!(b.evaluate(R::from(4)), R::from(78));
    assert_eq!(c.evaluate(R::from(4)), R::from(14));
    assert_eq!(d.evaluate(R::from(4)), R::from(2));

    assert_eq!(a.at_0_plus_1(), R::from(16));
    assert_eq!(b.at_0_plus_1(), R::from(11));
    assert_eq!(c.at_0_plus_1(), R::from(7));
    assert_eq!(d.at_0_plus_1(), R::from(4));
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

    let p_assigner = Assigner::new(p_plain.steal(), &z);
    let y_assigned = p_assigner.evaluate(x_plain);

    assert_eq!(y_assigned, y_plain);
    assert_eq!(r1cs.is_satisfied(&z.finish()), Ok(()));
}

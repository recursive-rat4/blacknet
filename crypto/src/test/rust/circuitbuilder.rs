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

use blacknet_crypto::circuitbuilder::{CircuitBuilder, Constant};
use blacknet_crypto::customizableconstraintsystem::CustomizableConstraintSystem;
use blacknet_crypto::matrixdense::MatrixDense;
use blacknet_crypto::matrixsparse::MatrixSparse;
use blacknet_crypto::r1cs::R1CS;
use blacknet_crypto::vectordense::VectorDense;

type R = blacknet_crypto::field25519::Field25519;

#[test]
fn scopelism() {
    let circuit = CircuitBuilder::<R>::new(1);
    {
        let scope = circuit.scope("scopelism");
        let a = scope.public_input();
        {
            let scope = circuit.scope("comparatism");
            let b = scope.public_output();
            {
                let scope = circuit.scope("additivism");
                scope.constrain(a, b);
            }
            {
                let scope = circuit.scope("multiplism");
                scope.constrain(b, a);
            }
        }
        {
            let scope = circuit.scope("expressionism");
            let c = scope.private_input();
            let d = {
                let scope = circuit.scope("cubism");
                scope.private_output()
            };
            {
                let scope = circuit.scope("algebraism");
                scope.constrain(c, d);
            }
        }
    }
    assert_eq!(
        circuit.to_string(),
        "Circuit degree 1 constraints 3 variables 5
Root 0x1
└──scopelism 0x1
   ├──comparatism 0x1
   │  ├──additivism 1x0
   │  └──multiplism 1x0
   └──expressionism 0x1
      ├──cubism 0x1
      └──algebraism 1x0
"
    );
}

#[test]
fn comparatism() {
    #[rustfmt::skip]
    let am = MatrixDense::new(4, 4, [
        0, 1, 0, 0,
        0, 0, 0, 1,
        0, 0, 0, 1,
        4, 0, 0, 0,
    ].map(R::from).into());
    #[rustfmt::skip]
    let bm = MatrixDense::new(4, 4, [
        1, 0, 0, 0,
        1, 0, 0, 0,
        1, 0, 0, 0,
        1, 0, 0, 0,
    ].map(R::from).into());
    #[rustfmt::skip]
    let cm = MatrixDense::new(4, 4, [
        0, 0, 0, 1,
        0, 0, 1, 0,
        4, 0, 0, 0,
        0, 0, 0, 1,
    ].map(R::from).into());
    let r1cs = R1CS::new(
        MatrixSparse::from(&am),
        MatrixSparse::from(&bm),
        MatrixSparse::from(&cm),
    );

    let circuit = CircuitBuilder::<R>::new(1);

    let scope = circuit.scope("equalism");
    let c = Constant::from(R::from(4));
    let x = scope.public_input();
    let y = scope.public_output();
    let w = scope.private_input();

    scope.constrain(x, w);
    scope.constrain(w, y);
    scope.constrain(w, c);
    scope.constrain(c, w);

    drop(scope);
    assert_eq!(circuit.r1cs(), r1cs);

    let z = VectorDense::from([1, 4, 4, 4].map(R::from));
    assert!(r1cs.is_satisfied(&z));
}

#[test]
fn additivism() {
    #[rustfmt::skip]
    let am = MatrixDense::new(5, 4, [
        0, 0, 0, 2,
        0, 0, 2, 0,
        0, 0, 2, 0,
        4, 0, 0, 1,
        4, 0, 0, 1,
    ].map(R::from).into());
    #[rustfmt::skip]
    let bm = MatrixDense::new(5, 4, [
        1, 0, 0, 0,
        1, 0, 0, 0,
        1, 0, 0, 0,
        1, 0, 0, 0,
        1, 0, 0, 0,
    ].map(R::from).into());
    #[rustfmt::skip]
    let cm = MatrixDense::new(5, 4, [
        0, 1, 0, 0,
        0, 0, 0, 1,
        4, 0, 0, 0,
        0, 1, 0, 0,
        0, 1, 0, 0,
    ].map(R::from).into());
    let r1cs = R1CS::new(
        MatrixSparse::from(&am),
        MatrixSparse::from(&bm),
        MatrixSparse::from(&cm),
    );

    let circuit = CircuitBuilder::<R>::new(1);

    let scope = circuit.scope("additivism");
    let c = Constant::from(R::from(4));
    let x = scope.public_input();
    let y = scope.public_input();
    let w = scope.private_output();

    scope.constrain(w + w, x);
    scope.constrain(y + y, w);
    scope.constrain(y + y, c);
    scope.constrain(w + c, x);
    scope.constrain(c + w, x);

    drop(scope);
    assert_eq!(circuit.r1cs(), r1cs);

    let z = VectorDense::from([1, 8, 2, 4].map(R::from));
    assert!(r1cs.is_satisfied(&z));
}

#[test]
fn multiplism() {
    #[rustfmt::skip]
    let am = MatrixDense::new(5, 4, [
        0, 0, 0, 1,
        0, 0, 1, 0,
        0, 0, 1, 0,
        0, 0, 0, 4,
        0, 0, 0, 4,
    ].map(R::from).into());
    #[rustfmt::skip]
    let bm = MatrixDense::new(5, 4, [
        0, 0, 0, 1,
        0, 0, 1, 0,
        0, 0, 1, 0,
        1, 0, 0, 0,
        1, 0, 0, 0,
    ].map(R::from).into());
    #[rustfmt::skip]
    let cm = MatrixDense::new(5, 4, [
        0, 1, 0, 0,
        0, 0, 0, 1,
        4, 0, 0, 0,
        0, 1, 0, 0,
        0, 1, 0, 0,
    ].map(R::from).into());
    let r1cs = R1CS::new(
        MatrixSparse::from(&am),
        MatrixSparse::from(&bm),
        MatrixSparse::from(&cm),
    );

    let circuit = CircuitBuilder::<R>::new(2);

    let scope = circuit.scope("multiplism");
    let c = Constant::from(R::from(4));
    let x = scope.public_output();
    let y = scope.public_output();
    let w = scope.private_input();

    scope.constrain(w * w, x);
    scope.constrain(y * y, w);
    scope.constrain(y * y, c);
    scope.constrain(w * c, x);
    scope.constrain(c * w, x);

    drop(scope);
    assert_eq!(circuit.r1cs(), r1cs);

    let z = VectorDense::from([1, 16, 2, 4].map(R::from));
    assert!(r1cs.is_satisfied(&z));
}

#[test]
fn expressionism() {
    #[rustfmt::skip]
    let am = MatrixDense::new(8, 5, [
        0, 1, 1, 0, 0,
        0, 0, 0, 1, 0,
        4, 1, 1, 1, 0,
        8, 0, 1, 1, 0,
        0, 0, 2, 2, 0,
        0, 6, 0, 0, 0,
        8, 2, 0, 0, 0,
        8, 2, 0, 0, 0,
    ].map(R::from).into());
    #[rustfmt::skip]
    let bm = MatrixDense::new(8, 5, [
        0, 0, 0, 1, 1,
        0, 0, 0, 1, 0,
        1, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
    ].map(R::from).into());
    #[rustfmt::skip]
    let cm = MatrixDense::new(8, 5, [
        160, 0, 0, 0, 0,
        0, 4, 0, 0, 0,
        0, 0, 0, 0, 1,
        0, 0, 0, 0, 1,
        0, 0, 0, 0, 1,
        24, 0, 0, 0, 0,
        0, 0, 0, 0, 1,
        0, 0, 0, 0, 1,
    ].map(R::from).into());
    let r1cs = R1CS::new(
        MatrixSparse::from(&am),
        MatrixSparse::from(&bm),
        MatrixSparse::from(&cm),
    );

    let circuit = CircuitBuilder::<R>::new(2);

    let scope = circuit.scope("expressionism");
    let a = Constant::from(R::from(160));
    let b = Constant::from(R::from(2));
    let c = Constant::from(R::from(4));
    let d = Constant::from(R::from(24));
    let x = scope.public_input();
    let y = scope.public_input();
    let z = scope.public_input();
    let w = scope.private_input();

    scope.constrain((x + y) * (z + w), a);
    scope.constrain(z * z, x * c);
    scope.constrain(x + y + z + c, w);
    scope.constrain(c + y + z + c, w);
    scope.constrain(b * y + z * b, w);
    scope.constrain(b * x + x * c, d);
    scope.constrain(c + b * (x + b), w);
    scope.constrain(b * (x + c), w);

    drop(scope);
    assert_eq!(circuit.r1cs(), r1cs);

    let z = VectorDense::from([1, 4, 4, 4, 16].map(R::from));
    assert!(r1cs.is_satisfied(&z));
}

#[test]
fn cubism() {
    #[rustfmt::skip]
    let am = MatrixDense::new(2, 5, [
        0, 1, 0, 0, 0,
        0, 1, 1, 0, 0,
    ].map(R::from).into());
    #[rustfmt::skip]
    let bm = MatrixDense::new(2, 5, [
        0, 1, 0, 0, 0,
        0, 1, 0, 1, 0,
    ].map(R::from).into());
    #[rustfmt::skip]
    let cm = MatrixDense::new(2, 5, [
        0, 1, 0, 0, 0,
        0, 1, 0, 0, 1,
    ].map(R::from).into());
    #[rustfmt::skip]
    let dm = MatrixDense::new(2, 5, [
        0, 0, 0, 0, 1,
        350, 0, 0, 0, 0,
    ].map(R::from).into());
    let ccs = CustomizableConstraintSystem::new(
        2,
        5,
        [&am, &bm, &cm, &dm].map(MatrixSparse::from).into(),
        vec![vec![0, 1, 2], vec![3]],
        [1, -1].map(R::from).into(),
    );

    let circuit = CircuitBuilder::<R>::new(3);

    let scope = circuit.scope("cubism");
    let c = Constant::from(R::from(350));
    let x = scope.public_input();
    let y = scope.public_input();
    let z = scope.public_input();
    let w = scope.auxiliary();

    scope.constrain(x * x * x, w);
    scope.constrain((x + y) * (x + z) * (x + w), c);

    drop(scope);
    assert_eq!(circuit.ccs(), ccs);

    let z = VectorDense::from([1, 2, 3, 5, 8].map(R::from));
    assert!(ccs.is_satisfied(&z));
}

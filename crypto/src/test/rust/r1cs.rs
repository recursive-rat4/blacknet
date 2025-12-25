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
use blacknet_crypto::constraintsystem::ConstraintSystem;
use blacknet_crypto::matrix::{DenseMatrix, DenseVector, SparseMatrix};
use blacknet_crypto::r1cs::R1CS;

type R = blacknet_crypto::pervushin::PervushinField;

#[test]
fn satisfaction() {
    #[rustfmt::skip]
    let a = DenseMatrix::new(3, 5, [
        0, 0, 1, 0, 0,
        0, 0, 0, 1, 0,
        0, 0, 0, 0, 1,
    ].map(R::from).into());
    #[rustfmt::skip]
    let b = DenseMatrix::new(3, 5, [
        0, 0, 0, 1, 0,
        0, 0, 0, 1, 0,
        0, 0, 0, 0, 1,
    ].map(R::from).into());
    #[rustfmt::skip]
    let c = DenseMatrix::new(3, 5, [
        4, 1, 0, 0, 0,
        0, 0, 1, 0, 0,
        0, 0, 0, 1, 0,
    ].map(R::from).into());
    let z = DenseVector::from([1, 60, 16, 4, 2].map(R::from));
    let r1cs = R1CS::new(
        SparseMatrix::from(&a),
        SparseMatrix::from(&b),
        SparseMatrix::from(&c),
    );

    assert_eq!(r1cs.degree(), 2);
    assert_eq!(r1cs.constraints(), 3);
    assert_eq!(r1cs.variables(), 5);

    assert_ok!(r1cs.is_satisfied(&z));
}

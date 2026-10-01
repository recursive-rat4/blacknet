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

use blacknet_crypto::algebra::One;
use blacknet_crypto::constraintsystem::{ConstraintSystem, Error};
use blacknet_crypto::matrix::{DenseMatrix, DenseVector, SparseBinaryMatrix, SparseMatrix};
use blacknet_crypto::r1cs::{BinaryR1CS, R1CS};
use core::assert_matches;

type R = blacknet_crypto::uring::U16Ring;

#[test]
fn r1cs() {
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

    assert_matches!(r1cs.is_satisfied(&z), Ok(()));
    assert_matches!(
        r1cs.is_satisfied(&DenseVector::default()),
        Err(Error::Variables(0, 5))
    );
    assert_matches!(
        r1cs.is_satisfied(&vec![R::ONE; 5].into()),
        Err(Error::Constraint(0))
    );
}

#[test]
fn binary_r1cs() {
    let a = unsafe { SparseBinaryMatrix::new(5, vec![0, 1, 2, 3], vec![2, 3, 4]) };
    let b = unsafe { SparseBinaryMatrix::new(5, vec![0, 1, 2, 3], vec![3, 3, 4]) };
    let c = unsafe { SparseBinaryMatrix::new(5, vec![0, 2, 3, 4], vec![0, 1, 2, 3]) };
    let z = DenseVector::from([1, 63, 16, 4, 2].map(R::from));
    let r1cs = BinaryR1CS::new(a, b, c);

    assert_eq!(ConstraintSystem::<DenseVector<R>>::degree(&r1cs), 2);
    assert_eq!(ConstraintSystem::<DenseVector<R>>::constraints(&r1cs), 3);
    assert_eq!(ConstraintSystem::<DenseVector<R>>::variables(&r1cs), 5);

    assert_matches!(
        ConstraintSystem::<DenseVector<R>>::is_satisfied(&r1cs, &z),
        Ok(())
    );
    assert_matches!(
        ConstraintSystem::<DenseVector<R>>::is_satisfied(&r1cs, &DenseVector::default()),
        Err(Error::Variables(0, 5))
    );
    assert_matches!(
        ConstraintSystem::<DenseVector<R>>::is_satisfied(&r1cs, &vec![R::ONE; 5].into()),
        Err(Error::Constraint(0))
    );
}

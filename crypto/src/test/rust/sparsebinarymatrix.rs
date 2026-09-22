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

use blacknet_crypto::matrix::{DenseVector, SparseBinaryMatrix, SparseMatrix};
use core::assert_matches;

type R = blacknet_crypto::uring::U16Ring;

#[test]
fn conversion() {
    let b = unsafe {
        SparseBinaryMatrix::new(
            4,
            [0, 2, 5, 7, 9].into(),
            [0, 1, 1, 2, 3, 0, 3, 1, 3].into(),
        )
    };
    let s = unsafe {
        SparseMatrix::<R>::new(
            4,
            [0, 2, 5, 7, 9].into(),
            [0, 1, 1, 2, 3, 0, 3, 1, 3].into(),
            [1, 1, 1, 1, 1, 1, 1, 1, 1].map(R::from).into(),
        )
    };
    assert_matches!(SparseBinaryMatrix::try_from(&s), Ok(x) if x == b);
    assert_eq!(SparseMatrix::from(b), s);
}

#[test]
fn vector() {
    let a = unsafe {
        SparseBinaryMatrix::new(
            4,
            [0, 3, 3, 6, 9, 11].into(),
            [0, 1, 3, 0, 1, 3, 0, 1, 3, 1, 3].into(),
        )
    };
    let b = DenseVector::<R>::from([61, 67, 71, 73].map(R::from));
    let c = DenseVector::<R>::from([201, 0, 201, 201, 140].map(R::from));
    assert_eq!(&a * &b, c);
}

#[test]
fn pad() {
    let a = unsafe { SparseBinaryMatrix::new(3, [0, 2, 4, 5].into(), [0, 1, 1, 2, 0].into()) };
    let b = unsafe { SparseBinaryMatrix::new(4, [0, 2, 4, 5, 5].into(), [0, 1, 1, 2, 0].into()) };
    assert_eq!(a.pad_to_power_of_two(), b);
    assert_eq!(b.clone().pad_to_power_of_two(), b);
}

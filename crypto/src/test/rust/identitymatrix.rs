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

use blacknet_crypto::algebra::Tensor;
use blacknet_crypto::matrix::{DenseMatrix, IdentityMatrix};

type R = blacknet_crypto::pervushin::PervushinField;

#[test]
#[rustfmt::skip]
fn tensor() {
    let a = IdentityMatrix::<R>::new(2);
    let b = DenseMatrix::<R>::new(2, 2, [
        5, 6,
        8, 9,
    ].map(R::from).into());
    let c = DenseMatrix::<R>::new(4, 4, [
         5, 6, 0, 0,
         8, 9, 0, 0,
         0, 0, 5, 6,
         0, 0, 8, 9,
    ].map(R::from).into());
    assert_eq!(a.tensor(b), c);
}

#[test]
fn trace() {
    let a = IdentityMatrix::<R>::new(4);
    let b = R::from(4);
    assert_eq!(a.trace(), b);
}

#[test]
#[rustfmt::skip]
fn dense() {
    let a = IdentityMatrix::<R>::new(3);
    let b = DenseMatrix::<R>::new(3, 3, [
        1, 0, 0,
        0, 1, 0,
        0, 0, 1,
    ].map(R::from).into());
    let c: DenseMatrix<R> = a.into();
    assert_eq!(c, b);
}

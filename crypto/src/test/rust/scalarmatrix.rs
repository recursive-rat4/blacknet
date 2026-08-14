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

use blacknet_crypto::matrix::{DenseMatrix, DenseVector, ScalarMatrix};

type R = blacknet_crypto::uring::U32Ring;

#[test]
#[rustfmt::skip]
fn mul_matrix() {
    let a = DenseMatrix::<R>::new(2, 4, [
         2,  3,  5,  7,
        11, 13, 17, 19,
    ].map(R::from).into());
    let b = DenseMatrix::<R>::new(2, 1, [
        23,
        29,
    ].map(R::from).into());
    let c = ScalarMatrix::<DenseMatrix<R>>::new(2, b);
    let d = DenseMatrix::<R>::new(2, 2, [
        133, 318,
        630, 942,
    ].map(R::from).into());
    assert_eq!(&a * &c, d);
}

#[test]
#[rustfmt::skip]
fn mul_vector() {
    let a = DenseMatrix::<R>::new(2, 3, [
        2,  3,  5,
        7, 11, 13,
    ].map(R::from).into());
    let b = ScalarMatrix::<DenseMatrix<R>>::new(2, a);
    let c = DenseVector::<R>::from([
        17, 19, 23, 29, 31, 37,
    ].map(R::from));
    let d = DenseVector::<R>::from([
        206, 627, 336, 1025,
    ].map(R::from));
    assert_eq!(&b * &c, d);
}

#[test]
fn trace() {
    let a = ScalarMatrix::<R>::new(3, R::from(2));
    let b = R::from(6);
    assert_eq!(a.trace(), b);
}

#[test]
fn transpose() {
    let a = DenseMatrix::<R>::new(2, 1, [1, 2].map(R::from).into());
    let b = ScalarMatrix::<DenseMatrix<R>>::new(2, a);
    let c = DenseMatrix::<R>::new(1, 2, [1, 2].map(R::from).into());
    let d = ScalarMatrix::<DenseMatrix<R>>::new(2, c);
    assert_eq!(b.transpose(), d);
    assert_eq!(d.transpose(), b);
}

#[test]
fn pad() {
    let a = ScalarMatrix::<R>::new(3, R::from(2));
    let b = ScalarMatrix::<R>::new(4, R::from(2));
    assert_eq!(a.pad_to_power_of_two(), b);
    assert_eq!(b.pad_to_power_of_two(), b);
}

#[test]
#[rustfmt::skip]
fn into() {
    let a = ScalarMatrix::<R>::new(3, R::from(2));
    let b = DenseMatrix::<R>::new(3, 3, [
        2, 0, 0,
        0, 2, 0,
        0, 0, 2,
    ].map(R::from).into());
    let c: DenseMatrix<R> = (&a).into();
    assert_eq!(c, b);
}

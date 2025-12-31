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

use blacknet_crypto::algebra::Double;
use blacknet_crypto::matrix::{DenseMatrix, DenseVector, SymmetricTridiagonalMatrix};

type R = blacknet_crypto::pervushin::PervushinField;

#[test]
fn conversion() {
    #[rustfmt::skip]
    let s = SymmetricTridiagonalMatrix::<R>::new([
        1, 2, 3, 4,
        5, 6, 7
    ].map(R::from).into());
    #[rustfmt::skip]
    let d = DenseMatrix::<R>::new(4, 4, [
        1, 5, 0, 0,
        5, 2, 6, 0,
        0, 6, 3, 7,
        0, 0, 7, 4,
    ].map(R::from).into());
    assert_eq!(DenseMatrix::from(&s), d);
}

#[test]
#[rustfmt::skip]
fn add() {
    let a = SymmetricTridiagonalMatrix::<R>::new([
        1, 2, 3,
        4, 5,
    ].map(R::from).into());
    let b = SymmetricTridiagonalMatrix::<R>::new([
        8, 6, 4,
        2, 0,
    ].map(R::from).into());
    let c = SymmetricTridiagonalMatrix::<R>::new([
        9, 8, 7,
        6, 5,
    ].map(R::from).into());
    assert_eq!(&a + &b, c);
    assert_eq!(b + a, c);
}

#[test]
#[rustfmt::skip]
fn dbl() {
    let a = SymmetricTridiagonalMatrix::<R>::new([
        0, 1, 2,
        3, 4,
    ].map(R::from).into());
    let b = SymmetricTridiagonalMatrix::<R>::new([
        0, 2, 4,
        6, 8,
    ].map(R::from).into());
    assert_eq!(a.double(), b);
}

#[test]
#[rustfmt::skip]
fn scalar() {
    let a = SymmetricTridiagonalMatrix::<R>::new([
        1, 2, 3,
        4, 5,
    ].map(R::from).into());
    let b = R::from(3);
    let c = SymmetricTridiagonalMatrix::<R>::new([
        3, 6, 9,
        12, 15,
    ].map(R::from).into());
    assert_eq!(a * b, c);
}

#[test]
#[rustfmt::skip]
fn vector() {
    let a = SymmetricTridiagonalMatrix::<R>::new([
        17, 23,
        29,
    ].map(R::from).into());
    let b = DenseVector::<R>::from([
        2,
        3,
    ].map(R::from));
    let c = DenseVector::<R>::from([
        121,
        127,
    ].map(R::from));
    let d = DenseVector::<R>::from([
        5740,
        6430,
    ].map(R::from));
    assert_eq!(&a * &b, c);
    assert_eq!(&c * &a, d);
}

#[test]
#[rustfmt::skip]
fn trace() {
    let a = SymmetricTridiagonalMatrix::<R>::new([
        1, 5,
        7,
    ].map(R::from).into());
    let b = R::from(6);
    assert_eq!(a.trace(), b);
}

#[test]
#[rustfmt::skip]
fn transpose() {
    let a = SymmetricTridiagonalMatrix::<R>::new([
        1, 2, 3,
        4, 5,
    ].map(R::from).into());
    let b = SymmetricTridiagonalMatrix::<R>::new([
        1, 2, 3,
        4, 5,
    ].map(R::from).into());
    assert_eq!(a.transpose(), &b);
    assert_eq!(b.transpose(), &a);
}

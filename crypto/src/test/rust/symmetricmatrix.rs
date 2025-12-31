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
use blacknet_crypto::matrix::{DenseMatrix, DenseVector, SymmetricMatrix};

type R = blacknet_crypto::pervushin::PervushinField;

#[test]
fn conversion() {
    #[rustfmt::skip]
    let s = SymmetricMatrix::<R>::new(4, [
        1,
        2, 3,
        4, 5, 6,
        7, 8, 9, 0,
    ].map(R::from).into());
    #[rustfmt::skip]
    let d = DenseMatrix::<R>::new(4, 4, [
        1, 2, 4, 7,
        2, 3, 5, 8,
        4, 5, 6, 9,
        7, 8, 9, 0,
    ].map(R::from).into());
    assert_eq!(DenseMatrix::from(&s), d);
}

#[test]
#[rustfmt::skip]
fn add() {
    let a = SymmetricMatrix::<R>::new(3, [
        1,
        3, 0,
        2, 1, 1,
    ].map(R::from).into());
    let b = SymmetricMatrix::<R>::new(3, [
        7,
        2, 9,
        4, 5, 6,
    ].map(R::from).into());
    let c = SymmetricMatrix::<R>::new(3, [
        8,
        5, 9,
        6, 6, 7,
    ].map(R::from).into());
    assert_eq!(&a + &b, c);
    assert_eq!(b + a, c);
}

#[test]
#[rustfmt::skip]
fn dbl() {
    let a = SymmetricMatrix::<R>::new(3, [
        9,
        2, 7,
        5, 8, 1,
    ].map(R::from).into());
    let b = SymmetricMatrix::<R>::new(3, [
        18,
        4, 14,
        10, 16, 2,
    ].map(R::from).into());
    assert_eq!(a.double(), b);
}

#[test]
#[rustfmt::skip]
fn scalar() {
    let a = SymmetricMatrix::<R>::new(3, [
        1,
        2, 3,
        4, 5, 6,
    ].map(R::from).into());
    let b = R::from(3);
    let c = SymmetricMatrix::<R>::new(3, [
        3,
        6, 9,
        12, 15, 18,
    ].map(R::from).into());
    assert_eq!(a * b, c);
}

#[test]
#[rustfmt::skip]
fn vector() {
    let a = SymmetricMatrix::<R>::new(2, [
        17,
        33, 49,
    ].map(R::from).into());
    let b = DenseVector::<R>::from([
        2,
        3,
    ].map(R::from));
    let c = DenseVector::<R>::from([
        133,
        213,
    ].map(R::from));
    let d = DenseVector::<R>::from([
        9290,
        14826,
    ].map(R::from));
    assert_eq!(&a * &b, c);
    assert_eq!(&c * &a, d);
}

#[test]
#[rustfmt::skip]
fn trace() {
    let a = SymmetricMatrix::<R>::new(2, [
        1,
        7, 5,
    ].map(R::from).into());
    let b = R::from(6);
    assert_eq!(a.trace(), b);
}

#[test]
#[rustfmt::skip]
fn transpose() {
    let a = SymmetricMatrix::<R>::new(3, [
        1,
        2, 3,
        4, 5, 6,
    ].map(R::from).into());
    let b = SymmetricMatrix::<R>::new(3, [
        1,
        2, 3,
        4, 5, 6,
    ].map(R::from).into());
    assert_eq!(a.transpose(), &b);
    assert_eq!(b.transpose(), &a);
}

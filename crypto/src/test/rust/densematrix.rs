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

use blacknet_crypto::algebra::{Double, Square, Tensor};
use blacknet_crypto::matrix::{DenseMatrix, DenseVector};
use blacknet_crypto::norm::InfinityNorm;
use core::iter::zip;

type R = blacknet_crypto::pervushin::PervushinField;

#[test]
#[rustfmt::skip]
fn add() {
    let a = DenseMatrix::<R>::new(3, 2, [
        1, 3,
        1, 0,
        1, 2,
    ].map(R::from).into());
    let b = DenseMatrix::<R>::new(3, 2, [
        0, 0,
        7, 5,
        2, 1,
    ].map(R::from).into());
    let c = DenseMatrix::<R>::new(3, 2, [
        1, 3,
        8, 5,
        3, 3,
    ].map(R::from).into());
    assert_eq!(&a + &b, c);
    assert_eq!(b + a, c);
}

#[test]
#[rustfmt::skip]
fn dbl() {
    let a = DenseMatrix::<R>::new(3, 3, [
        9, 3, 4,
        2, 7, 0,
        5, 8, 1,
    ].map(R::from).into());
    let b = DenseMatrix::<R>::new(3, 3, [
        18, 6, 8,
        4, 14, 0,
        10, 16, 2,
    ].map(R::from).into());
    assert_eq!(a.double(), b);
}

#[test]
#[rustfmt::skip]
fn mul() {
    let a = DenseMatrix::<R>::new(4, 3, [
        1, 0, 1,
        2, 1, 1,
        0, 1, 1,
        1, 1, 2,
    ].map(R::from).into());
    let b = DenseMatrix::<R>::new(3, 3, [
        1, 2, 1,
        2, 3, 1,
        4, 2, 2,
    ].map(R::from).into());
    let c = DenseMatrix::<R>::new(4, 3, [
        5, 4, 3,
        8, 9, 5,
        6, 5, 3,
        11, 9, 6,
    ].map(R::from).into());
    assert_eq!(a * b, c);
}

#[test]
#[rustfmt::skip]
fn sqr() {
    let a = DenseMatrix::<R>::new(3, 3, [
        9, 3, 4,
        2, 7, 0,
        5, 8, 1,
    ].map(R::from).into());
    let b = DenseMatrix::<R>::new(3, 3, [
        107, 80, 40,
        32, 55, 8,
        66, 79, 21,
    ].map(R::from).into());
    assert_eq!(a.square(), b);
}

#[test]
#[rustfmt::skip]
fn vector_product() {
    let a = DenseMatrix::<R>::new(3, 2, [
        17, 18,
        33, 34,
        49, 50,
    ].map(R::from).into());
    let b = DenseVector::<R>::from([
        2,
        3,
    ].map(R::from));
    let c = DenseVector::<R>::from([
        88,
        168,
        248,
    ].map(R::from));
    let d = DenseVector::<R>::from([
        19192,
        19696,
    ].map(R::from));
    assert_eq!(&a * &b, c);
    assert_eq!(&c * &a, d);
}

#[test]
#[rustfmt::skip]
fn tensor() {
    let a = DenseMatrix::<R>::new(2, 2, [
        1, 2,
        3, 4,
    ].map(R::from).into());
    let b = DenseMatrix::<R>::new(2, 2, [
        5, 6,
        8, 9,
    ].map(R::from).into());
    let c = DenseMatrix::<R>::new(4, 4, [
         5,  6, 10, 12,
         8,  9, 16, 18,
        15, 18, 20, 24,
        24, 27, 32, 36,
    ].map(R::from).into());
    let d = DenseMatrix::<R>::new(4, 4, [
         5, 10,  6, 12,
        15, 20, 18, 24,
         8, 16,  9, 18,
        24, 32, 27, 36,
    ].map(R::from).into());
    assert_eq!((&a).tensor(&b), c);
    assert_eq!(b.tensor(a), d);
}

#[test]
#[rustfmt::skip]
fn row_tensor() {
    let a = DenseMatrix::<R>::new(2, 2, [
        1, 2,
        4, 5,
    ].map(R::from).into());
    let b = DenseMatrix::<R>::new(2, 2, [
        1, 7,
        3, 8,
    ].map(R::from).into());
    let c = DenseMatrix::<R>::new(2, 4, [
         1,  7,  2, 14,
        12, 32, 15, 40,
    ].map(R::from).into());
    let d = DenseMatrix::<R>::new(2, 4, [
         1,  2,  7, 14,
        12, 15, 32, 40,
    ].map(R::from).into());
    assert_eq!(a.row_tensor(&b), c);
    assert_eq!(b.row_tensor(&a), d);
}

#[test]
#[rustfmt::skip]
fn column_tensor() {
    let a = DenseMatrix::<R>::new(2, 2, [
        1, 2,
        4, 5,
    ].map(R::from).into());
    let b = DenseMatrix::<R>::new(2, 2, [
        1, 7,
        3, 8,
    ].map(R::from).into());
    let c = DenseMatrix::<R>::new(4, 2, [
         1, 14,
         3, 16,
         4, 35,
        12, 40,
    ].map(R::from).into());
    let d = DenseMatrix::<R>::new(4, 2, [
         1, 14,
         4, 35,
         3, 16,
        12, 40,
    ].map(R::from).into());
    assert_eq!(a.column_tensor(&b), c);
    assert_eq!(b.column_tensor(&a), d);
}

#[test]
#[rustfmt::skip]
fn concat() {
    let a = DenseMatrix::<R>::new(3, 2, [
        1, 3,
        1, 0,
        1, 2,
    ].map(R::from).into());
    let b = DenseMatrix::<R>::new(3, 2, [
        0, 0,
        7, 5,
        2, 1,
    ].map(R::from).into());
    let c = DenseMatrix::<R>::new(3, 4, [
        1, 3, 0, 0,
        1, 0, 7, 5,
        1, 2, 2, 1,
    ].map(R::from).into());
    assert_eq!(a.concat(&b), c);
}

#[test]
#[rustfmt::skip]
fn vectorize() {
    let a = DenseMatrix::<R>::new(2, 2, [
        1, 2,
        3, 4,
    ].map(R::from).into());
    let b = DenseVector::<R>::from([
        1, 2, 3, 4,
    ].map(R::from));
    assert_eq!(a.vectorize(), b);
}

#[test]
#[rustfmt::skip]
fn trace() {
    let a = DenseMatrix::<R>::new(2, 2, [
        1, 3,
        7, 5,
    ].map(R::from).into());
    let b = R::from(6);
    assert_eq!(a.trace(), b);
}

#[test]
#[rustfmt::skip]
fn transpose() {
    let a = DenseMatrix::<R>::new(3, 2, [
        1, 2,
        3, 4,
        5, 6,
    ].map(R::from).into());
    let b = DenseMatrix::<R>::new(2, 3, [
        1, 3, 5,
        2, 4, 6,
    ].map(R::from).into());
    assert_eq!(a.transpose(), b);
    assert_eq!(b.transpose(), a);
}

#[test]
#[rustfmt::skip]
fn pad() {
    let a = DenseMatrix::<R>::new(3, 3, [
        1, 2, 3,
        4, 5, 6,
        7, 8, 9,
    ].map(R::from).into());
    let b = DenseMatrix::<R>::new(4, 4, [
        1, 2, 3, 0,
        4, 5, 6, 0,
        7, 8, 9, 0,
        0, 0, 0, 0,
    ].map(R::from).into());
    assert_eq!(a.pad_to_power_of_two(), b);
    assert_eq!(b.pad_to_power_of_two(), b);
}

#[test]
#[rustfmt::skip]
fn infinity_norm() {
    let a = DenseMatrix::<R>::new(2, 2, [
        0, 1,
        2, 3,
    ].map(R::from).into());
    let n = 3;
    let b = 4;
    assert!(!a.check_infinity_norm(&n));
    assert!(a.check_infinity_norm(&b));
    assert_eq!(a.infinity_norm(), n);
}

#[test]
#[rustfmt::skip]
fn iter_row() {
    let a = DenseMatrix::<R>::new(2, 3, [
        1, 2, 3,
        4, 5, 6,
    ].map(R::from).into());
    let b: Vec<Vec<R>> = vec![
        [1, 2, 3].map(R::from).into(),
        [4, 5, 6].map(R::from).into(),
    ];
    zip(a.iter_row(), b).for_each(|(a,b)| assert_eq!(a, b));
}

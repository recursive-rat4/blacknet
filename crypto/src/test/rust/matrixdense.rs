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

use blacknet_crypto::matrixdense::MatrixDense;
use blacknet_crypto::vectordense::VectorDense;

type R = blacknet_crypto::field25519::Field25519;

#[test]
#[rustfmt::skip]
fn add() {
    let a = MatrixDense::<R>::new(3, 2, [
        1, 3,
        1, 0,
        1, 2,
    ].map(R::from).into());
    let b = MatrixDense::<R>::new(3, 2, [
        0, 0,
        7, 5,
        2, 1,
    ].map(R::from).into());
    let c = MatrixDense::<R>::new(3, 2, [
        1, 3,
        8, 5,
        3, 3,
    ].map(R::from).into());
    assert_eq!(&a + &b, c);
    assert_eq!(b + a, c);
}

#[test]
#[rustfmt::skip]
fn mul() {
    let a = MatrixDense::<R>::new(4, 3, [
        1, 0, 1,
        2, 1, 1,
        0, 1, 1,
        1, 1, 2,
    ].map(R::from).into());
    let b = MatrixDense::<R>::new(3, 3, [
        1, 2, 1,
        2, 3, 1,
        4, 2, 2,
    ].map(R::from).into());
    let c = MatrixDense::<R>::new(4, 3, [
        5, 4, 3,
        8, 9, 5,
        6, 5, 3,
        11, 9, 6,
    ].map(R::from).into());
    assert_eq!(a * b, c);
}

#[test]
#[rustfmt::skip]
fn vector() {
    let a = MatrixDense::<R>::new(3, 2, [
        17, 18,
        33, 34,
        49, 50,
    ].map(R::from).into());
    let b = VectorDense::<R>::from([
        2,
        3,
    ].map(R::from));
    let c = VectorDense::<R>::from([
        88,
        168,
        248,
    ].map(R::from));
    let d = VectorDense::<R>::from([
        19192,
        19696,
    ].map(R::from));
    assert_eq!(&a * &b, c);
    assert_eq!(&c * &a, d);
}

#[test]
#[rustfmt::skip]
fn cat() {
    let a = MatrixDense::<R>::new(3, 2, [
        1, 3,
        1, 0,
        1, 2,
    ].map(R::from).into());
    let b = MatrixDense::<R>::new(3, 2, [
        0, 0,
        7, 5,
        2, 1,
    ].map(R::from).into());
    let c = MatrixDense::<R>::new(3, 4, [
        1, 3, 0, 0,
        1, 0, 7, 5,
        1, 2, 2, 1,
    ].map(R::from).into());
    assert_eq!(a.cat(&b), c);
}

#[test]
#[rustfmt::skip]
fn trace() {
    let a = MatrixDense::<R>::new(2, 2, [
        1, 3,
        7, 5,
    ].map(R::from).into());
    let b = R::from(6);
    assert_eq!(a.trace(), b);
}

#[test]
#[rustfmt::skip]
fn transpose() {
    let a = MatrixDense::<R>::new(3, 2, [
        1, 2,
        3, 4,
        5, 6,
    ].map(R::from).into());
    let b = MatrixDense::<R>::new(2, 3, [
        1, 3, 5,
        2, 4, 6,
    ].map(R::from).into());
    assert_eq!(a.transpose(), b);
    assert_eq!(b.transpose(), a);
}

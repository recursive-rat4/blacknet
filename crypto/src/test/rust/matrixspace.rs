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

use blacknet_crypto::algebra::{Double, MatrixSpace, VectorRing, Zero};
use blacknet_crypto::norm::InfinityNorm;

type Z = blacknet_crypto::uring::U16Ring;
type M2 = VectorRing<Z, 2>;
type M3 = VectorRing<Z, 3>;
type S = MatrixSpace<Z, 2, 3, 6>;
type ST = MatrixSpace<Z, 3, 2, 6>;

#[test]
#[rustfmt::skip]
fn add() {
    let a = S::from([
        1, 3, 7,
        1, 0, 0,
    ].map(Z::from));
    let b = S::from([
        0, 0, 0,
        7, 5, 3,
    ].map(Z::from));
    let c = S::from([
        1, 3, 7,
        8, 5, 3,
    ].map(Z::from));
    assert_eq!(a + b, c);
    assert_eq!(b + a, c);
}

#[test]
#[rustfmt::skip]
fn dbl() {
    let a = S::from([
        0, 1, 2,
        3, 4, 5,
    ].map(Z::from));
    let b = S::from([
        0, 2, 4,
        6, 8, 10,
    ].map(Z::from));
    assert_eq!(a.double(), b);
    assert_eq!(S::ZERO.double(), S::ZERO);
}

#[test]
#[rustfmt::skip]
fn vector() {
    let a = S::from([
        17, 18, 19,
        33, 34, 35,
    ].map(Z::from));
    let b = M3::from([
        2,
        3,
        5,
    ].map(Z::from));
    let c = M2::from([
        183, 343,
    ].map(Z::from));
    let d = M3::from([
        14430, 14956, 15482,
    ].map(Z::from));
    assert_eq!(a * b, c);
    assert_eq!(c * a, d);
}

#[test]
#[rustfmt::skip]
fn scalar() {
    let a = S::from([
        1, 2, 4,
        0, 3, 9,
    ].map(Z::from));
    let b = Z::from(2);
    let c = S::from([
        2, 4, 8,
        0, 6, 18,
    ].map(Z::from));
    assert_eq!(a * b, c);
    //assert_eq!(b * a, c);
}

#[test]
#[rustfmt::skip]
fn transpose() {
    let a = S::from([
        1, 2, 3,
        4, 5, 6,
    ].map(Z::from));
    let b = ST::from([
        1, 4,
        2, 5,
        3, 6,
    ].map(Z::from));
    assert_eq!(a.transpose(), b);
    assert_eq!(b.transpose(), a);
}

#[test]
#[rustfmt::skip]
fn infinity_norm() {
    let a = S::from([
        0, 1, 2,
        3, 4, 5,
    ].map(Z::from));
    let n = 5;
    let b = 6;
    assert!(!a.check_infinity_norm(&n));
    assert!(a.check_infinity_norm(&b));
    assert_eq!(a.infinity_norm(), n);
}

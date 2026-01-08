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

use blacknet_crypto::algebra::{FreeModule, MatrixRing, MultiplicativeMonoid};
use blacknet_crypto::norm::InfinityNorm;

type Z = blacknet_crypto::pervushin::PervushinField;
type M = FreeModule<Z, 2>;
type R = MatrixRing<Z, 2, 4>;

#[test]
#[rustfmt::skip]
fn add() {
    let a = R::new([
        1, 3,
        1, 0,
    ].map(Z::from));
    let b = R::new([
        0, 0,
        7, 5,
    ].map(Z::from));
    let c = R::new([
        1, 3,
        8, 5,
    ].map(Z::from));
    assert_eq!(a + b, c);
    assert_eq!(b + a, c);
}

#[test]
#[rustfmt::skip]
fn mul() {
    let a = R::new([
        1, 0,
        2, 1,
    ].map(Z::from));
    let b = R::new([
        1, 2,
        2, 3,
    ].map(Z::from));
    let c = R::new([
        1, 2,
        4, 7,
    ].map(Z::from));
    let d = R::new([
        5, 2,
        8, 3,
    ].map(Z::from));
    assert_eq!(a * b, c);
    assert_eq!(b * a, d);
}

#[test]
#[rustfmt::skip]
fn module() {
    let a = R::new([
        17, 18,
        33, 34,
    ].map(Z::from));
    let b = M::from([
        2,
        3,
    ].map(Z::from));
    let c = M::from([
        88,
        168,
    ].map(Z::from));
    let d = M::from([
        133,
        138,
    ].map(Z::from));
    assert_eq!(a * b, c);
    assert_eq!(b * a, d);
}

#[test]
#[rustfmt::skip]
fn scalar() {
    let a = R::new([
        1, 2,
        0, 3,
    ].map(Z::from));
    let b = Z::from(2);
    let c = R::new([
        2, 4,
        0, 6,
    ].map(Z::from));
    assert_eq!(a * b, c);
    //assert_eq!(b * a, c);
}

#[test]
#[rustfmt::skip]
fn trace() {
    let a = R::new([
        1, 2,
        0, 3,
    ].map(Z::from));
    let b = Z::from(4);
    assert_eq!(a.trace(), b);
}

#[test]
#[rustfmt::skip]
fn transpose() {
    let a = R::new([
        1, 2,
        3, 4,
    ].map(Z::from));
    let b = R::new([
        1, 3,
        2, 4,
    ].map(Z::from));
    assert_eq!(a.transpose(), b);
    assert_eq!(b.transpose(), a);
}

#[test]
#[rustfmt::skip]
fn identity() {
    let i = R::new([
        1, 0,
        0, 1,
    ].map(Z::from));
    assert_eq!(R::ONE, i);
}

#[test]
#[rustfmt::skip]
fn infinity_norm() {
    let a = R::new([
        0, 1,
        2, 3,
    ].map(Z::from));
    let nb = 3;
    let ng = 4;
    assert!(!a.check_infinity_norm(&nb));
    assert!(a.check_infinity_norm(&ng));
}

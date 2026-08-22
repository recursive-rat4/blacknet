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

use blacknet_crypto::algebra::{Dot, Double, One, Square, VectorRing, Zero};
use blacknet_crypto::norm::{LInf, Norm};

type Z = blacknet_crypto::uring::U8Ring;
type R = VectorRing<Z, 2>;

#[test]
fn add() {
    let a = R::from([2, 3].map(Z::from));
    let b = R::from([11, 13].map(Z::from));
    let c = R::from([13, 16].map(Z::from));
    assert_eq!(a + b, c);
    assert_eq!(b + a, c);
    assert_eq!(c + R::ZERO, c);
    assert_eq!(R::ZERO + c, c);
    assert_eq!(R::ONE + R::ZERO, R::ONE);
    assert_eq!(R::ZERO + R::ONE, R::ONE);
}

#[test]
fn dbl() {
    let a = R::from([2, 3].map(Z::from));
    let b = R::from([4, 6].map(Z::from));
    assert_eq!(a.double(), b);
    assert_eq!(R::ONE.double(), R::ONE + R::ONE);
    assert_eq!(R::ZERO.double(), R::ZERO);
}

#[test]
fn neg() {
    let a = R::from([2, 3].map(Z::from));
    let b = R::from([254, 253].map(Z::from));
    assert_eq!(-a, b);
    assert_eq!(-b, a);
    assert_eq!(-R::ZERO, R::ZERO);
}

#[test]
fn sub() {
    let a = R::from([2, 3].map(Z::from));
    let b = R::from([5, 7].map(Z::from));
    let c = R::from([253, 252].map(Z::from));
    let d = R::from([3, 4].map(Z::from));
    assert_eq!(a - b, c);
    assert_eq!(b - a, d);
    assert_eq!(c - R::ZERO, c);
    assert_eq!(R::ZERO - R::ZERO, R::ZERO);
    assert_eq!(R::ONE - R::ONE, R::ZERO);
}

#[test]
fn mul() {
    let a = R::from([2, 3].map(Z::from));
    let b = R::from([5, 7].map(Z::from));
    let c = R::from([10, 21].map(Z::from));
    assert_eq!(a * b, c);
    assert_eq!(b * a, c);
    assert_eq!(c * Z::ZERO, R::ZERO);
    assert_eq!(R::ZERO * c, R::ZERO);
    assert_eq!(R::ONE * c, c);
    assert_eq!(c * R::ONE, c);
}

#[test]
fn sqr() {
    let a = R::from([2, 3].map(Z::from));
    let b = R::from([4, 9].map(Z::from));
    assert_eq!(a.square(), b);
    assert_eq!(R::ZERO.square(), R::ZERO);
    assert_eq!(R::ONE.square(), R::ONE);
}

#[test]
fn dot() {
    let a = R::from([3, 252].map(Z::from));
    let b = R::from([254, 255].map(Z::from));
    let c = Z::from(254);
    let d = Z::from(25);
    assert_eq!(a.dot(b), c);
    assert_eq!(b.dot(a), c);
    assert_eq!(a.dot(a), d);
}

#[test]
fn inf() {
    let a = R::from([255, 4].map(Z::from));
    let n = 4;
    let b = 8;

    assert!(!Norm::<LInf>::check_norm(&a, &n));
    assert!(Norm::<LInf>::check_norm(&a, &b));
    assert_eq!(Norm::<LInf>::norm(&a), n);
}

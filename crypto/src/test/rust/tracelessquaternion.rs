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

use blacknet_crypto::algebra::{
    Conjugate, Double, IntegerModRing, Inv, QuaternionAlgebra, Square, TracelessQuaternion, Zero,
};

type Z = blacknet_crypto::pervushin::PervushinField;
type A = QuaternionAlgebra<Z>;
type M = TracelessQuaternion<Z>;

#[test]
fn conversion() {
    let a = M::from([3, 5, 7].map(Z::from));
    let b = A::from([0, 3, 5, 7].map(Z::from));
    assert_eq!(A::from(a), b);
}

#[test]
fn add() {
    let a = M::from([3, 5, 7].map(Z::from));
    let b = M::from([13, 17, 19].map(Z::from));
    let c = M::from([16, 22, 26].map(Z::from));
    assert_eq!(a + b, c);
    assert_eq!(b + a, c);
    assert_eq!(c + M::ZERO, c);
    assert_eq!(M::ZERO + c, c);
}

#[test]
fn dbl() {
    let a = M::from([3, 5, 7].map(Z::from));
    let b = M::from([6, 10, 14].map(Z::from));
    assert_eq!(a.double(), b);
    assert_eq!(M::ZERO.double(), M::ZERO);
}

#[test]
fn neg() {
    let a = M::from([3, 5, 7].map(Z::from));
    let b = M::from([-3, -5, -7].map(Z::from));
    assert_eq!(-a, b);
    assert_eq!(-b, a);
    assert_eq!(-M::ZERO, M::ZERO);
}

#[test]
fn sub() {
    let a = M::from([3, 5, 7].map(Z::from));
    let b = M::from([13, 17, 19].map(Z::from));
    let c = M::from([-10, -12, -12].map(Z::from));
    let d = M::from([10, 12, 12].map(Z::from));
    assert_eq!(a - b, c);
    assert_eq!(b - a, d);
    assert_eq!(c - M::ZERO, c);
    assert_eq!(M::ZERO - M::ZERO, M::ZERO);
}

#[test]
fn sqr() {
    let a = M::from([3, 5, 7].map(Z::from));
    let b = Z::from(-83);
    assert_eq!(a.square(), b);
    assert_eq!(M::ZERO.square(), Z::ZERO);
}

#[test]
fn inv() {
    let a = M::from([3, 5, 7].map(Z::from));
    let b = M::from([0x1BC2503159721ED7, 0x0E43DAFCEA68DE12, 0x00C565C87B5F9D4D].map(Z::new));
    assert_eq!(b.inv().unwrap(), a);
    assert_eq!(a.inv().unwrap(), b);
    assert!(M::ZERO.inv().is_none());
}

#[test]
fn conjugate() {
    let a = M::from([3, 5, 7].map(Z::from));
    let b = M::from([-3, -5, -7].map(Z::from));
    assert_eq!(a.conjugate(), b);
    assert_eq!(b.conjugate(), a);
}

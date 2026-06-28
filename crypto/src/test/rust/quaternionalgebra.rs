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
    Commutator, Conjugate, Double, IntegerModRing, Inv, One, QuaternionAlgebra, Square,
    TracelessQuaternion, Zero,
};
use blacknet_crypto::norm::InfinityNorm;

type Z = blacknet_crypto::pervushin::PervushinField;
type A = QuaternionAlgebra<Z>;
type M = TracelessQuaternion<Z>;

#[test]
fn add() {
    let a = A::from([2, 3, 5, 7].map(Z::from));
    let b = A::from([11, 13, 17, 19].map(Z::from));
    let c = A::from([13, 16, 22, 26].map(Z::from));
    assert_eq!(a + b, c);
    assert_eq!(b + a, c);
    assert_eq!(c + A::ZERO, c);
    assert_eq!(A::ZERO + c, c);
    assert_eq!(A::ONE + A::ZERO, A::ONE);
    assert_eq!(A::ZERO + A::ONE, A::ONE);
}

#[test]
fn dbl() {
    let a = A::from([2, 3, 5, 7].map(Z::from));
    let b = A::from([4, 6, 10, 14].map(Z::from));
    assert_eq!(a.double(), b);
    assert_eq!(A::ONE.double(), A::ONE + A::ONE);
    assert_eq!(A::ZERO.double(), A::ZERO);
}

#[test]
fn neg() {
    let a = A::from([2, 3, 5, 7].map(Z::from));
    let b = A::from([-2, -3, -5, -7].map(Z::from));
    assert_eq!(-a, b);
    assert_eq!(-b, a);
    assert_eq!(-A::ZERO, A::ZERO);
}

#[test]
fn sub() {
    let a = A::from([2, 3, 5, 7].map(Z::from));
    let b = A::from([11, 13, 17, 19].map(Z::from));
    let c = A::from([-9, -10, -12, -12].map(Z::from));
    let d = A::from([9, 10, 12, 12].map(Z::from));
    assert_eq!(a - b, c);
    assert_eq!(b - a, d);
    assert_eq!(c - A::ZERO, c);
    assert_eq!(A::ZERO - A::ZERO, A::ZERO);
    assert_eq!(A::ONE - A::ONE, A::ZERO);
}

#[test]
fn mul() {
    let a = A::from([2, 3, 5, 7].map(Z::from));
    let b = A::from([11, 13, 17, 19].map(Z::from));
    let c = A::from([-235, 35, 123, 101].map(Z::from));
    let d = A::from([-235, 83, 55, 129].map(Z::from));
    assert_eq!(a * b, c);
    assert_eq!(b * a, d);
    assert_eq!(c * Z::ZERO, A::ZERO);
    assert_eq!(A::ZERO * c, A::ZERO);
    assert_eq!(A::ONE * c, c);
    assert_eq!(c * A::ONE, c);
}

#[test]
fn sqr() {
    let a = A::from([2, 3, 5, 7].map(Z::from));
    let b = A::from([-79, 12, 20, 28].map(Z::from));
    assert_eq!(a.square(), b);
    assert_eq!(A::ZERO.square(), A::ZERO);
    assert_eq!(A::ONE.square(), A::ONE);
}

#[test]
fn inv() {
    let a = A::from([2, 3, 5, 7].map(Z::from));
    let b = A::from(
        [
            0x0A4C8178A4C8178A,
            0x108D3DCB08D3DCB0,
            0x0640BC52640BC526,
            0x1BF43AD9BF43AD9B,
        ]
        .map(Z::new),
    );
    assert_eq!(b.inv().unwrap(), a);
    assert_eq!(a.inv().unwrap(), b);
    assert_eq!(A::ZERO.inv(), None);
}

#[test]
fn conjugate() {
    let a = A::from([2, 3, 5, 7].map(Z::from));
    let b = A::from([2, -3, -5, -7].map(Z::from));
    assert_eq!(a.conjugate(), b);
    assert_eq!(b.conjugate(), a);
}

#[test]
fn commutator() {
    let a = A::from([2, 3, 5, 7].map(Z::from));
    let b = A::from([11, 13, 17, 19].map(Z::from));
    let c = M::from([-48, 68, -28].map(Z::from));
    assert_eq!(a.commutator(b), c);
}

#[test]
fn infinity_norm() {
    let a = A::from([0, 3, 6, -7].map(Z::from));
    let ab = 7;
    let ag = 8;
    assert!(!a.check_infinity_norm(&ab));
    assert!(a.check_infinity_norm(&ag));
}

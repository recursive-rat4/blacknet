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

use blacknet_crypto::algebra::{BalancedRepresentative, Double, IntegerModRing, One, Square, Zero};
use blacknet_crypto::norm::InfinityNorm;

type Z = blacknet_crypto::uring::U32Ring;

#[test]
fn representative() {
    let a = Z::from(0);
    let b = Z::from(0xFFFFFFFF);
    assert_eq!(a.canonical(), 0);
    assert_eq!(b.canonical(), 4294967295);
    assert_eq!(a.balanced(), 0);
    assert_eq!(b.balanced(), -1);
    assert_eq!(a.absolute(), 0);
    assert_eq!(b.absolute(), 1);
}

#[test]
fn add() {
    let a = Z::from(0x6A9DC620);
    let b = Z::from(0xF9EC0358);
    let c = Z::from(0x6489C978);
    assert_eq!(a + b, c);
    assert_eq!(b + a, c);
    assert_eq!(c + Z::ZERO, c);
    assert_eq!(Z::ZERO + c, c);
    assert_eq!(Z::ONE + Z::ZERO, Z::ONE);
    assert_eq!(Z::ZERO + Z::ONE, Z::ONE);
}

#[test]
fn dbl() {
    let a = Z::from(0xF9EC0358);
    let b = Z::from(0xF3D806B0);
    assert_eq!(a.double(), b);
    assert_eq!(Z::ZERO.double(), Z::ZERO);
    assert_eq!(Z::ONE.double(), Z::from(2));
}

#[test]
fn neg() {
    let a = Z::from(4);
    let b = Z::from(0xFFFFFFFC);
    assert_eq!(-a, b);
    assert_eq!(-b, a);
    assert_eq!(-Z::ZERO, Z::ZERO);
}

#[test]
fn sub() {
    let a = Z::from(0x6A9DC620);
    let b = Z::from(0xF9EC0358);
    let c = Z::from(0x70B1C2C8);
    let d = Z::from(0x8F4E3D38);
    assert_eq!(a - b, c);
    assert_eq!(b - a, d);
    assert_eq!(c - Z::ZERO, c);
    assert_eq!(Z::ZERO - Z::ZERO, Z::ZERO);
    assert_eq!(Z::ONE - Z::ONE, Z::ZERO);
}

#[test]
fn mul() {
    let a = Z::from(0x6A9DC620);
    let b = Z::from(0xF9EC0358);
    let c = Z::from(0x450E7B00);
    assert_eq!(a * b, c);
    assert_eq!(b * a, c);
    assert_eq!(c * Z::ZERO, Z::ZERO);
    assert_eq!(Z::ZERO * c, Z::ZERO);
    assert_eq!(Z::ONE * c, c);
    assert_eq!(c * Z::ONE, c);
}

#[test]
fn sqr() {
    assert_eq!(Z::from(0x6A9DC620).square(), Z::from(0x1C958400));
    assert_eq!(Z::ZERO.square(), Z::ZERO);
    assert_eq!(Z::ONE.square(), Z::ONE);
}

#[test]
fn sum() {
    let a = [0xFFFFFFFF, 0xFFFFFFFE, 0xFFFFFFFD, 0xFFFFFFFC, 11].map(Z::from);
    assert_eq!(a.into_iter().sum::<Z>(), Z::ONE);
}

#[test]
fn infinity_norm() {
    let a = Z::from(0xFFFF8AD0);
    let b = Z::from(30000);
    let nb = 30000;
    let ng = 30001;
    assert!(!a.check_infinity_norm(&nb));
    assert!(a.check_infinity_norm(&ng));
    assert!(!b.check_infinity_norm(&nb));
    assert!(b.check_infinity_norm(&ng));
}

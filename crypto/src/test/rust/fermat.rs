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

use blacknet_crypto::algebra::{BalancedRepresentative, IntegerModRing, Inv, One, Square, Zero};
use blacknet_crypto::norm::InfinityNorm;

type Z = blacknet_crypto::fermat::FermatField;

#[test]
fn representative() {
    let a = Z::with_int(-1);
    let b = Z::with_int(65536);
    assert_eq!(b, a);
    assert_eq!(a.canonical(), 65536);
    assert_eq!(b.canonical(), 65536);
    assert_eq!(a.balanced(), -1);
    assert_eq!(b.balanced(), -1);
    assert_eq!(a.absolute(), 1);
    assert_eq!(b.absolute(), 1);
}

#[test]
fn add() {
    let a = Z::with_int(981);
    let b = Z::with_int(-1516);
    let c = Z::with_int(-535);
    assert_eq!(a + b, c);
    assert_eq!(b + a, c);
    assert_eq!(c + Z::ZERO, c);
    assert_eq!(Z::ZERO + c, c);
    assert_eq!(Z::ONE + Z::ZERO, Z::ONE);
    assert_eq!(Z::ZERO + Z::ONE, Z::ONE);
    assert_eq!(Z::ONE + (-Z::ONE), Z::ZERO);
}

#[test]
fn neg() {
    let a = Z::with_int(4);
    let b = Z::with_int(-4);
    assert_eq!(-a, b);
    assert_eq!(-b, a);
    assert_eq!(-Z::ZERO, Z::ZERO);
}

#[test]
fn sub() {
    let a = Z::with_int(-1045);
    let b = Z::with_int(32750);
    let c = Z::with_int(31742);
    let d = Z::with_int(-31742);
    assert_eq!(a - b, c);
    assert_eq!(b - a, d);
    assert_eq!(c - Z::ZERO, c);
    assert_eq!(Z::ZERO - Z::ZERO, Z::ZERO);
    assert_eq!(Z::ONE - Z::ONE, Z::ZERO);
}

#[test]
fn mul() {
    let a = Z::with_int(-684);
    let b = Z::with_int(-133);
    let c = Z::with_int(25435);
    assert_eq!(a * b, c);
    assert_eq!(b * a, c);
    assert_eq!(c * Z::ZERO, Z::ZERO);
    assert_eq!(Z::ZERO * c, Z::ZERO);
    assert_eq!(Z::ONE * c, c);
    assert_eq!(c * Z::ONE, c);
}

#[test]
fn sqr() {
    assert_eq!((-Z::ONE).square(), Z::ONE);
    assert_eq!(Z::ZERO.square(), Z::ZERO);
    assert_eq!(Z::ONE.square(), Z::ONE);
}

#[test]
fn inv() {
    let a = Z::with_int(24);
    let b = Z::with_int(19115);
    let c = Z::with_int(-25);
    let d = Z::with_int(-5243);
    assert_eq!(b.inv().unwrap(), a);
    assert_eq!(a.inv().unwrap(), b);
    assert_eq!(c.inv().unwrap(), d);
    assert_eq!(d.inv().unwrap(), c);
    assert!(Z::ZERO.inv().is_none());
}

#[test]
fn div() {
    let a = Z::with_int(0xE9);
    let b = Z::with_int(0x40);
    let c = Z::with_int(0x5C04);
    let d = Z::with_int(0xFEE8);
    assert_eq!((a / b).unwrap(), c);
    assert_eq!((b / a).unwrap(), d);
    assert_eq!((-Z::ONE / -Z::ONE).unwrap(), Z::ONE);
    assert!((Z::ONE / Z::ZERO).is_none());
}

#[test]
fn sum() {
    let a = [-1, -2, -3, -4, 11].map(Z::with_int);
    assert_eq!(a.into_iter().sum::<Z>(), Z::ONE);
}

#[test]
fn infinity_norm() {
    let a = Z::with_int(-30000);
    let b = Z::with_int(30000);
    let nb = 30000;
    let ng = 30001;
    assert!(!a.check_infinity_norm(&nb));
    assert!(a.check_infinity_norm(&ng));
    assert!(!b.check_infinity_norm(&nb));
    assert!(b.check_infinity_norm(&ng));
}

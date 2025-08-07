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

use blacknet_crypto::magma::{Inv, MultiplicativeMagma};
use blacknet_crypto::norm::InfinityNorm;
use blacknet_crypto::ring::{IntegerRing, Ring};

type Z = blacknet_crypto::fermat::FermatField;

#[test]
fn representative() {
    let a = Z::new(-1);
    let b = Z::new(65536);
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
    let a = Z::from(981);
    let b = Z::from(-1516);
    let c = Z::from(-535);
    assert_eq!(a + b, c);
    assert_eq!(b + a, c);
    assert_eq!(c + Z::ZERO, c);
    assert_eq!(Z::ZERO + c, c);
    assert_eq!(Z::UNITY + Z::ZERO, Z::UNITY);
    assert_eq!(Z::ZERO + Z::UNITY, Z::UNITY);
    assert_eq!(Z::from(1) + Z::from(-1), Z::ZERO);
}

#[test]
fn mul() {
    let a = Z::from(-684);
    let b = Z::from(-133);
    let c = Z::from(25435);
    assert_eq!(a * b, c);
    assert_eq!(b * a, c);
    assert_eq!(c * Z::ZERO, Z::ZERO);
    assert_eq!(Z::ZERO * c, Z::ZERO);
    assert_eq!(Z::UNITY * c, c);
    assert_eq!(c * Z::UNITY, c);
}

#[test]
fn sqr() {
    assert_eq!(Z::new(-1).square(), Z::new(1));
    assert_eq!(Z::new(0).square(), Z::new(0));
    assert_eq!(Z::new(1).square(), Z::new(1));
}

#[test]
fn sub() {
    let a = Z::from(-1045);
    let b = Z::from(32750);
    let c = Z::from(31742);
    let d = Z::from(-31742);
    assert_eq!(a - b, c);
    assert_eq!(b - a, d);
    assert_eq!(c - Z::ZERO, c);
    assert_eq!(Z::ZERO - Z::ZERO, Z::ZERO);
    assert_eq!(Z::UNITY - Z::UNITY, Z::ZERO);
}

#[test]
fn inv() {
    let a = Z::from(24);
    let b = Z::from(19115);
    let c = Z::from(-25);
    let d = Z::from(-5243);
    assert_eq!(b.inv().unwrap(), a);
    assert_eq!(a.inv().unwrap(), b);
    assert_eq!(c.inv().unwrap(), d);
    assert_eq!(d.inv().unwrap(), c);
    assert_eq!(Z::ZERO.inv(), None);
}

#[test]
fn infinity_norm() {
    let a = Z::from(-30000);
    let b = Z::from(30000);
    let nb = 30000;
    let ng = 30001;
    assert!(!a.check_infinity_norm(nb));
    assert!(a.check_infinity_norm(ng));
    assert!(!b.check_infinity_norm(nb));
    assert!(b.check_infinity_norm(ng));
}

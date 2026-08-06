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

use blacknet_crypto::algebra::{Double, Inv, One, Square, Zero};
use blacknet_crypto::gf2::GF2;

type F = blacknet_crypto::gf2::RijndaelField;

#[test]
fn scalar() {
    let a = F::new(0xE6);
    assert_eq!(a * GF2::ONE, a);
    assert_eq!(a * GF2::ZERO, F::ZERO);
    assert_eq!((a / GF2::ONE).unwrap(), a);
    assert!((a / GF2::ZERO).is_none());
    assert_eq!(F::from(GF2::ONE), F::ONE);
    assert_eq!(F::from(GF2::ZERO), F::ZERO);
}

#[test]
fn add() {
    let a = F::new(0xFA);
    let b = F::new(0x7C);
    let c = F::new(0x86);
    assert_eq!(a + b, c);
    assert_eq!(b + a, c);
    assert_eq!(c + F::ZERO, c);
    assert_eq!(F::ZERO + c, c);
    assert_eq!(F::ONE + F::ZERO, F::ONE);
    assert_eq!(F::ZERO + F::ONE, F::ONE);
    assert_eq!(F::ONE + (-F::ONE), F::ZERO);
}

#[test]
fn dbl() {
    let a = F::new(0x85);
    assert_eq!(a.double(), F::ZERO);
    assert_eq!(F::ZERO.double(), F::ZERO);
    assert_eq!(F::ONE.double(), F::ZERO);
}

#[test]
fn neg() {
    let a = F::new(0xF7);
    assert_eq!(-a, a);
    assert_eq!(-F::ZERO, F::ZERO);
}

#[test]
fn sub() {
    let a = F::new(0xD1);
    let b = F::new(0xBD);
    let c = F::new(0x6C);
    assert_eq!(a - b, c);
    assert_eq!(b - a, c);
    assert_eq!(c - F::ZERO, c);
    assert_eq!(F::ZERO - F::ZERO, F::ZERO);
    assert_eq!(F::ONE - F::ONE, F::ZERO);
}

#[test]
fn mul() {
    let a = F::new(0xCA);
    let b = F::new(0x75);
    let c = F::new(0xA3);
    assert_eq!(a * b, c);
    assert_eq!(b * a, c);
    assert_eq!(c * F::ZERO, F::ZERO);
    assert_eq!(F::ZERO * c, F::ZERO);
    assert_eq!(F::ONE * c, c);
    assert_eq!(c * F::ONE, c);
}

#[test]
fn sqr() {
    let a = F::new(0x64);
    let b = F::new(0xD7);
    assert_eq!(a.square(), b);
    assert_eq!(F::ZERO.square(), F::ZERO);
    assert_eq!(F::ONE.square(), F::ONE);
}

#[test]
fn inv() {
    let a = F::new(0x91);
    let b = F::new(0x6A);
    assert_eq!(b.inv().unwrap(), a);
    assert_eq!(a.inv().unwrap(), b);
    assert_eq!(F::ONE.inv().unwrap(), F::ONE);
    assert!(F::ZERO.inv().is_none());
}

#[test]
fn div() {
    let a = F::new(0x31);
    let b = F::new(0xDD);
    let c = F::new(0xD3);
    let d = F::new(0x63);
    assert_eq!((a / b).unwrap(), c);
    assert_eq!((b / a).unwrap(), d);
    assert_eq!((-F::ONE / -F::ONE).unwrap(), F::ONE);
    assert!((F::ONE / F::ZERO).is_none());
}

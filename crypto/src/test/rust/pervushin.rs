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
use blacknet_crypto::ring::{IntegerRing, Ring, UnitalRing};

type Z = blacknet_crypto::pervushin::PervushinField;

#[test]
fn representative() {
    let a = Z::new(-1);
    let b = Z::new(2305843009213693950);
    assert_eq!(b, a);
    assert_eq!(a.canonical(), 2305843009213693950);
    assert_eq!(b.canonical(), 2305843009213693950);
    assert_eq!(a.balanced(), -1);
    assert_eq!(b.balanced(), -1);
    assert_eq!(a.absolute(), 1);
    assert_eq!(b.absolute(), 1);
}

#[test]
fn add() {
    let a = Z::new(1152921504606846974);
    let b = Z::new(1152921504606846970);
    let c = Z::new(-7);
    assert_eq!(a + b, c);
    assert_eq!(b + a, c);
    assert_eq!(c + Z::ZERO, c);
    assert_eq!(Z::ZERO + c, c);
    assert_eq!(Z::UNITY + Z::ZERO, Z::UNITY);
    assert_eq!(Z::ZERO + Z::UNITY, Z::UNITY);
    assert_eq!(Z::new(1) + Z::new(-1), Z::ZERO);
}

#[test]
fn neg() {
    let a = Z::new(4);
    let b = Z::new(-4);
    assert_eq!(-a, b);
    assert_eq!(-b, a);
}

#[test]
fn sub() {
    let a = Z::new(-2048);
    let b = Z::new(65536);
    let c = Z::new(-67584);
    let d = Z::new(67584);
    assert_eq!(a - b, c);
    assert_eq!(b - a, d);
    assert_eq!(c - Z::ZERO, c);
    assert_eq!(Z::ZERO - Z::ZERO, Z::ZERO);
    assert_eq!(Z::UNITY - Z::UNITY, Z::ZERO);
}

#[test]
fn mul() {
    let a = Z::new(1152102451225612864);
    let b = Z::new(-32);
    let c = Z::new(26209708199491568);
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
fn inv() {
    let a = Z::new(24);
    let b = Z::new(-672537544353994069);
    let c = Z::new(-25);
    let d = Z::new(92233720368547758);
    assert_eq!(b.inv().unwrap(), a);
    assert_eq!(a.inv().unwrap(), b);
    assert_eq!(c.inv().unwrap(), d);
    assert_eq!(d.inv().unwrap(), c);
    assert_eq!(Z::ZERO.inv(), None);
}

#[test]
fn infinity_norm() {
    let a = Z::new(-677133638855483916);
    let b = Z::new(1140329745848183219);
    let ab = 677133638855483916;
    let ag = 677133638855483917;
    let bb = 1140329745848183219;
    let bg = 1140329745848183220;
    assert!(!a.check_infinity_norm(ab));
    assert!(a.check_infinity_norm(ag));
    assert!(!b.check_infinity_norm(bb));
    assert!(b.check_infinity_norm(bg));
}

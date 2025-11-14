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

use blacknet_crypto::norm::InfinityNorm;
use blacknet_crypto::operation::{Inv, Square};
use blacknet_crypto::ring::{IntegerRing, PowerOfTwoCyclotomicRing, Ring, UnitalRing};

type Z = blacknet_crypto::pervushin::PervushinField;
type F = blacknet_crypto::pervushin::PervushinField2;

#[test]
fn z_representative() {
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
fn z_add() {
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
fn z_neg() {
    let a = Z::new(4);
    let b = Z::new(-4);
    assert_eq!(-a, b);
    assert_eq!(-b, a);
}

#[test]
fn z_sub() {
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
fn z_mul() {
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
fn z_sqr() {
    assert_eq!(Z::new(-1).square(), Z::new(1));
    assert_eq!(Z::new(0).square(), Z::new(0));
    assert_eq!(Z::new(1).square(), Z::new(1));
}

#[test]
fn z_inv() {
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
fn z_infinity_norm() {
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

#[test]
fn f_add() {
    let a = F::from([4, 3].map(Z::new));
    let b = F::from([2, 1].map(Z::new));
    let c = F::from([6, 4].map(Z::new));
    assert_eq!(a + b, c);
    assert_eq!(b + a, c);
}

#[test]
fn f_mul() {
    let a = F::from([-562956929497444169, 136532190776072177].map(Z::new));
    let b = Z::new(51280928868087145);
    let c = F::from([-557186355960048698, -800938371403945454].map(Z::new));
    let d = F::from([483463506662809566, -624462247079014308].map(Z::new));
    assert_eq!(a * b, c);
    //assert_eq!(b * a, c);
    assert_eq!(a * c, d);
    assert_eq!(c * a, d);
}

#[test]
fn f_inv() {
    let a = F::from([-355525067034500326, -826748688154628891].map(Z::new));
    let b = F::from([654336260586812980, -209289517407125934].map(Z::new));
    assert_eq!(b.inv().unwrap(), a);
    assert_eq!(a.inv().unwrap(), b);
    assert_eq!(F::ZERO.inv(), None);
}

#[test]
fn f_cnj() {
    let a = F::from([4, 0].map(Z::new));
    let b = F::from([654336260586812980, -209289517407125934].map(Z::new));
    let c = F::from([654336260586812980, 209289517407125934].map(Z::new));
    assert_eq!(a.conjugate(), a);
    assert_eq!(b.conjugate(), c);
}

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

use blacknet_crypto::algebra::{
    BalancedRepresentative, Conjugate, IntegerModRing, Inv, One, PolynomialRing, Sqrt, Square, Zero,
};
use blacknet_crypto::norm::InfinityNorm;

type Z = blacknet_crypto::pervushin::PervushinField;
type F = blacknet_crypto::pervushin::PervushinField2;

#[test]
fn z_representative() {
    let a = Z::with_int(-1);
    let b = Z::with_int(2305843009213693950);
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
    let a = Z::with_int(1152921504606846974);
    let b = Z::with_int(1152921504606846970);
    let c = Z::with_int(-7);
    assert_eq!(a + b, c);
    assert_eq!(b + a, c);
    assert_eq!(c + Z::ZERO, c);
    assert_eq!(Z::ZERO + c, c);
    assert_eq!(Z::ONE + Z::ZERO, Z::ONE);
    assert_eq!(Z::ZERO + Z::ONE, Z::ONE);
    assert_eq!(Z::ONE + (-Z::ONE), Z::ZERO);
}

#[test]
fn z_neg() {
    let a = Z::from(4);
    let b = Z::from(-4);
    assert_eq!(-a, b);
    assert_eq!(-b, a);
    assert_eq!(-Z::ZERO, Z::ZERO);
}

#[test]
fn z_sub() {
    let a = Z::from(-2048);
    let b = Z::from(65536);
    let c = Z::from(-67584);
    let d = Z::from(67584);
    assert_eq!(a - b, c);
    assert_eq!(b - a, d);
    assert_eq!(c - Z::ZERO, c);
    assert_eq!(Z::ZERO - Z::ZERO, Z::ZERO);
    assert_eq!(Z::ONE - Z::ONE, Z::ZERO);
}

#[test]
fn z_mul() {
    let a = Z::with_int(1152102451225612864);
    let b = Z::with_int(-32);
    let c = Z::with_int(26209708199491568);
    assert_eq!(a * b, c);
    assert_eq!(b * a, c);
    assert_eq!(c * Z::ZERO, Z::ZERO);
    assert_eq!(Z::ZERO * c, Z::ZERO);
    assert_eq!(Z::ONE * c, c);
    assert_eq!(c * Z::ONE, c);
}

#[test]
fn z_sqr() {
    assert_eq!((-Z::ONE).square(), Z::ONE);
    assert_eq!(Z::ZERO.square(), Z::ZERO);
    assert_eq!(Z::ONE.square(), Z::ONE);
}

#[test]
fn z_inv() {
    let a = Z::with_int(24);
    let b = Z::with_int(-672537544353994069);
    let c = Z::with_int(-25);
    let d = Z::with_int(92233720368547758);
    assert_eq!(b.inv().unwrap(), a);
    assert_eq!(a.inv().unwrap(), b);
    assert_eq!(c.inv().unwrap(), d);
    assert_eq!(d.inv().unwrap(), c);
    assert!(Z::ZERO.inv().is_none());
}

#[test]
fn z_div() {
    let a = Z::with_int(0x1C31);
    let b = Z::with_int(0xFCDD);
    let c = Z::with_int(0x619A2686D8FA9A1);
    let d = Z::with_int(0x1BCACAD3E896AF31);
    assert_eq!((a / b).unwrap(), c);
    assert_eq!((b / a).unwrap(), d);
    assert_eq!((-Z::ONE / -Z::ONE).unwrap(), Z::ONE);
    assert!((Z::ONE / Z::ZERO).is_none());
}

#[test]
fn z_sqrt() {
    let a = Z::with_int(0xC8F7B5AA744AF1);
    let b = Z::with_int(0x121B318906FE12B);
    let c = Z::with_int(0x2475A9E305021CF);
    assert_eq!(a.sqrt().unwrap(), c);
    assert!(b.sqrt().is_none());
    assert_eq!(Z::ZERO.sqrt().unwrap(), Z::ZERO);
    assert_eq!(Z::ONE.sqrt().unwrap(), Z::ONE);
}

#[test]
fn z_sum() {
    let a = [-1, -2, -3, -4, 11].map(Z::from);
    assert_eq!(a.into_iter().sum::<Z>(), Z::ONE);
}

#[test]
fn z_infinity_norm() {
    let a = Z::with_int(-677133638855483916);
    let b = Z::with_int(1140329745848183219);
    let ab = 677133638855483916;
    let ag = 677133638855483917;
    let bb = 1140329745848183219;
    let bg = 1140329745848183220;
    assert!(!a.check_infinity_norm(&ab));
    assert!(a.check_infinity_norm(&ag));
    assert!(!b.check_infinity_norm(&bb));
    assert!(b.check_infinity_norm(&bg));
}

#[test]
fn f_add() {
    let a = F::from([4, 3].map(Z::from));
    let b = F::from([2, 1].map(Z::from));
    let c = F::from([6, 4].map(Z::from));
    assert_eq!(a + b, c);
    assert_eq!(b + a, c);
}

#[test]
fn f_mul() {
    let a = F::from([-562956929497444169, 136532190776072177].map(Z::with_int));
    let b = Z::with_int(51280928868087145);
    let c = F::from([-557186355960048698, -800938371403945454].map(Z::with_int));
    let d = F::from([483463506662809566, -624462247079014308].map(Z::with_int));
    assert_eq!(a * b, c);
    //assert_eq!(b * a, c);
    assert_eq!(a * c, d);
    assert_eq!(c * a, d);
}

#[test]
fn f_inv() {
    let a = F::from([-355525067034500326, -826748688154628891].map(Z::with_int));
    let b = F::from([654336260586812980, -209289517407125934].map(Z::with_int));
    assert_eq!(b.inv().unwrap(), a);
    assert_eq!(a.inv().unwrap(), b);
    assert!(F::ZERO.inv().is_none());
}

#[test]
fn f_cnj() {
    let a = F::from([4, 0].map(Z::with_int));
    let b = F::from([654336260586812980, -209289517407125934].map(Z::with_int));
    let c = F::from([654336260586812980, 209289517407125934].map(Z::with_int));
    assert_eq!(a.conjugate(), a);
    assert_eq!(b.conjugate(), c);
}

#[test]
fn f_evl() {
    let a = F::from([3, 19].map(Z::from));
    let b = Z::from(2);
    let c = Z::from(41);
    assert_eq!(a.evaluate(&b), c);
}

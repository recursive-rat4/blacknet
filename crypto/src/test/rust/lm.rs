/*
 * Copyright (c) 2025 Pavel Vasin
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

use blacknet_crypto::magma::{AdditiveMagma, Inv, MultiplicativeMagma};
use blacknet_crypto::ring::{IntegerRing, Ring, UnitalRing};

type Z = blacknet_crypto::lm::LMField;
type F2 = blacknet_crypto::lm::LMField2;

#[test]
fn z_representative() {
    let a = Z::new(-1);
    let b = Z::new(1152921504606847008);
    assert_eq!(b, a);
    assert_eq!(a.canonical(), 1152921504606847008);
    assert_eq!(b.canonical(), 1152921504606847008);
    assert_eq!(a.balanced(), -1);
    assert_eq!(b.balanced(), -1);
    assert_eq!(a.absolute(), 1);
    assert_eq!(b.absolute(), 1);
}

#[test]
fn z_add() {
    let a = Z::new(0x0A5F69B110721F52);
    let b = Z::new(0x031D58AA351F29B8);
    let c = Z::new(0x0D7CC25B4591490A);
    assert_eq!(a + b, c);
    assert_eq!(b + a, c);
    assert_eq!(Z::new(0) + c, c);
    assert_eq!(c + Z::new(0), c);
    assert_eq!(Z::new(1) + Z::new(0), Z::new(1));
    assert_eq!(Z::new(0) + Z::new(1), Z::new(1));
    assert_eq!(Z::new(-1) + Z::new(1), Z::new(0));
}

#[test]
fn z_dbl() {
    let a = Z::new(0x06D2A9446D1F8F70);
    let b = Z::new(0x0DA55288DA3F1EE0);
    assert_eq!(a.double(), b);
    assert_eq!(Z::new(0).double(), Z::new(0));
}

#[test]
fn z_mul() {
    let a = Z::new(0x099615CDE000EBE6);
    let b = Z::new(-32);
    let c = Z::new(0x0D3D4643FFE285D4);
    assert_eq!(a * b, c);
    assert_eq!(b * a, c);
    assert_eq!(Z::new(0) * c, Z::new(0));
    assert_eq!(c * Z::new(0), Z::new(0));
    assert_eq!(c * Z::new(1), c);
    assert_eq!(Z::new(1) * c, c);
}

#[test]
fn z_sqr() {
    let a = Z::new(0x0787B7C32E50965F);
    let b = Z::new(0x0360B621DB30D71F);
    assert_eq!(a.square(), b);
    assert_eq!(Z::new(0).square(), Z::new(0));
    assert_eq!(Z::new(1).square(), Z::new(1));
}

#[test]
fn z_sub() {
    let a = Z::new(-2048);
    let b = Z::new(65536);
    let c = Z::new(-67584);
    let d = Z::new(67584);
    assert_eq!(a - b, c);
    assert_eq!(b - a, d);
    assert_eq!(c - Z::new(0), c);
    assert_eq!(Z::new(0) - Z::new(0), Z::new(0));
    assert_eq!(Z::new(1) - Z::new(1), Z::new(0));
}

#[test]
fn z_inv() {
    let a = Z::new(24);
    let b = Z::new(-48038396025285292);
    let c = Z::new(-25);
    let d = Z::new(645636042579834325);
    assert_eq!(b.inv().unwrap(), a);
    assert_eq!(a.inv().unwrap(), b);
    assert_eq!(d.inv().unwrap(), c);
    assert_eq!(c.inv().unwrap(), d);
    assert!(Z::new(0).inv().is_none());
}

#[test]
fn z_neg() {
    let a = Z::new(90908245220660597);
    let b = Z::new(-90908245220660597);
    let c = Z::new(-700200562559151818);
    let d = Z::new(700200562559151818);
    assert_eq!(-a, b);
    assert_eq!(-c, d);
    assert_eq!(-Z::new(0), Z::new(0));
    assert_eq!(-(-Z::new(1)), Z::new(1));
}

#[test]
fn f2_add() {
    let a = F2::from([0x04704AB88E022F0F, 0x0F72F4AF92783A07].map(Z::new));
    let b = F2::from([0x05941B02585DC435, 0x0468CBAF2A310378].map(Z::new));
    let c = F2::from([0x0A0465BAE65FF344, 0x03DBC05EBCA93D5E].map(Z::new));
    assert_eq!(a + b, c);
    assert_eq!(b + a, c);
    assert_eq!(F2::ZERO + c, c);
    assert_eq!(c + F2::ZERO, c);
    assert_eq!(F2::UNITY + F2::ZERO, F2::UNITY);
    assert_eq!(F2::ZERO + F2::UNITY, F2::UNITY);
}

#[test]
fn f2_dbl() {
    let a = F2::from([0x0DA509CE5C2E447D, 0x0CF93F1D19DA1831].map(Z::new));
    let b = F2::from([0x0B4A139CB85C88D9, 0x09F27E3A33B43041].map(Z::new));
    assert_eq!(a.double(), b);
    assert_eq!(F2::ZERO.double(), F2::ZERO);
}

#[test]
fn f2_mul() {
    let a = F2::from([0x078E9867B7449CC5, 0x034BB612DF5FA3BE].map(Z::new));
    let b = Z::new(0x034A9A2A4199777D);
    let c = F2::from([0x0932B97A0E0BC6DF, 0x00570D03D7425AD5].map(Z::new));
    let d = F2::from([0x0089CB9D545B5198, 0x08965B2825649C1C].map(Z::new));
    assert_eq!(a * b, c);
    assert_eq!(a * c, d);
    assert_eq!(c * a, d);
    assert_eq!(F2::ZERO * c, F2::ZERO);
    assert_eq!(c * F2::ZERO, F2::ZERO);
    assert_eq!(c * F2::UNITY, c);
    assert_eq!(F2::UNITY * c, c);
}

#[test]
fn f2_inv() {
    let a = F2::from([0x03D1E71872BE33F1, 0x0417CA10FEEEE2C8].map(Z::new));
    let b = F2::from([0x051326C4E39F112B, 0x04787FAC4F00ADC5].map(Z::new));
    assert_eq!(b.inv().unwrap(), a);
    assert_eq!(a.inv().unwrap(), b);
    assert_eq!(F2::ZERO.inv(), None);
}

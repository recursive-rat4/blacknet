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

type F = blacknet_crypto::gf2::GHashField;

#[test]
fn scalar() {
    let a = F::with_u128(0xF812507661D9A605C6090A27BC6AE5C4);
    assert_eq!(a * GF2::ONE, a);
    assert_eq!(a * GF2::ZERO, F::ZERO);
    assert_eq!((a / GF2::ONE).unwrap(), a);
    assert!((a / GF2::ZERO).is_none());
    assert_eq!(F::from(GF2::ONE), F::ONE);
    assert_eq!(F::from(GF2::ZERO), F::ZERO);
}

#[test]
fn add() {
    let a = F::with_u128(0x96918A423D5E9A6EDF674F0BA88AA968);
    let b = F::with_u128(0xCBEC169DDF9EB4F32DF1A55698C8D742);
    let c = F::with_u128(0x5D7D9CDFE2C02E9DF296EA5D30427E2A);
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
    let a = F::with_u128(0x4E321C7527B3008AFCC1A5CB28396DA7);
    assert_eq!(a.double(), F::ZERO);
    assert_eq!(F::ZERO.double(), F::ZERO);
    assert_eq!(F::ONE.double(), F::ZERO);
}

#[test]
fn neg() {
    let a = F::with_u128(0xB6DE48AF06C3AF4738B59BDB7CDF5A60);
    assert_eq!(-a, a);
    assert_eq!(-F::ZERO, F::ZERO);
}

#[test]
fn sub() {
    let a = F::with_u128(0x89C413FE5BCBC6906C53D7C2BC532FF6);
    let b = F::with_u128(0x5DC66A9EA3FB0E355090C566AE47519F);
    let c = F::with_u128(0xD4027960F830C8A53CC312A412147E69);
    assert_eq!(a - b, c);
    assert_eq!(b - a, c);
    assert_eq!(c - F::ZERO, c);
    assert_eq!(F::ZERO - F::ZERO, F::ZERO);
    assert_eq!(F::ONE - F::ONE, F::ZERO);
}

#[test]
fn mul() {
    let a = F::with_u128(0xDD9373160F5532A91E42EF2AB01C2F6B);
    let b = F::with_u128(0x74C76A84648E8C7F0649EDE2EB8329FB);
    let c = F::with_u128(0x89358C4BB668B483FD97FE90644A3804);
    assert_eq!(a * b, c);
    assert_eq!(b * a, c);
    assert_eq!(c * F::ZERO, F::ZERO);
    assert_eq!(F::ZERO * c, F::ZERO);
    assert_eq!(F::ONE * c, c);
    assert_eq!(c * F::ONE, c);
}

#[test]
fn sqr() {
    let a = F::with_u128(0x24DE30CDE2FFBA208C6CB2A41B850788);
    let b = F::with_u128(0x540908FEDE2DDD0CAF72401978CB5D4E);
    assert_eq!(a.square(), b);
    assert_eq!(F::ZERO.square(), F::ZERO);
    assert_eq!(F::ONE.square(), F::ONE);
}

#[test]
fn inv() {
    let a = F::with_u128(0x6D8880AEBA50B29593B30D9AE5A83407);
    let b = F::with_u128(0x60D93FBC77E954B7F031EC49A27CA394);
    assert_eq!(b.inv().unwrap(), a);
    assert_eq!(a.inv().unwrap(), b);
    assert_eq!(F::ONE.inv().unwrap(), F::ONE);
    assert!(F::ZERO.inv().is_none());
}

#[test]
fn div() {
    let a = F::with_u128(0x581742999BC333C1CAAE4120BB935050);
    let b = F::with_u128(0x2F55B5FAA8C4F4CB0B2C108DA8CD9327);
    let c = F::with_u128(0xACA77D521D76FE7DE49728A0AC044475);
    let d = F::with_u128(0xC54D41A96466AFE9522C5DB55963C4FC);
    assert_eq!((a / b).unwrap(), c);
    assert_eq!((b / a).unwrap(), d);
    assert_eq!((-F::ONE / -F::ONE).unwrap(), F::ONE);
    assert!((F::ONE / F::ZERO).is_none());
}

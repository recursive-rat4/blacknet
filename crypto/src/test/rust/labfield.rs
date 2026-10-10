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

type F = blacknet_crypto::gf2::LabField;

#[test]
fn scalar() {
    let a = F::new([0x43D6B58F3A23D2B0, 0x023D7755110A3ACC, 0x0055CD12A]).unwrap();
    assert_eq!(a * GF2::ONE, a);
    assert_eq!(a * GF2::ZERO, F::ZERO);
    assert_eq!((a / GF2::ONE).unwrap(), a);
    assert!((a / GF2::ZERO).is_none());
    assert_eq!(F::from(GF2::ONE), F::ONE);
    assert_eq!(F::from(GF2::ZERO), F::ZERO);
}

#[test]
fn add() {
    let a = F::new([0x9F743C10B62BA000, 0x0730FFF36123A5A4, 0x2905F1701]).unwrap();
    let b = F::new([0x46258292CA77DBCA, 0xE181EA8A5915FD77, 0x30E241137]).unwrap();
    let c = F::new([0xD951BE827C5C7BCA, 0xE6B11579383658D3, 0x19E7B0636]).unwrap();
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
    let a = F::new([0x43D6B58F3A23D2B0, 0x023D7755110A3ACC, 0x0055CD12A]).unwrap();
    assert_eq!(a.double(), F::ZERO);
    assert_eq!(F::ZERO.double(), F::ZERO);
    assert_eq!(F::ONE.double(), F::ZERO);
}

#[test]
fn neg() {
    let a = F::new([0x43D6B58F3A23D2B0, 0x023D7755110A3ACC, 0x0055CD12A]).unwrap();
    assert_eq!(-a, a);
    assert_eq!(-F::ZERO, F::ZERO);
}

#[test]
fn sub() {
    let a = F::new([0x9F743C10B62BA000, 0x0730FFF36123A5A4, 0x2905F1701]).unwrap();
    let b = F::new([0x46258292CA77DBCA, 0xE181EA8A5915FD77, 0x30E241137]).unwrap();
    let c = F::new([0xD951BE827C5C7BCA, 0xE6B11579383658D3, 0x19E7B0636]).unwrap();
    assert_eq!(a - b, c);
    assert_eq!(b - a, c);
    assert_eq!(c - F::ZERO, c);
    assert_eq!(F::ZERO - F::ZERO, F::ZERO);
    assert_eq!(F::ONE - F::ONE, F::ZERO);
}

#[test]
fn mul() {
    let a = F::new([0xA4DB82A8825D842D, 0xA061496189B968CC, 0x3A3405450]).unwrap();
    let b = F::new([0xD8150B5BF7FB5D9D, 0x999A573C95E38F60, 0x03EA7A4AE]).unwrap();
    let c = F::new([0xBFC02B6DB03C9423, 0xB86C76E0221BF7E7, 0x3036B5CDE]).unwrap();
    assert_eq!(a * b, c);
    assert_eq!(b * a, c);
    assert_eq!(c * F::ZERO, F::ZERO);
    assert_eq!(F::ZERO * c, F::ZERO);
    assert_eq!(F::ONE * c, c);
    assert_eq!(c * F::ONE, c);
}

#[test]
fn sqr() {
    let a = F::new([0x66BE76C2AD705E6D, 0x638AA421517B4513, 0x0C7865C93]).unwrap();
    let b = F::new([0xFD57962A5D34F380, 0x1614CDD49FB70A17, 0x3B033230D]).unwrap();
    assert_eq!(a.square(), b);
    assert_eq!(F::ZERO.square(), F::ZERO);
    assert_eq!(F::ONE.square(), F::ONE);
}

#[test]
fn inv() {
    let a = F::new([0x551BD963E64FB9BB, 0xE88864C7927D6E8B, 0x07472EDFB]).unwrap();
    let b = F::new([0xF7FBD766161C77F4, 0xC6B0E192E2ECC513, 0x365D4320B]).unwrap();
    assert_eq!(b.inv().unwrap(), a);
    assert_eq!(a.inv().unwrap(), b);
    assert_eq!(F::ONE.inv().unwrap(), F::ONE);
    assert!(F::ZERO.inv().is_none());
}

#[test]
fn div() {
    let a = F::new([0x12BF1F49CCFCFF05, 0xE98027BBF691E59D, 0x295EC09F4]).unwrap();
    let b = F::new([0xFCD0BFC1C6386809, 0x4653C2BF2F987E0C, 0x15B38CF00]).unwrap();
    let c = F::new([0x43D6B58F3A23D2B0, 0x023D7755110A3ACC, 0x0055CD12A]).unwrap();
    let d = F::new([0x3795A04C6C7F547C, 0x067C8B0FB1B740EC, 0x0BA7F758D]).unwrap();
    assert_eq!((a / b).unwrap(), c);
    assert_eq!((b / a).unwrap(), d);
    assert_eq!((-F::ONE / -F::ONE).unwrap(), F::ONE);
    assert!((F::ONE / F::ZERO).is_none());
}

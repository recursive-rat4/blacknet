/*
 * Copyright (c) 2025-2026 Pavel Vasin
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
    AdditiveMonoid, BalancedRepresentative, Double, IntegerRing, Inv, MultiplicativeMonoid, Square,
};
use blacknet_crypto::norm::InfinityNorm;

type Z = blacknet_crypto::lm::LMField;
type F2 = blacknet_crypto::lm::LMField2;
type R64 = blacknet_crypto::lm::LMRing64;
type NTT64 = blacknet_crypto::lm::LMNTT64;

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
    assert_eq!(F2::ONE + F2::ZERO, F2::ONE);
    assert_eq!(F2::ZERO + F2::ONE, F2::ONE);
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
    assert_eq!(c * F2::ONE, c);
    assert_eq!(F2::ONE * c, c);
}

#[test]
fn f2_inv() {
    let a = F2::from([0x03D1E71872BE33F1, 0x0417CA10FEEEE2C8].map(Z::new));
    let b = F2::from([0x051326C4E39F112B, 0x04787FAC4F00ADC5].map(Z::new));
    assert_eq!(b.inv().unwrap(), a);
    assert_eq!(a.inv().unwrap(), b);
    assert_eq!(F2::ZERO.inv(), None);
}

#[test]
fn r64_mul() {
    let a_coeffs = [
        0x5b22afe9eb6776e,
        0xfae29b2f98880c,
        0x21bf8c39c5bab3d,
        0x9fe5ca88e9ddd1f,
        0x37fa34fb05c7213,
        0x2cc0d459af2edd5,
        0xf25f432faf7315,
        0xe7d1b87a15e7629,
        0xb6073a48066dc49,
        0x19c266847c7da75,
        0xd93c41fb3e62163,
        0x28fe0202640be94,
        0xcc1dd411bca6a45,
        0x70d2376ee4f7244,
        0xf4aec77a28d2809,
        0x262077bc8398e59,
        0xd4563b660e9acc0,
        0x98a6f928b443ea3,
        0x8d5335fe44efe58,
        0x3c54e153a7d8b4a,
        0x1e86a650147771,
        0xf8e713f0c7a3e71,
        0xd74dd4b5ff493f7,
        0xa5ebf65edfcd8f7,
        0x687b783bee19ea1,
        0x77c4bb4937ff12,
        0x63c6950bd280e84,
        0x65c8ee654a7621d,
        0x84d91f5ae021fae,
        0x3c9fd10c3add658,
        0x467408cbca1fb90,
        0x69ea4dbe6ea33e9,
        0xdd89e8363d5aa34,
        0x3ccce125a9444ef,
        0xd89190254c01038,
        0x8faea5debc1d52e,
        0x1517140e487a69c,
        0xb9939dc7eb1d925,
        0xe96d933a6feaf01,
        0xa69e5ed05efde25,
        0xa05e00382d4f349,
        0x59b2a0f5b96121e,
        0x20d639bb2891d29,
        0x5da583ab7954646,
        0xf28395d37b1f4a6,
        0x8da8f006092175d,
        0xd8020811f4f473b,
        0x60f626dd550c8f7,
        0x13a0a94ecd76469,
        0xd2dcfd917102f7e,
        0xdfed5e9be86587e,
        0x87b838a16bbe030,
        0xafc83e1c63d5529,
        0x5d78dd5db9d0254,
        0x14ceb35d6c147da,
        0x7b2ff5496e675b6,
        0x9fa6ca74b9740ab,
        0x27f27a5e423f819,
        0xc05030084b27ce,
        0xe5c050a00e31636,
        0x3b3933b02ef2cf4,
        0xe46dda99f753705,
        0xe5db38191bc0fad,
        0xfec11594ed14c44,
    ]
    .map(Z::new);

    let b_coeffs = [
        0xc6375069ee50a73,
        0xc38a0e011d589c2,
        0x14cdb939efb45ec,
        0xa66b5690f0cbe11,
        0xb6cef55750ae80a,
        0x5f3418cadf7a213,
        0x370efd53c3584b6,
        0x150b1cde53f3177,
        0xea1028cf225b0b1,
        0xc4b2c5a258f0da,
        0x4b9924defc529b4,
        0x22e60527f0333f5,
        0xc48b1db9afc95bc,
        0x176603e4337f171,
        0x320d490fde2eb52,
        0xef2e2a60a65cad7,
        0x17142333d677fe9,
        0xb4369e9f9079903,
        0xa54cb658037605,
        0x617398bc0826c2f,
        0xd228b7e13b328b5,
        0x6161f650b62807a,
        0x2cae6cdd38d5e1e,
        0x3c0a764546946c8,
        0xc8414356c2fa551,
        0x3549c5f6166848,
        0xe15ae8add214fc4,
        0x8e15b2c8509413,
        0x11ffe950040d123,
        0x37f3459033ec452,
        0xeb8b503255e48e2,
        0x37a5e0c34d42844,
        0x593a6c4ecbdf29f,
        0xef02cf3e616d809,
        0x22b618bb9b6f9bd,
        0x97c74d6527b3d04,
        0xa968a28b2404f68,
        0x76d0b1b5670c79a,
        0xb2353916732c1c,
        0x21a3cc6f6ab8505,
        0x432e909f440d0f7,
        0x8d75c76a0293407,
        0xde9d1ce06a2054e,
        0xc0f10019d671f38,
        0xc61cd78ea70f06f,
        0x3df19bfa9e52fc6,
        0x2075a7da11ce19c,
        0xcb630ae7d08d53c,
        0x9593782b7724ddf,
        0x1556c09d8570d07,
        0xd8e360545b7265a,
        0x7f0e4506da8ec3d,
        0x8d3e563929413ae,
        0x907d7abf9397129,
        0xc67b7ad55afdc18,
        0xb1eec9084aa930,
        0x49e02652b1ded78,
        0x5b68e5e8275a000,
        0x7ede3abe080ca42,
        0xe18616295bd19b5,
        0x8230e58edec225d,
        0x66c726f6a43e0bc,
        0xa1061f2d534374e,
        0x3587728dbd1e121,
    ]
    .map(Z::new);

    let c_coeffs = [
        0xd065f091d6b655e,
        0x2d1aa8baa5e62a5,
        0x72baa936d981a69,
        0x25c4f4367ae603c,
        0xa31f329ba38df24,
        0xb41ba443b054484,
        0xa7bb5f9d204ffa6,
        0x741b75fa9826dab,
        0x31d8816cf178a17,
        0xbfd309997ef77fc,
        0x5221dc0d470325f,
        0x669bb5e022be8a8,
        0x75cf7e9d54b380,
        0x6a019af3cc298a5,
        0xf61e10c0d6adbfe,
        0xeb62bd17cd780b,
        0x7a20d37fcaa14f1,
        0x11146a85cb7d5dd,
        0x629adae3c025d8b,
        0x8b4a1706165d168,
        0x2bb6a0ebdc23509,
        0x1b77df509e3e6ae,
        0x8f9046809005d62,
        0xeedaf63d3515732,
        0x8ba937cb5a96a5f,
        0x78c1386a4ed4344,
        0x6aa49969d070840,
        0xa7c5daec1343292,
        0xa9a8b025d71b191,
        0xd19100c8690eea7,
        0xab6f8f72ab5fc1a,
        0xba237c4caeefcb4,
        0x8e85ac4051d01db,
        0x8d3ea7d4f33b14a,
        0x9345b4b6988e304,
        0x9a97218260bec6f,
        0x3832b00fa972f86,
        0x1a378c53832adc8,
        0x88be4b03e25cb56,
        0x1f26917acacd1ee,
        0x40300d957dde48,
        0x8d1868998527af1,
        0x2eceb68a356bfda,
        0xdc9b88d90b4fb82,
        0xa73d532f22c3498,
        0xd5d75fd2b4bb002,
        0x9668cfa95978d9f,
        0xdfbe72dae5e53e1,
        0x757f2140b5c40f,
        0x762ba90021e8eeb,
        0x9f10b89ab3cce82,
        0xbafc1437653f868,
        0x65b3a386a64dee2,
        0xd96ac0481d40b63,
        0xc365d4f2acc9063,
        0x69e43515bfb25dc,
        0x35154921271f4c0,
        0x12dc54723e89e3f,
        0xba8d5408b216954,
        0xfa3d59df6d1c003,
        0xbb0627456274297,
        0x85cec8684b11179,
        0x6624fec11a68d59,
        0xe173e51e7e566cc,
    ]
    .map(Z::new);

    let a = R64::from(a_coeffs);
    let b = R64::from(b_coeffs);
    let c = R64::from(c_coeffs);

    assert_eq!(a * b, c);
    assert_eq!(R64::ONE * c, c);
    assert_eq!(c * Z::ZERO, R64::ZERO);

    let a_ntt = NTT64::from(a_coeffs);
    let b_ntt = NTT64::from(b_coeffs);
    let c_ntt = NTT64::from(c_coeffs);

    assert_eq!(a_ntt * b_ntt, c_ntt);
    assert_eq!(NTT64::ONE * c_ntt, c_ntt);
    assert_eq!(c_ntt * Z::ZERO, NTT64::ZERO);
}

#[test]
fn r64_infinity_norm() {
    let mut coeffs = [Z::ZERO; 64];
    coeffs[..8].copy_from_slice(&[-67133638855483916, 6, 5, 4, 3, 2, 1, 0].map(Z::new));
    let bad = 67133638855483916;
    let good = 67133638855483917;

    let elt = R64::from(coeffs);
    assert!(!elt.check_infinity_norm(bad));
    assert!(elt.check_infinity_norm(good));

    let elt = NTT64::from(coeffs);
    assert!(!elt.check_infinity_norm(bad));
    assert!(elt.check_infinity_norm(good));
}

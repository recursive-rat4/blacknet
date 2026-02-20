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

use blacknet_crypto::algebra::{IntegerRing, Inv, One, Sqrt, Square, Zero};
use blacknet_crypto::bigint::UInt256;

type F = blacknet_crypto::ed25519::Scalar25519;

#[test]
fn representative() {
    let a = UInt256::from(1);
    let b = UInt256::from_hex("1000000000000000000000000000000014DEF9DEA2F79CD65812631A5CF5D3EC");
    let c = F::from(-1);
    let d = F::new(b);
    assert_eq!(d, c);
    assert_eq!(c.canonical(), b);
    assert_eq!(d.canonical(), b);
    assert_eq!(c.absolute(), a);
    assert_eq!(d.absolute(), a);
}

#[test]
fn add() {
    let a = F::from_hex("02BFBE818DD7D8205D034DA6AE4E081A918B213E57431394307A35AAD416B5F4");
    let b = F::from_hex("0B4C5CDCFACDB4BECFBF827D8143E65BC7C9CB9FF3121A264A4EEFB69D26BE3D");
    let c = F::from_hex("0E0C1B5E88A58CDF2CC2D0242F91EE765954ECDE4A552DBA7AC92561713D7431");
    assert_eq!(a + b, c);
    assert_eq!(b + a, c);
    assert_eq!(F::ZERO + c, c);
    assert_eq!(c + F::ZERO, c);
    assert_eq!(F::ONE + F::ZERO, F::ONE);
    assert_eq!(F::ZERO + F::ONE, F::ONE);
}

#[test]
fn neg() {
    let a = F::from_hex("02610BC44A0BBC319A91FC24E99A98EF16F3A04CB06420441CA9ACF00C98610D");
    let b = F::from_hex("0D9EF43BB5F443CE656E03DB16656710FDEB5991F2937C923B68B62A505D72E0");
    assert_eq!(-a, b);
    assert_eq!(-b, a);
    assert_eq!(-(-F::ONE), F::ONE);
    assert_eq!(-F::ZERO, F::ZERO);
}

#[test]
fn sub() {
    let a = F::from_hex("063C6FA6BC7FD187EE00659A73A97B1589892A4AE753FE00C7B3764DDD663CD2");
    let b = F::from_hex("00AC2A42B38F940E1D6D81E7B258588BDAF0EF33D488F4AAE00E99E546F35F56");
    let c = F::from_hex("0590456408F03D79D092E3B2C1512289AE983B1712CB0955E7A4DC689672DD7C");
    let d = F::from_hex("0A6FBA9BF70FC2862F6D1C4D3EAEDD766646BEC7902C9380706D86B1C682F671");
    assert_eq!(a - b, c);
    assert_eq!(b - a, d);
    assert_eq!(c - F::ZERO, c);
    assert_eq!(F::ZERO - F::ZERO, F::ZERO);
    assert_eq!(F::ONE - F::ONE, F::ZERO);
}

#[test]
fn mul() {
    let a = F::from_hex("0CFC44A5ED3B23B2EE255AAB66FAEDFB186F5EB206482AB56217E1034942E5E9");
    let b = F::from_hex("0CF69A2302FB516F08473812AFAA2A97F93A40E9B756FF5303236A699D1D08EF");
    let c = F::from_hex("0037258828ECD41E1F2511DD82A479AB6902AF77B5CAF50FA9987E204F6088F4");
    assert_eq!(a * b, c);
    assert_eq!(b * a, c);
    assert_eq!(F::ZERO * c, F::ZERO);
    assert_eq!(c * F::ZERO, F::ZERO);
    assert_eq!(c * F::ONE, c);
    assert_eq!(F::ONE * c, c);
}

#[test]
fn div() {
    let a = F::from_hex("000932DBED8AF1F02DEAF2554493BA7B62C2AC0A781BA33D96C560C2307366E9");
    let b = F::from_hex("052D43A9A19991AA7F8C98ED185A79ED94D35C4FA94DCEDEFCAE5DBA73405517");
    let c = F::from_hex("01A08079F6CD3675B821D6B8CF7358D7D3087B11D06000CDC6E60B034D28B6D6");
    assert_eq!((a / b).unwrap(), c);
    assert_eq!((F::ZERO / c).unwrap(), F::ZERO);
    assert!((c / F::ZERO).is_none());
    assert_eq!((F::ONE / F::ONE).unwrap(), F::ONE);
    assert_eq!((c / F::ONE).unwrap(), c);
}

#[test]
fn sqr() {
    let a = F::from_hex("066420CC4B01B39FB5BFB8E96A974086376203FAD307883583C525DC9E633C69");
    let b = F::from_hex("0D09F14F890C7B5F433F7E63F017420C2705AFBF0739916416849BF60EB91D45");
    assert_eq!(a.square(), b);
    assert_eq!(F::ZERO.square(), F::ZERO);
    assert_eq!(F::ONE.square(), F::ONE);
}

#[test]
fn inv() {
    let a = F::from_hex("065B26C5DF99C34AEB2B18548E7B23054FC2052FCCC2D167066927F97588876F");
    let b = F::from_hex("0DC021B2AACBD0ECAFBAED2F984CACEE430324FE9056DA2D133C759A293F9587");
    assert_eq!(b.inv().unwrap(), a);
    assert_eq!(a.inv().unwrap(), b);
    assert!(F::ZERO.inv().is_none());
}

#[test]
fn sqrt() {
    let a = F::from_hex("08938AB74D9B6E57EB0DF50C3E9EF34EE99C5A6B441A47A19BBB56248FAB4515");
    let b = F::from_hex("08938AB74D9B6E57EB0DF50C3E9EF34EE99C5A6B441A47A19BBB56248FAB4514");
    let c = F::from_hex("045762A67565A10F3DB6E73E69AB0F94D7521E98371C9E7B70F2DE0317F82126");
    assert_eq!(a.sqrt().unwrap(), c);
    assert!(b.sqrt().is_none());
    assert_eq!(F::ZERO.sqrt().unwrap(), F::ZERO);
    assert_eq!(F::ONE.sqrt().unwrap(), F::ONE);
}

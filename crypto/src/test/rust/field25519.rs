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

use blacknet_crypto::algebra::IntegerRing;
use blacknet_crypto::algebra::{Inv, Square};
use blacknet_crypto::algebra::{Presemiring, Semiring};
use blacknet_crypto::bigint::UInt256;

type F = blacknet_crypto::field25519::Field25519;

#[test]
fn representative() {
    let a = UInt256::from(1);
    let b = UInt256::from_hex("7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEC");
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
    let a = F::from_hex("22BFBE818DD7D8205D034DA6AE4E081ABB4914FB9D324D40E09EFBDF8E025DCE");
    let b = F::from_hex("0B4C5CDCFACDB4BECFBF827D8143E65BC7C9CB9FF3121A264A4EEFB69D26BE3D");
    let c = F::from_hex("2E0C1B5E88A58CDF2CC2D0242F91EE768312E09B904467672AEDEB962B291C0B");
    assert_eq!(a + b, c);
    assert_eq!(b + a, c);
    assert_eq!(F::ZERO + c, c);
    assert_eq!(c + F::ZERO, c);
    assert_eq!(F::ONE + F::ZERO, F::ONE);
    assert_eq!(F::ZERO + F::ONE, F::ONE);
}

#[test]
fn neg() {
    let a = F::from_hex("12610BC44A0BBC319A91FC24E99A98EF2BD29A2B535BBD1A74BC100A698E34FA");
    let b = F::from_hex("6D9EF43BB5F443CE656E03DB16656710D42D65D4ACA442E58B43EFF59671CAF3");
    assert_eq!(-a, b);
    assert_eq!(-b, a);
    assert_eq!(-(-F::ONE), F::ONE);
}

#[test]
fn sub() {
    let a = F::from_hex("063C6FA6BC7FD187EE00659A73A97B1589892A4AE753FE00C7B3764DDD663CD2");
    let b = F::from_hex("20AC2A42B38F940E1D6D81E7B258588C04AEE2F11A782E579033601A00DF0730");
    let c = F::from_hex("6590456408F03D79D092E3B2C151228984DA4759CCDBCFA937801633DC87358F");
    let d = F::from_hex("1A6FBA9BF70FC2862F6D1C4D3EAEDD767B25B8A633243056C87FE9CC2378CA5E");
    assert_eq!(a - b, c);
    assert_eq!(b - a, d);
    assert_eq!(c - F::ZERO, c);
    assert_eq!(F::ZERO - F::ZERO, F::ZERO);
    assert_eq!(F::ONE - F::ONE, F::ZERO);
}

#[test]
fn mul() {
    let a = F::from_hex("4CFC44A5ED3B23B2EE255AAB66FAEDFB6BEB462C92269E0EC2616D6CBD1A359D");
    let b = F::from_hex("3CF69A2302FB516F08473812AFAA2A9837D72E85A03DD5D60B5A93B8B3FE84B6");
    let c = F::from_hex("2D3960B5603F737CEC7C3B65CD2524FECD5A4961F4792DF842EF6ABADDD9A09B");
    assert_eq!(a * b, c);
    assert_eq!(b * a, c);
    assert_eq!(F::ZERO * c, F::ZERO);
    assert_eq!(c * F::ZERO, F::ZERO);
    assert_eq!(c * F::ONE, c);
    assert_eq!(F::ONE * c, c);
}

#[test]
fn div() {
    let a = F::from_hex("3FACED132F5641F57B1162D06ED827D8CA9FA69F0C7B14822818EEF4DB6F6FDC");
    let b = F::from_hex("152D43A9A19991AA7F8C98ED185A79EDA9B2562E4C456BB554C0C0D4D0362904");
    let c = F::from_hex("58C4824D139DC383C143CBB9CC8329AEFEE44752E3B33771AD362FE03ACF52A8");
    assert_eq!((a / b).unwrap(), c);
    assert_eq!((F::ZERO / c).unwrap(), F::ZERO);
    assert!((c / F::ZERO).is_none());
    assert_eq!((F::ONE / F::ONE).unwrap(), F::ONE);
    assert_eq!((c / F::ONE).unwrap(), c);
}

#[test]
fn sqr() {
    let a = F::from_hex("38938AB74D9B6E57EB0DF50C3E9EF34F283948072D011E24A3F27F73A68CC0DB");
    let b = F::from_hex("6BF8A5C2B6D265BF399F5BB05C70E62E9DFB403BE7548DB98E1DA13BD6EDC9D9");
    assert_eq!(a.square(), b);
    assert_eq!(F::ZERO.square(), F::ZERO);
    assert_eq!(F::ONE.square(), F::ONE);
}

#[test]
fn inv() {
    let a = F::from_hex("0F34FE2FD15703DC7EBA4A68D48FA9EE0E9AB8746F759EB8FC23828A4AA48900");
    let b = F::from_hex("16CEA88227C9F5A181A5C35996A1DB400D53A6D5B42E33B3CA0CCA7E7C8E27B7");
    assert_eq!(b.inv().unwrap(), a);
    assert_eq!(a.inv().unwrap(), b);
    assert!(F::ZERO.inv().is_none());
}

#[test]
fn sqrt() {
    let a = F::from_hex("35AEB660A5F2E7DF341A8F256036C025E07B8E45002F7D9DA0C8F7B5AA744AEA");
    let b = F::from_hex("39FCE7DBF35569B5DC603860E3254BF9E61E3B57BA958A05A121B318906FE126");
    let c = F::from_hex("729CF1CEDFC54274C1AF22F2B342AC455B664221EEF9C53D24A2F079F7AB16F5");
    assert_eq!(a.sqrt().unwrap(), c);
    assert!(b.sqrt().is_none());
    assert_eq!(F::ZERO.sqrt().unwrap(), F::ZERO);
    assert_eq!(F::ONE.sqrt().unwrap(), F::ONE);
}

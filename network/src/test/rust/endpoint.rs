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

use blacknet_network::endpoint::Endpoint;
use blacknet_serialization::format::{from_bytes, to_bytes, to_size};

#[test]
fn ipv4() {
    let data = [
        ("0.0.0.0", true, false),
        ("100.64.0.0", false, true),
        ("100.128.0.0", false, false),
        ("127.0.1.4", true, false),
        ("255.255.255.255", false, false),
    ];
    for (string, is_local, is_private) in data {
        let endpoint = Endpoint::parse(string, 28453).unwrap();
        assert!(matches!(endpoint, Endpoint::IPv4 { .. }));
        assert!(!endpoint.is_permissionless());
        assert_eq!(endpoint.is_local(), is_local);
        assert_eq!(endpoint.is_private(), is_private);
        assert!(endpoint.to_rust().is_some());
        assert_eq!(endpoint.to_host(), string);
    }
}

#[test]
fn ipv6() {
    let data = [
        ("1001:1001:1001:1001:1001:1001:1001:1001", false, false),
        ("2001:8db8:8558:8888:1331:8aa8:3789:7337", false, false),
        ("f00f:f00f:f00f:f00f:f00f:f00f:f00f:f00f", false, false),
    ];
    for (string, is_local, is_private) in data {
        let endpoint = Endpoint::parse(string, 28453).unwrap();
        assert!(matches!(endpoint, Endpoint::IPv6 { .. }));
        assert!(!endpoint.is_permissionless());
        assert_eq!(endpoint.is_local(), is_local);
        assert_eq!(endpoint.is_private(), is_private);
        assert!(endpoint.to_rust().is_some());
        assert_eq!(endpoint.to_host(), string);
    }
}

#[test]
fn torv3() {
    let data = [
        "pg6mmjiyjmcrsslvykfwnntlaru7p5svn6y2ymmju6nubxndf4pscryd.onion",
        "sp3k262uwy4r2k3ycr5awluarykdpag6a7y33jxop4cs2lu5uz5sseqd.onion",
        "xa4r2iadxm55fbnqgwwi5mymqdcofiu3w6rpbtqn7b2dyn7mgwj64jyd.onion",
    ];
    for string in data {
        let endpoint = Endpoint::parse(string, 28453).unwrap();
        assert!(matches!(endpoint, Endpoint::TORv3 { .. }));
        assert!(endpoint.is_permissionless());
        assert!(!endpoint.is_local());
        assert!(!endpoint.is_private());
        assert!(endpoint.to_rust().is_none());
        assert_eq!(endpoint.to_host(), string);
    }
}

#[test]
fn i2p() {
    let data = ["y45f23mb2apgywmftrjmfg35oynzfwjed7rxs2mh76pbdeh4fatq.b32.i2p"];
    for string in data {
        let endpoint = Endpoint::parse(string, 28453).unwrap();
        assert!(matches!(endpoint, Endpoint::I2P { .. }));
        assert!(endpoint.is_permissionless());
        assert!(!endpoint.is_local());
        assert!(!endpoint.is_private());
        assert!(endpoint.to_rust().is_none());
        assert_eq!(endpoint.to_host(), string);
    }
}

#[test]
fn compare() {
    let a = Endpoint::parse("127.0.0.1", 12345).unwrap();
    let b = Endpoint::parse("127.0.0.2", 12345).unwrap();
    let c = Endpoint::parse(
        "mzgt4svgc72euhvkpfdow7aiiivziqwhsl2fdzgiwkqeronnjjtq.b32.i2p",
        0,
    )
    .unwrap();
    let d = Endpoint::parse(
        "mzgt4svgc72euhvkpfdow7aiiivziqwhsl2fdzgiwkqeronnjjtq.b32.i2p",
        0,
    )
    .unwrap();

    assert_ne!(a, b);
    assert_ne!(b, c);
    assert_eq!(c, d);
    assert_ne!(d, a);
}

#[test]
fn serialization() {
    let endpoint = Endpoint::parse("127.0.0.4", 258).unwrap();
    #[rustfmt::skip]
    let bytes: [u8; 7] = [
        0x80,
        0x01, 0x02,
        0x7F, 0x00, 0x00, 0x04
    ];

    let deserialized = from_bytes::<Endpoint>(&bytes, false).unwrap();
    assert_eq!(deserialized, endpoint);

    let size = to_size(&endpoint).unwrap();
    assert_eq!(size, bytes.len());

    let serialized = to_bytes(&endpoint).unwrap();
    assert_eq!(serialized, bytes);
}

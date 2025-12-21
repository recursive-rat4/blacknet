/*
 * Copyright (c) 2020-2025 Pavel Vasin
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

use blacknet_serialization::format::{from_bytes, to_bytes, to_size};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct Structure {
    boolean1: bool,
    boolean2: bool,
    byte: u8,
    short: u16,
    int: u32,
    long: u64,
    nullable1: Option<()>,
    nullable2: Option<()>,
    string: String,
    nested: Nested,
    list: Vec<u8>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct Nested {
    int: u32,
}

fn structure() -> Structure {
    Structure {
        boolean1: false,
        boolean2: true,
        byte: 0,
        short: 0x01FF,
        int: 0x0201FFFE,
        long: 0x04030201FFFEFDFC,
        nullable1: None,
        nullable2: Some(()),
        string: "八".to_string(),
        nested: Nested { int: 0x0201FFFE },
        list: vec![1, 2, 3, 4],
    }
}

#[rustfmt::skip]
fn bytes() -> Vec<u8> {
    vec![
        0,
        1,
        0,
        1, 255,
        2, 1, 255, 254,
        4, 3, 2, 1, 255, 254, 253, 252,
        0,
        1, // Unit //
        0x83, 0xE5, 0x85, 0xAB,
        2, 1, 255, 254,
        0x84, 0x01, 0x02, 0x03, 0x04,
    ]
}

#[test]
fn deserialize() {
    assert_eq!(
        from_bytes::<Structure>(&bytes(), false).unwrap(),
        structure()
    )
}

#[test]
fn serialize() {
    assert_eq!(to_bytes(&structure()).unwrap(), bytes())
}

#[test]
fn compute_size() {
    assert_eq!(to_size(&structure()).unwrap(), bytes().len());
}

#[test]
fn invalid_mark() {
    let bytes = vec![2];
    assert!(from_bytes::<bool>(&bytes, true).is_err());
    assert!(from_bytes::<Option<()>>(&bytes, true).is_err());
}

#[test]
fn invalid_size() {
    let bytes = bytes();
    let bytes1 = &bytes[..bytes.len() - 1];
    let mut bytes2 = bytes.clone();
    bytes2.extend_from_slice(&[1]);
    assert!(from_bytes::<Structure>(bytes1, false).is_err());
    assert!(from_bytes::<Structure>(&bytes2, false).is_err());
}

#[test]
fn float() {
    #[rustfmt::skip]
    let bytes = vec![
        128 + 6,
        0x40, 0xA0, 0x00, 0x00,
        0xC0, 0xA0, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x80, 0x00, 0x00, 0x00,
        0x7F, 0x80, 0x00, 0x00,
        0xFF, 0x80, 0x00, 0x00,
    ];
    let floats = vec![
        5.0_f32,
        -5.0_f32,
        0.0_f32,
        -0.0_f32,
        f32::INFINITY,
        f32::NEG_INFINITY,
    ];
    assert_eq!(from_bytes::<Vec<f32>>(&bytes, false).unwrap(), floats);
    assert_eq!(to_bytes(&floats).unwrap(), bytes);
    assert_eq!(to_size(&floats).unwrap(), bytes.len());
}

#[test]
fn double() {
    #[rustfmt::skip]
    let bytes = vec![
        128 + 6,
        0x40, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xC0, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x7F, 0xF0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xFF, 0xF0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let doubles = vec![
        5.0_f64,
        -5.0_f64,
        0.0_f64,
        -0.0_f64,
        f64::INFINITY,
        f64::NEG_INFINITY,
    ];
    assert_eq!(from_bytes::<Vec<f64>>(&bytes, false).unwrap(), doubles);
    assert_eq!(to_bytes(&doubles).unwrap(), bytes);
    assert_eq!(to_size(&doubles).unwrap(), bytes.len());
}

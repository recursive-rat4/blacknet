/*
 * Copyright (c) 2023-2025 Pavel Vasin
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

use blacknet_network::packet::Hello;
use blacknet_serialization::format::{from_bytes, to_bytes};

#[test]
fn serialization() {
    let empty = Hello::default();
    let empty_bytes: &[u8] = &[128];

    let mut hello = Hello::default();
    hello.set_magic(1).unwrap();
    hello.set_version(2).unwrap();
    hello.set_nonce(3).unwrap();
    hello.set_agent("4").unwrap();
    hello.set_fee_filter(5.into()).unwrap();
    #[rustfmt::skip]
    let hello_bytes: &[u8] = &[
        128 + 5,
        128, 128 + 4, 0, 0, 0, 1,
        129, 128 + 4, 0, 0, 0, 2,
        130, 128 + 8, 0, 0, 0, 0, 0, 0, 0, 3,
        131, 128 + 2, 128 + 1, b'4',
        132, 128 + 8, 0, 0, 0, 0, 0, 0, 0, 5,
    ];

    let deserialized_empty = from_bytes::<Hello>(&empty_bytes, false).unwrap();
    assert_eq!(deserialized_empty.magic(), None);
    assert_eq!(deserialized_empty.version(), None);
    assert_eq!(deserialized_empty.nonce(), None);
    assert_eq!(deserialized_empty.agent(), None);
    assert_eq!(deserialized_empty.fee_filter(), None);

    let deserialized = from_bytes::<Hello>(&hello_bytes, false).unwrap();
    assert_eq!(deserialized.magic(), Some(1));
    assert_eq!(deserialized.version(), Some(2));
    assert_eq!(deserialized.nonce(), Some(3));
    assert_eq!(deserialized.agent(), Some("4".to_owned()));
    assert_eq!(deserialized.fee_filter(), Some(5.into()));

    let serialized_empty = to_bytes::<Hello>(&empty).unwrap();
    assert_eq!(serialized_empty, empty_bytes);

    let serialized = to_bytes::<Hello>(&hello).unwrap();
    // field order isn't fixed
    assert_eq!(serialized.len(), hello_bytes.len());
}

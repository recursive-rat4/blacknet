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

use blacknet_crypto::bigint::UInt256;
use blacknet_kernel::blake2b::Hash;
use blacknet_network::packet::BlockAnnounce;
use blacknet_serialization::format::{from_bytes, to_bytes};

#[test]
fn serialization() {
    #[rustfmt::skip]
    let hash: Hash = [
        0xFB, 0x40, 0x64, 0x28, 0x3A, 0x07, 0xA6, 0x97,
        0x49, 0xA8, 0x01, 0x28, 0x97, 0xBD, 0xA7, 0x15,
        0x9D, 0x6F, 0x25, 0x6F, 0x9A, 0xE4, 0x1B, 0xEB,
        0xF4, 0x7D, 0x4E, 0x96, 0xE7, 0x82, 0x5A, 0x26,
    ];
    let cumulative_difficulty =
        UInt256::from_hex("000000000000000000000000000000000000000000015E6B7FEE4E21DF56BDAE");
    let announce = BlockAnnounce::new(hash, cumulative_difficulty);
    #[rustfmt::skip]
    let bytes: [u8; 65] = [
        0xFB, 0x40, 0x64, 0x28, 0x3A, 0x07, 0xA6, 0x97,
        0x49, 0xA8, 0x01, 0x28, 0x97, 0xBD, 0xA7, 0x15,
        0x9D, 0x6F, 0x25, 0x6F, 0x9A, 0xE4, 0x1B, 0xEB,
        0xF4, 0x7D, 0x4E, 0x96, 0xE7, 0x82, 0x5A, 0x26,
        128 + 32,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00,
        0x01, 0x5E, 0x6B, 0x7F, 0xEE, 0x4E, 0x21, 0xDF,
        0x56, 0xBD, 0xAE,
    ];
    #[rustfmt::skip]
    let bytes_java: [u8; 44] = [
        0xFB, 0x40, 0x64, 0x28, 0x3A, 0x07, 0xA6, 0x97,
        0x49, 0xA8, 0x01, 0x28, 0x97, 0xBD, 0xA7, 0x15,
        0x9D, 0x6F, 0x25, 0x6F, 0x9A, 0xE4, 0x1B, 0xEB,
        0xF4, 0x7D, 0x4E, 0x96, 0xE7, 0x82, 0x5A, 0x26,
        128 + 11,
        0x01, 0x5E, 0x6B, 0x7F, 0xEE, 0x4E, 0x21, 0xDF,
        0x56, 0xBD, 0xAE,
    ];

    let deserialized = from_bytes::<BlockAnnounce>(&bytes, false).unwrap();
    assert_eq!(deserialized.hash(), hash);
    assert_eq!(deserialized.cumulative_difficulty(), cumulative_difficulty);

    let deserialized_java = from_bytes::<BlockAnnounce>(&bytes_java, false).unwrap();
    assert_eq!(deserialized_java.hash(), hash);
    assert_eq!(
        deserialized_java.cumulative_difficulty(),
        cumulative_difficulty
    );

    let serialized = to_bytes(&announce).unwrap();
    assert_eq!(serialized, bytes);
}

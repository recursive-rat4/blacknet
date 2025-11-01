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

use blacknet_kernel::blake2b::Hash;
use blacknet_kernel::block::Block;
use blacknet_serialization::format::{from_bytes, to_bytes};
use data_encoding::HEXUPPER;

#[test]
fn serialization() {
    let deserialized = Block::with_all(
        0,
        Default::default(),
        1545556624.into(),
        HEXUPPER.decode(b"B7E64C1BC5ADD0593397E75E827A8DA323EA8C6E1FE6142A86092C9359117E50").unwrap().try_into().unwrap(),
        Hash::try_from("45B0CFC220CEEC5B7C1C62C4D4193D38E4EBA48E8815729CE75F9C0AB0E4C1C0").unwrap(),
        HEXUPPER.decode(b"0BD14B678ED7C9C5E44E4C2EF6307416B44CFE3315D17345DAF80EF60CD684A5AABDFD0DA0983ED1EC8B3797E49D89053BE49FA2149597FB3E14AAA48DE02505").unwrap().try_into().unwrap(),
        Default::default(),
    );
    let serialized = HEXUPPER.decode(b"000000000000000000000000000000000000000000000000000000000000000000000000000000005C1F5290B7E64C1BC5ADD0593397E75E827A8DA323EA8C6E1FE6142A86092C9359117E5045B0CFC220CEEC5B7C1C62C4D4193D38E4EBA48E8815729CE75F9C0AB0E4C1C00BD14B678ED7C9C5E44E4C2EF6307416B44CFE3315D17345DAF80EF60CD684A5AABDFD0DA0983ED1EC8B3797E49D89053BE49FA2149597FB3E14AAA48DE0250580").unwrap();
    assert_eq!(to_bytes(&deserialized).unwrap(), serialized);
    assert_eq!(
        from_bytes::<Block>(&serialized, false).unwrap(),
        deserialized
    );
}

#[test]
fn hash() {
    let invalid_bytes: [u8; 4] = [0, 1, 2, 3];
    assert_eq!(Block::compute_hash(&invalid_bytes), None);
    assert_eq!(Block::compute_content_hash(&invalid_bytes), None);
}

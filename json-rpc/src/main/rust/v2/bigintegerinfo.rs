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

use num_bigint::BigUint;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct BigIntegerInfo(String);

impl From<&[u8]> for BigIntegerInfo {
    fn from(bytes: &[u8]) -> Self {
        Self(BigUint::from_bytes_be(bytes).to_str_radix(10))
    }
}

#[test]
fn test() {
    let bytes = [
        0x01, 0x5E, 0x6B, 0x7F, 0xEE, 0x4E, 0x21, 0xDF, 0x56, 0xBD, 0xAE,
    ];
    let string = "1654811289011657408691630";
    let info = BigIntegerInfo::from(bytes.as_slice());
    assert_eq!(info.0, string);
}

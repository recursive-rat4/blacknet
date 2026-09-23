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

use blacknet_kernel::blake2b::Hash256;
use core::{array, assert_matches};
use serde_json::{from_str, to_string};

#[test]
fn hash256() {
    let string = "\"000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F\"";
    let bad1 = "\"000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1Z\"";
    let bad2 = "\"00010203\"";
    let bytes: [u8; 32] = array::from_fn(|i| i as u8);
    let hash256 = Hash256::from(bytes);

    assert_matches!(from_str::<Hash256>(string), Ok(x) if x == hash256);
    assert_matches!(from_str::<Hash256>(bad1), Err(_));
    assert_matches!(from_str::<Hash256>(bad2), Err(_));
    assert_matches!(to_string(&hash256), Ok(x) if x == string);
}

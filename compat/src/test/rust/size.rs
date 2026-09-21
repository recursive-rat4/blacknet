/*
 * Copyright (c) 2020-2026 Pavel Vasin
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

use blacknet_compat::size::{Error, Size};
use core::assert_matches;

#[test]
fn parse() {
    for (string, n) in [
        ("400", 400),
        ("4kB", 4000),
        ("4MB", 4000000),
        ("400B", 400),
        ("4KiB", 4096),
        ("4MiB", 4194304),
        ("4GiB", 4294967296),
    ] {
        assert_matches!(Size::parse(string), Ok(x) if x == Size::from(n));
    }
    assert_matches!(Size::parse("litre"), Err(Error::Int(_)));
    assert_matches!(Size::parse("1 pear"), Err(Error::Unit));
    assert_matches!(Size::parse("17179869184 GiB"), Err(Error::Overflow));
}

#[test]
fn to_string_lossy() {
    for (n, string, binary) in [
        (5737956, "5.74 MB", false),
        (2494885, "2.49 MB", false),
        (1025, "1.02 kB", false),
        (33554432, "32 MiB", true),
        (97516, "95.23 KiB", true),
        (1024, "1 KiB", true),
        (1023, "1023 B", true),
    ] {
        assert_eq!(Size::from(n).to_string_lossy(binary), string);
    }
}

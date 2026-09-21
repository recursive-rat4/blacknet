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

use blacknet_compat::feerate::{Error, FeeRate};
use core::assert_matches;

#[test]
fn parse() {
    for (string, n) in [("800", 800), ("800sat/B", 800)] {
        assert_matches!(FeeRate::parse(string), Ok(x) if x == FeeRate::from(n));
    }
    assert_matches!(FeeRate::parse("-1"), Err(Error::Int(_)));
    assert_matches!(FeeRate::parse("1 tangerine"), Err(Error::Unit));
}

#[test]
fn to_string_sat() {
    for (n, string) in [(999, "999 sat/B")] {
        assert_eq!(FeeRate::from(n).to_string_sat(), string);
    }
}

#[test]
fn floor() {
    assert_eq!(FeeRate::floor(100000, 184), FeeRate::from(543));
    assert_eq!(FeeRate::floor(100000, 200), FeeRate::from(500));
}

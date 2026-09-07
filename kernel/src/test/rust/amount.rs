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

use blacknet_kernel::amount::Amount;
use core::{assert_matches, str::FromStr};

#[test]
fn display() {
    let a = Amount::COIN;
    let b = "100000000";

    assert_eq!(a.to_string(), b);
}

#[test]
fn from_str() {
    let a = "100000000";
    let b = "1.00000000";
    let c = "+100000000";

    assert_matches!(Amount::from_str(a), Ok(Amount::COIN));
    assert_matches!(Amount::from_str(b), Err(_));
    assert_matches!(Amount::from_str(c), Err(_));
}

#[test]
fn checked_sum() {
    let a = [0, 1, 2, 3].map(Amount::new).into_iter();
    let b = [1, 0, u64::MAX].map(Amount::new).into_iter();
    let c = Amount::new(6);

    assert_eq!(Amount::checked_sum(a), Some(c));
    assert_eq!(Amount::checked_sum(b), None);
}

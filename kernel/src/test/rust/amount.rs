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

#[test]
fn checked_sum() {
    let a = [0, 1, 2, 3].map(Amount::new).into_iter();
    let b = [1, 0, u64::MAX].map(Amount::new).into_iter();
    let c = Amount::new(6);
    assert_eq!(Amount::checked_sum(a), Some(c));
    assert_eq!(Amount::checked_sum(b), None);
}

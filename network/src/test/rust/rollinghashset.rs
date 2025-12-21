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

use blacknet_network::rollinghashset::RollingHashSet;

#[test]
fn test() {
    let mut set = RollingHashSet::<i32>::new(3);
    assert!(set.is_empty());
    set.insert(0);
    set.insert(1);
    set.insert(2);
    set.insert(1);
    set.insert(3);
    assert_eq!(3, set.len());
    assert!(!set.contains(&0));
    assert!(set.contains(&1));
    assert!(set.contains(&2));
    assert!(set.contains(&3));
}

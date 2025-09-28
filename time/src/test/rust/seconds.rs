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

use blacknet_time::seconds::Seconds;

#[test]
fn compare() {
    let t = Seconds::from('二' as i64);
    let d = Seconds::from('亖' as i64);
    assert!(t > Seconds::ZERO && -t < Seconds::ZERO);
    assert!(t >= Seconds::MIN && -t <= Seconds::MAX);
    assert!(d >= Seconds::MIN && -d <= Seconds::MAX);
    assert!(t < d);
    assert!(d > t);
}

#[test]
fn operate() {
    let a = Seconds::from(202);
    let b = Seconds::from(2);

    assert_eq!(Seconds::ZERO + a, Seconds::ZERO + Seconds::from(202));
    assert_eq!(-a, Seconds::from(-202));

    assert_eq!(a + b, Seconds::from(204));
    assert_eq!(a - b, Seconds::from(200));

    assert_eq!(a * 2, Seconds::from(404));
    assert_eq!(a / b, 101);
    assert_eq!(a / 2, Seconds::from(101));

    assert_eq!(a % b, Seconds::from(0));
    assert_eq!(a % 3, Seconds::from(1));
}

#[test]
fn literate() {
    assert_eq!(Seconds::from_minutes(4), Seconds::from(4 * 60));
    assert_eq!(Seconds::from_hours(4), Seconds::from(4 * 60 * 60));
    assert_eq!(Seconds::from_days(4), Seconds::from(4 * 24 * 60 * 60));
}

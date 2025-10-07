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

use blacknet_time::Milliseconds;

#[test]
fn compare() {
    let t = Milliseconds::from('二' as i64);
    let d = Milliseconds::from('亖' as i64);
    assert!(t > Milliseconds::ZERO && -t < Milliseconds::ZERO);
    assert!(t >= Milliseconds::MIN && -t <= Milliseconds::MAX);
    assert!(d >= Milliseconds::MIN && -d <= Milliseconds::MAX);
    assert!(t < d);
    assert!(d > t);
}

#[test]
fn operate() {
    let a = Milliseconds::from(202);
    let b = Milliseconds::from(2);

    assert_eq!(
        Milliseconds::ZERO + a,
        Milliseconds::ZERO + Milliseconds::from(202)
    );
    assert_eq!(-a, Milliseconds::from(-202));

    assert_eq!(a + b, Milliseconds::from(204));
    assert_eq!(a - b, Milliseconds::from(200));

    assert_eq!(a * 2, Milliseconds::from(404));
    assert_eq!(a / b, 101);
    assert_eq!(a / 2, Milliseconds::from(101));

    assert_eq!(a % b, Milliseconds::from(0));
    assert_eq!(a % 3, Milliseconds::from(1));
}

#[test]
fn literate() {
    assert_eq!(Milliseconds::from_seconds(4), Milliseconds::from(4 * 1000));
    assert_eq!(
        Milliseconds::from_minutes(4),
        Milliseconds::from(4 * 60 * 1000)
    );
    assert_eq!(
        Milliseconds::from_hours(4),
        Milliseconds::from(4 * 60 * 60 * 1000)
    );
    assert_eq!(
        Milliseconds::from_days(4),
        Milliseconds::from(4 * 24 * 60 * 60 * 1000)
    );
}

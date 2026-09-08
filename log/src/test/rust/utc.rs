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

use blacknet_log::UTC;
use core::time::Duration;
use std::time::SystemTime;

#[test]
fn format() {
    // Whether SystemTime can represent test vectors is platform-specific.
    let mut tested = 0;

    let utc = UTC::new();
    for (t, s) in [
        (i64::MIN, "-292275055-05-16 16:47:04.192"),
        (-68729844735001, "-208-01-14 16:27:44.999"),
        (0, "1970-01-01 00:00:00.000"),
        (1545555600000, "2018-12-23 09:00:00.000"),
        (i64::MAX, "292278994-08-17 07:12:55.807"),
    ] {
        let mut b = String::with_capacity(s.len());
        let d = Duration::from_millis(t.unsigned_abs());
        if let Some(st) = if t > 0 {
            SystemTime::UNIX_EPOCH.checked_add(d)
        } else {
            SystemTime::UNIX_EPOCH.checked_sub(d)
        } {
            utc.format(st, &mut b).unwrap();
            assert_eq!(b, s);
            tested += 1;
        }
    }

    assert!(tested > 0);
}

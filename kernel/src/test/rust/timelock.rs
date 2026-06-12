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

use blacknet_kernel::timelock::*;
use core::assert_matches;

#[test]
fn absolute_time() {
    let time_lock = TimeLock::new(TIME, 1000000);
    assert_matches!(
        time_lock.verify(0, 0.into(), 0, (1000000 - 1).into()),
        Err(_)
    );
    assert_matches!(time_lock.verify(0, 0.into(), 0, 1000000.into()), Err(_));
    assert_matches!(time_lock.verify(0, 0.into(), 0, 1000001.into()), Ok(()));
    assert_matches!(
        time_lock.verify(0, 0.into(), 0, (1000001 + 1).into()),
        Ok(())
    );
}

#[test]
fn absolute_height() {
    let time_lock = TimeLock::new(HEIGHT, 1000000);
    assert_matches!(time_lock.verify(0, 0.into(), 1000000 - 1, 0.into()), Err(_));
    assert_matches!(time_lock.verify(0, 0.into(), 1000000, 0.into()), Err(_));
    assert_matches!(time_lock.verify(0, 0.into(), 1000001, 0.into()), Ok(()));
    assert_matches!(time_lock.verify(0, 0.into(), 1000001 + 1, 0.into()), Ok(()));
}

#[test]
fn relative_time() {
    let time_lock = TimeLock::new(RELATIVE_TIME, 10000);
    assert_matches!(
        time_lock.verify(0, 990000.into(), 0, (1000000 - 1).into()),
        Err(_)
    );
    assert_matches!(
        time_lock.verify(0, 990000.into(), 0, 1000000.into()),
        Err(_)
    );
    assert_matches!(
        time_lock.verify(0, 990000.into(), 0, 1000001.into()),
        Ok(())
    );
    assert_matches!(
        time_lock.verify(0, 990000.into(), 0, (1000001 + 1).into()),
        Ok(())
    );
}

#[test]
fn relative_height() {
    let time_lock = TimeLock::new(RELATIVE_HEIGHT, 10000);
    assert_matches!(
        time_lock.verify(990000, 0.into(), 1000000 - 1, 0.into()),
        Err(_)
    );
    assert_matches!(
        time_lock.verify(990000, 0.into(), 1000000, 0.into()),
        Err(_)
    );
    assert_matches!(
        time_lock.verify(990000, 0.into(), 1000001, 0.into()),
        Ok(())
    );
    assert_matches!(
        time_lock.verify(990000, 0.into(), 1000001 + 1, 0.into()),
        Ok(())
    );
}

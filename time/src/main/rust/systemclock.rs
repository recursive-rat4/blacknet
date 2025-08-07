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

use crate::milliseconds::Milliseconds;
use std::time::SystemTime;

// In milliseconds since UNIX epoch not counting leap seconds
pub struct SystemClock {}

impl SystemClock {
    pub fn now() -> Milliseconds {
        let now = SystemTime::now();
        match now.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(since) => (since.as_millis() as i64).into(),
            Err(until) => (-(until.duration().as_millis() as i64)).into(),
        }
    }
}

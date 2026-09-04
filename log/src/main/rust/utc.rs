/*
 * Copyright (c) 2025-2026 Pavel Vasin
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

use core::fmt::{Error as FmtError, Write};
use spdlog::{
    error::Error as LogError,
    formatter::{Pattern, PatternContext},
    {Record, StringBuf},
};
use std::time::SystemTime;

#[derive(Clone, Copy, Default)]
pub struct UTC;

impl UTC {
    pub const fn new() -> Self {
        Self
    }

    pub fn format<W: Write>(&self, st: SystemTime, write: &mut W) -> Result<(), FmtError> {
        let dt = DateTime::new(st);
        write!(
            write,
            "{}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}",
            dt.year, dt.month, dt.day, dt.hour, dt.minute, dt.second, dt.millisecond,
        )
    }
}

impl Pattern for UTC {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> Result<(), LogError> {
        self.format(record.time(), dest)
            .map_err(LogError::FormatRecord)
    }
}

struct DateTime {
    year: i32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    millisecond: u16,
}

impl DateTime {
    fn new(st: SystemTime) -> Self {
        let (seconds, nanosecond) = match st.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(since) => (since.as_secs().cast_signed(), since.subsec_nanos()),
            Err(until) => {
                let until = until.duration();
                let nanosecond = until.subsec_nanos();
                if nanosecond != 0 {
                    (-until.as_secs().cast_signed() - 1, 1000000000 - nanosecond)
                } else {
                    (-until.as_secs().cast_signed(), nanosecond)
                }
            }
        };
        let millisecond = (nanosecond / 1000000) as u16;
        let (minutes, second) = (seconds.div_euclid(60), seconds.rem_euclid(60) as u8);
        let (hours, minute) = (minutes.div_euclid(60), minutes.rem_euclid(60) as u8);
        let (days, hour) = (hours.div_euclid(24), hours.rem_euclid(24) as u8);
        let (year, month, day) = ymd_from_days(days);
        Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
            millisecond,
        }
    }
}

const fn ymd_from_days(mut days: i64) -> (i32, u8, u8) {
    days += 719468;
    let (era, doe) = (days.div_euclid(146097), days.rem_euclid(146097));
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let mut y = era * 400 + yoe;
    let doy = doe - (yoe * 365 + yoe / 4 - yoe / 100);
    let mut m = (doy * 5 + 2) / 153;
    let d = doy - (m * 153 + 2) / 5 + 1;
    m += if m < 10 { 3 } else { -9 };
    y += (m < 3) as i64;
    (y as i32, m as u8, d as u8)
}

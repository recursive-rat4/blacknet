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

use chrono::{DateTime, Datelike, Timelike, Utc};
use core::fmt::Write;
use spdlog::error::{Error, Result};
use spdlog::formatter::{Pattern, PatternContext};
use spdlog::{Record, StringBuf};

#[derive(Clone, Default)]
pub struct UTC;

impl Pattern for UTC {
    fn format(
        &self,
        record: &Record,
        dest: &mut StringBuf,
        _ctx: &mut PatternContext,
    ) -> Result<()> {
        let dt: DateTime<Utc> = record.time().into();
        let millisecond = dt.nanosecond() / 1000000;
        write!(
            dest,
            "{}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}",
            dt.year(),
            dt.month(),
            dt.day(),
            dt.hour(),
            dt.minute(),
            dt.second(),
            millisecond,
        )
        .map_err(Error::FormatRecord)
    }
}

/*
 * Copyright (c) 2018-2026 Pavel Vasin
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

use crate::error::{Error, Result};
use blacknet_time::Seconds;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Copy, Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum TimeKind {
    Time = 0,
    Height = 1,
    RelativeTime = 2,
    RelativeHeight = 3,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct TimeLock {
    algorithm: TimeKind,
    data: i64,
}

impl TimeLock {
    pub const fn new(algorithm: TimeKind, data: i64) -> Self {
        Self { algorithm, data }
    }

    pub fn verify(
        &self,
        compiler_height: u32,
        compiler_time: Seconds,
        height: u32,
        time: Seconds,
    ) -> Result<()> {
        let result = match self.algorithm {
            TimeKind::Time => self.data < time.into(),
            TimeKind::Height => self.data < height as i64,
            TimeKind::RelativeTime => compiler_time + self.data.into() < time,
            TimeKind::RelativeHeight => compiler_height as i64 + self.data < height as i64,
        };
        if result {
            Ok(())
        } else {
            Err(Error::invalid("Invalid time lock"))
        }
    }

    pub const fn algorithm(&self) -> TimeKind {
        self.algorithm
    }

    pub const fn data(&self) -> i64 {
        self.data
    }
}

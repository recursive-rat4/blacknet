/*
 * Copyright (c) 2018-2025 Pavel Vasin
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

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const TIME: u8 = 0;
pub const HEIGHT: u8 = 1;
pub const RELATIVE_TIME: u8 = 2;
pub const RELATIVE_HEIGHT: u8 = 3;

#[derive(Deserialize, Serialize)]
pub struct TimeLock {
    algorithm: u8,
    data: i64,
}

impl TimeLock {
    pub fn new(algorithm: u8, data: i64) -> Self {
        Self { algorithm, data }
    }

    pub fn validate(&self) -> Result<()> {
        match self.algorithm {
            TIME => Ok(()),
            HEIGHT => Ok(()),
            RELATIVE_TIME => Ok(()),
            RELATIVE_HEIGHT => Ok(()),
            _ => Err(Error::UnknownType(self.algorithm)),
        }
    }

    pub fn verify(
        &self,
        compiler_height: u32,
        compiler_time: i64,
        height: u32,
        time: i64,
    ) -> Result<()> {
        let result = match self.algorithm {
            TIME => self.data < time,
            HEIGHT => self.data < height as i64,
            RELATIVE_TIME => compiler_time + self.data < time,
            RELATIVE_HEIGHT => compiler_height as i64 + self.data < height as i64,
            _ => return Err(Error::UnknownType(self.algorithm)),
        };
        if result {
            Ok(())
        } else {
            Err(Error::InvalidTime)
        }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unknown time lock type {0}")]
    UnknownType(u8),
    #[error("Invalid time")]
    InvalidTime,
}

pub type Result<T> = core::result::Result<T, Error>;

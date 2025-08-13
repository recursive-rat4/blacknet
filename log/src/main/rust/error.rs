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

use core::fmt;

#[derive(Debug)]
pub enum Error {
    NotUnicodeLogLevel,
    Spdlog(spdlog::error::Error),
}

impl core::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::NotUnicodeLogLevel => formatter.write_str("Not unicode data in environment variable BLACKNET_LOGLEVEL"),
            Error::Spdlog(err) => write!(formatter, "{err}"),
        }
    }
}

impl From<spdlog::error::Error> for Error {
    fn from(err: spdlog::error::Error) -> Self {
        Error::Spdlog(err)
    }
}

pub type Result<T> = core::result::Result<T, Error>;

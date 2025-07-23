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
use serde::{de, ser};

#[derive(Debug)]
pub enum Error {
    Message(String),
    Io(std::io::Error),
    Utf8(std::string::FromUtf8Error),
    TooLongVarInt,
    InvalidBool(u8),
    InvalidOption(u8),
    TrailingBytes(usize),
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Message(msg) => formatter.write_str(msg),
            Error::Io(err) => write!(formatter, "{err}"),
            Error::Utf8(err) => write!(formatter, "{err}"),
            Error::TooLongVarInt => formatter.write_str("Too long VarInt"),
            Error::InvalidBool(byte) => write!(formatter, "0x{byte:X} is not boolean"),
            Error::InvalidOption(byte) => write!(formatter, "0x{byte:X} is not option"),
            Error::TrailingBytes(remaining) => write!(formatter, "{remaining} trailing bytes"),
        }
    }
}

impl core::error::Error for Error {}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Error::Utf8(err)
    }
}

pub type Result<T> = core::result::Result<T, Error>;

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

use alloc::borrow::Cow;
use alloc::string::{FromUtf8Error, ToString};
use blacknet_io::Error as IoError;
use core::fmt;
use serde_core::{de, ser};

#[derive(Debug)]
pub enum Error {
    Message(Cow<'static, str>),
    Io(IoError),
    Utf8(FromUtf8Error),
    TooLongVarInt,
    InvalidBool(u8),
    InvalidOption(u8),
    TrailingBytes(usize),
}

impl Error {
    pub fn message<T>(msg: T) -> Self
    where
        Cow<'static, str>: From<T>,
    {
        Error::Message(msg.into())
    }
}

impl From<IoError> for Error {
    fn from(error: IoError) -> Self {
        Self::Io(error)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Self {
        Self::Utf8(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Message(msg) => f.write_str(msg),
            Self::Io(err) => write!(f, "{err}"),
            Self::Utf8(err) => write!(f, "{err}"),
            Self::TooLongVarInt => f.write_str("Too long VarInt"),
            Self::InvalidBool(byte) => write!(f, "0x{byte:X} is not boolean"),
            Self::InvalidOption(byte) => write!(f, "0x{byte:X} is not option"),
            Self::TrailingBytes(size) => write!(f, "{size} trailing bytes"),
        }
    }
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Message(msg.to_string().into())
    }
}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Message(msg.to_string().into())
    }
}

impl core::error::Error for Error {}

pub type Result<T, E = Error> = core::result::Result<T, E>;

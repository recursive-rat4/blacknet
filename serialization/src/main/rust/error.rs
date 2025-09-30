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

use alloc::string::{FromUtf8Error, String, ToString};
use core::fmt::Display;
use serde::{de, ser};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Message(String),
    #[error("{0}")]
    StaticMessage(&'static str),
    #[error("{0}")]
    Io(#[from] blacknet_io::Error),
    #[error("{0}")]
    Utf8(#[from] FromUtf8Error),
    #[error("Too long VarInt")]
    TooLongVarInt,
    #[error("0x{0:X} is not boolean")]
    InvalidBool(u8),
    #[error("0x{0:X} is not option")]
    InvalidOption(u8),
    #[error("{0} trailing bytes")]
    TrailingBytes(usize),
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

pub type Result<T> = core::result::Result<T, Error>;

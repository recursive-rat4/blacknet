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

use alloc::borrow::Cow;
use alloc::string::ToString;
use blacknet_serialization::error::Error as SerializationError;
use core::fmt;

#[derive(Debug)]
pub enum Error {
    AlreadyHave(Cow<'static, str>),
    InFuture(Cow<'static, str>),
    Invalid(Cow<'static, str>),
    NotReachableVertex(Cow<'static, str>),
}

impl Error {
    pub fn already_have<T>(msg: T) -> Self
    where
        Cow<'static, str>: From<T>,
    {
        Error::AlreadyHave(msg.into())
    }

    pub fn in_future<T>(msg: T) -> Self
    where
        Cow<'static, str>: From<T>,
    {
        Error::InFuture(msg.into())
    }

    pub fn invalid<T>(msg: T) -> Self
    where
        Cow<'static, str>: From<T>,
    {
        Error::Invalid(msg.into())
    }

    pub fn not_reachable_vertex<T>(msg: T) -> Self
    where
        Cow<'static, str>: From<T>,
    {
        Error::NotReachableVertex(msg.into())
    }
}

impl From<SerializationError> for Error {
    fn from(error: SerializationError) -> Self {
        Error::invalid(error.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::AlreadyHave(msg) => write!(f, "Already have {msg}"),
            Error::InFuture(msg) => write!(f, "Too far in future {msg}"),
            Error::Invalid(msg) => f.write_str(msg),
            Error::NotReachableVertex(msg) => write!(f, "Not reachable vertex {msg}"),
        }
    }
}

impl core::error::Error for Error {}

pub type Result<T, E = Error> = core::result::Result<T, E>;

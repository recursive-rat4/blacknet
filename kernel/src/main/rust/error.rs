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

use alloc::string::{String, ToString};
use blacknet_serialization::error::Error as SerializationError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Already have {0}")]
    AlreadyHave(String),
    #[error("Too far in future {0}")]
    InFuture(String),
    #[error("{0}")]
    Invalid(String),
    #[error("Not reachable vertex {0}")]
    NotReachableVertex(String),
}

impl From<SerializationError> for Error {
    fn from(error: SerializationError) -> Self {
        Error::Invalid(error.to_string())
    }
}

pub type Result<T> = core::result::Result<T, Error>;

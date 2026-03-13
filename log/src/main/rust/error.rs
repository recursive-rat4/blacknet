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

use core::fmt;
use spdlog::error::Error as SpdlogError;
use spdlog::error::InvalidArgumentError;
use std::io::Error as IoError;

#[derive(Debug)]
pub enum Error {
    NotUnicodeLogLevel,

    FormatRecord(fmt::Error),

    WriteRecord(IoError),
    FlushBuffer(IoError),
    CreateDirectory(IoError),
    OpenFile(IoError),
    QueryFileMetadata(IoError),
    RenameFile(IoError),
    RemoveFile(IoError),
    ParseLevel(String),

    InvalidArgument(InvalidArgumentError),

    Multiple(Vec<Error>),
    Upstream(String),
}

impl From<SpdlogError> for Error {
    fn from(err: SpdlogError) -> Self {
        match err {
            SpdlogError::FormatRecord(err) => Self::FormatRecord(err),
            SpdlogError::WriteRecord(err) => Self::WriteRecord(err),
            SpdlogError::FlushBuffer(err) => Self::FlushBuffer(err),
            SpdlogError::CreateDirectory(err) => Self::CreateDirectory(err),
            SpdlogError::OpenFile(err) => Self::OpenFile(err),
            SpdlogError::QueryFileMetadata(err) => Self::QueryFileMetadata(err),
            SpdlogError::RenameFile(err) => Self::RenameFile(err),
            SpdlogError::RemoveFile(err) => Self::RemoveFile(err),
            SpdlogError::ParseLevel(err) => Self::ParseLevel(err),
            SpdlogError::InvalidArgument(err) => Self::InvalidArgument(err),
            SpdlogError::Multiple(err) => {
                Self::Multiple(err.into_iter().map(Error::from).collect())
            }
            SpdlogError::Downstream(err) => Self::Upstream(err.to_string()),
            _ => Self::Upstream(err.to_string()),
        }
    }
}

#[cfg(feature = "log")]
impl From<log::SetLoggerError> for Error {
    fn from(err: log::SetLoggerError) -> Self {
        Self::Upstream(err.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotUnicodeLogLevel => {
                f.write_str("Not unicode data in environment variable BLACKNET_LOGLEVEL")
            }
            Self::FormatRecord(err) => write!(f, "{err}"),

            Self::WriteRecord(err) => write!(f, "{err}"),
            Self::FlushBuffer(err) => write!(f, "{err}"),
            Self::CreateDirectory(err) => write!(f, "{err}"),
            Self::OpenFile(err) => write!(f, "{err}"),
            Self::QueryFileMetadata(err) => write!(f, "{err}"),
            Self::RenameFile(err) => write!(f, "{err}"),
            Self::RemoveFile(err) => write!(f, "{err}"),
            Self::ParseLevel(msg) => f.write_str(msg),

            Self::InvalidArgument(err) => write!(f, "{err}"),

            Self::Multiple(errs) => write!(f, "{errs:?}"),
            Self::Upstream(msg) => f.write_str(msg),
        }
    }
}

impl core::error::Error for Error {}

pub type Result<T> = core::result::Result<T, Error>;

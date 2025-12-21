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

use spdlog::error::Error as Up;
use spdlog::error::InvalidArgumentError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Not unicode data in environment variable BLACKNET_LOGLEVEL")]
    NotUnicodeLogLevel,

    #[error("{0}")]
    FormatRecord(core::fmt::Error),

    #[error("{0}")]
    WriteRecord(std::io::Error),
    #[error("{0}")]
    FlushBuffer(std::io::Error),
    #[error("{0}")]
    CreateDirectory(std::io::Error),
    #[error("{0}")]
    OpenFile(std::io::Error),
    #[error("{0}")]
    QueryFileMetadata(std::io::Error),
    #[error("{0}")]
    RenameFile(std::io::Error),
    #[error("{0}")]
    RemoveFile(std::io::Error),
    #[error("{0}")]
    ParseLevel(String),

    #[error("{0}")]
    InvalidArgument(InvalidArgumentError),

    #[error("{0:?}")]
    Multiple(Vec<Error>),
    #[error("{0}")]
    Upstream(String),
}

impl From<Up> for Error {
    fn from(err: Up) -> Self {
        match err {
            Up::FormatRecord(err) => Self::FormatRecord(err),
            Up::WriteRecord(err) => Self::WriteRecord(err),
            Up::FlushBuffer(err) => Self::FlushBuffer(err),
            Up::CreateDirectory(err) => Self::CreateDirectory(err),
            Up::OpenFile(err) => Self::OpenFile(err),
            Up::QueryFileMetadata(err) => Self::QueryFileMetadata(err),
            Up::RenameFile(err) => Self::RenameFile(err),
            Up::RemoveFile(err) => Self::RemoveFile(err),
            Up::ParseLevel(err) => Self::ParseLevel(err),
            Up::InvalidArgument(err) => Self::InvalidArgument(err),
            Up::Multiple(err) => Self::Multiple(err.into_iter().map(Error::from).collect()),
            Up::Downstream(err) => Self::Upstream(err.to_string()),
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

pub type Result<T> = core::result::Result<T, Error>;

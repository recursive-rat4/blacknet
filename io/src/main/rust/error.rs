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

use thiserror::Error;

#[derive(Debug, Error)]
#[error("{message}")]
pub struct Error {
    message: &'static str,
}

impl Error {
    pub(crate) fn unexpected_eof() -> Self {
        Self {
            message: "Unexpected end of file",
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;

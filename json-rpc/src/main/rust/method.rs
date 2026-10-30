/*
 * Copyright (c) 2023-2026 Pavel Vasin
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
use core::fmt;
use serde::{Deserialize, Serialize};

const RESERVED: &str = "rpc.";

/// Type for the `method` field in a [Request][crate::Request].
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Method(Cow<'static, str>);

impl Method {
    /// Construct a new method name.
    pub fn new<T: Into<Cow<'static, str>>>(name: T) -> Self {
        Self(name.into())
    }

    /// Whether reserved for rpc-internal methods and extensions.
    pub fn is_reserved(&self) -> bool {
        self.0.starts_with(RESERVED)
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

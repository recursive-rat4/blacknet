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

use alloc::{borrow::Cow, string::String};
use serde::{Deserialize, Deserializer, Serialize, de::Error};

const RESERVED: &str = "rpc.";

/// Type for the `method` field in a [Request][crate::Request].
#[derive(Debug, Eq, Hash, PartialEq, Serialize)]
pub struct Method(Cow<'static, str>);

impl Method {
    /// Construct a new method name.
    ///
    /// Returns [None] if method name is reserved.
    pub fn new<T: Into<Cow<'static, str>>>(name: T) -> Option<Self> {
        let name = name.into();
        if !name.starts_with(RESERVED) {
            Some(Self(name))
        } else {
            None
        }
    }
}

impl<'de> Deserialize<'de> for Method {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let name = String::deserialize(deserializer)?;
        if !name.starts_with(RESERVED) {
            Ok(Self(Cow::Owned(name)))
        } else {
            Err(D::Error::custom("Reserved method name"))
        }
    }
}

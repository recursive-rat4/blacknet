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

use alloc::string::String;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};
use serde_json::{Number, Value};

/// Type for the `id` field in a [Request][crate::Request] or [Response][crate::Response].
#[derive(Debug, Default, Eq, PartialEq)]
pub enum Id {
    #[default]
    Null,
    Number(Number),
    String(String),
}

impl Id {
    /// Create a new null id.
    pub const fn new() -> Self {
        Self::Null
    }

    /// Create a new number id.
    pub fn number<T: Into<Number>>(n: T) -> Self {
        Self::Number(n.into())
    }

    /// Create a new string id.
    pub fn string<T: Into<String>>(s: T) -> Self {
        Self::String(s.into())
    }
}

impl Serialize for Id {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Null => Value::Null.serialize(serializer),
            Self::Number(number) => number.serialize(serializer),
            Self::String(string) => string.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for Id {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match Value::deserialize(deserializer)? {
            Value::Null => Ok(Self::Null),
            Value::Number(number) => Ok(Self::Number(number)),
            Value::String(string) => Ok(Self::String(string)),
            _ => Err(D::Error::custom("Id is not string, number or null")),
        }
    }
}

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

use alloc::{string::String, vec::Vec};
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{DeserializeOwned, Error as _},
};
use serde_json::{Error, Map, Value, from_value, to_value};

/// Type for the `params` field in a [Request][crate::Request].
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Params {
    Positional(Vec<Value>),
    Named(Map<String, Value>),
}

impl Params {
    /// Construct new positional parameters.
    pub fn positional<T: Into<Value>, I: IntoIterator<Item = T>>(params: I) -> Self {
        Self::Positional(params.into_iter().map(Into::into).collect())
    }

    /// Construct new named parameters.
    pub fn named<K: Into<String>, V: Into<Value>, I: IntoIterator<Item = (K, V)>>(
        params: I,
    ) -> Self {
        Self::Named(
            params
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        )
    }
}

impl Serialize for Params {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Positional(params) => params.serialize(serializer),
            Self::Named(params) => params.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for Params {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match Value::deserialize(deserializer)? {
            Value::Array(params) => Ok(Self::Positional(params)),
            Value::Object(params) => Ok(Self::Named(params)),
            _ => Err(D::Error::custom("Params are neither array nor object")),
        }
    }
}

/// Convert params to a struct.
pub fn from_params<T: DeserializeOwned>(params: Option<Params>) -> Result<T, Error> {
    from_value::<T>(match params {
        Some(Params::Positional(params)) => Value::Array(params),
        Some(Params::Named(params)) => Value::Object(params),
        None => Value::Null,
    })
}

/// Convert a struct to params.
pub fn to_params<T: Serialize>(t: T) -> Result<Option<Params>, Error> {
    match to_value(t)? {
        Value::Null => Ok(None),
        Value::Array(params) => Ok(Some(Params::Positional(params))),
        Value::Object(params) => Ok(Some(Params::Named(params))),
        _ => Err(Error::custom(
            "Present parameters are neither array nor object",
        )),
    }
}

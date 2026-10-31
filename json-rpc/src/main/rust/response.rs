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

use crate::{Error, Id, JsonRpc};
use serde::{
    Deserialize, Deserializer, Serialize, Serializer, de::Error as _, ser::SerializeStruct,
};
use serde_json::{Value, from_value};

/// A JSON-RPC response to a [Request][crate::Request] that matches by the [Id][crate::Id].
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    Result(Value, Id),
    Error(Error, Id),
}

impl Serialize for Response {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_struct("Response", 3)?;
        s.serialize_field("jsonrpc", &JsonRpc)?;
        let id = match self {
            Self::Result(value, id) => {
                s.serialize_field("result", value)?;
                id
            }
            Self::Error(error, id) => {
                s.serialize_field("error", error)?;
                id
            }
        };
        s.serialize_field("id", id)?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for Response {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(deserializer)?;
        let Value::Object(mut object) = value else {
            return Err(D::Error::custom("Response is not an object"));
        };

        let Some(jsonrpc) = object.remove("jsonrpc") else {
            return Err(D::Error::missing_field("jsonrpc"));
        };
        let result = object.remove("result");
        let error = object.remove("error");
        let Some(id) = object.remove("id") else {
            return Err(D::Error::missing_field("id"));
        };
        if let Some(field) = object.keys().next() {
            return Err(D::Error::unknown_field(
                field,
                &["jsonrpc", "result", "error", "id"],
            ));
        }

        let _jsonrpc = from_value::<JsonRpc>(jsonrpc).map_err(D::Error::custom)?;
        let id = from_value::<Id>(id).map_err(D::Error::custom)?;

        match (result, error) {
            (Some(result), None) => Ok(Self::Result(result, id)),
            (None, Some(error)) => Ok(Self::Error(
                from_value::<Error>(error).map_err(D::Error::custom)?,
                id,
            )),
            (Some(_), Some(_)) => Err(D::Error::custom("Response is both result and error")),
            (None, None) => Err(D::Error::custom("Response is neither result nor error")),
        }
    }
}

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
use serde_json::Value;

/// A JSON-RPC response to a [Request][crate::Request] that matches by the [Id][crate::Id].
#[derive(Debug, Eq, PartialEq)]
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
        let deser = Deser::deserialize(deserializer)?;
        match (deser.result, deser.error) {
            (Some(result), None) => Ok(Self::Result(result, deser.id)),
            (None, Some(error)) => Ok(Self::Error(error, deser.id)),
            (Some(_), Some(_)) => Err(D::Error::custom("Response is both result and error")),
            (None, None) => Err(D::Error::custom("Response is neither result nor error")),
        }
    }
}

#[derive(Deserialize)]
struct Deser {
    #[allow(dead_code)]
    jsonrpc: JsonRpc,
    result: Option<Value>,
    error: Option<Error>,
    id: Id,
}

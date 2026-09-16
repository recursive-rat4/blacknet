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
use serde::{Deserialize, Deserializer, Serialize, de::Error as _};
use serde_json::Value;

/// A JSON-RPC response to a [Request][crate::Request] that matches by the [Id][crate::Id].
#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct Response {
    jsonrpc: JsonRpc,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<Error>,
    id: Id,
}

impl Response {
    /// Construct a new response.
    pub fn success<T: Into<Value>>(result: T, id: Id) -> Self {
        Self {
            jsonrpc: JsonRpc,
            result: Some(result.into()),
            error: None,
            id,
        }
    }

    /// Construct a new response.
    pub const fn error(error: Error, id: Id) -> Self {
        Self {
            jsonrpc: JsonRpc,
            result: None,
            error: Some(error),
            id,
        }
    }
}

impl<'de> Deserialize<'de> for Response {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let deser = Deser::deserialize(deserializer)?;
        if deser.result.is_none() ^ deser.error.is_none() {
            Ok(Self {
                jsonrpc: deser.jsonrpc,
                result: deser.result,
                error: deser.error,
                id: deser.id,
            })
        } else {
            Err(D::Error::custom("Response is neither success nor error"))
        }
    }
}

#[derive(Deserialize)]
struct Deser {
    jsonrpc: JsonRpc,
    result: Option<Value>,
    error: Option<Error>,
    id: Id,
}

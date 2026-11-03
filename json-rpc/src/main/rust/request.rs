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

use crate::{Id, JsonRpc, Method, Params};
use serde::{Deserialize, Deserializer, Serialize, de::Error as _};
use serde_json::{Value, from_value};

/// A JSON-RPC request that must be replied
/// with a [Response][crate::Response] if an [Id][crate::Id] is present,
/// otherwise it's a notification.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Request {
    jsonrpc: JsonRpc,
    method: Method,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<Params>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<Id>,
}

impl Request {
    /// Create a new request.
    pub const fn new(method: Method, params: Option<Params>, id: Id) -> Self {
        Self {
            jsonrpc: JsonRpc,
            method,
            params,
            id: Some(id),
        }
    }

    /// Create a new notification.
    pub const fn notification(method: Method, params: Option<Params>) -> Self {
        Self {
            jsonrpc: JsonRpc,
            method,
            params,
            id: None,
        }
    }

    /// Requested method.
    pub const fn method(&self) -> &Method {
        &self.method
    }

    /// Id of request, or none if notification.
    pub const fn id(&self) -> Option<&Id> {
        self.id.as_ref()
    }

    /// Whether this request is a notification.
    pub const fn is_notification(&self) -> bool {
        self.id.is_none()
    }

    pub fn into_params_id(self) -> (Option<Params>, Option<Id>) {
        (self.params, self.id)
    }

    pub fn into_id(self) -> Option<Id> {
        self.id
    }
}

impl<'de> Deserialize<'de> for Request {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(deserializer)?;
        let Value::Object(mut object) = value else {
            return Err(D::Error::custom("Request is not an object"));
        };

        let Some(jsonrpc) = object.remove("jsonrpc") else {
            return Err(D::Error::missing_field("jsonrpc"));
        };
        let Some(method) = object.remove("method") else {
            return Err(D::Error::missing_field("method"));
        };
        let params = object.remove("params");
        let id = object.remove("id");
        if let Some(field) = object.keys().next() {
            return Err(D::Error::unknown_field(
                field,
                &["jsonrpc", "method", "params", "id"],
            ));
        }

        let jsonrpc = from_value::<JsonRpc>(jsonrpc).map_err(D::Error::custom)?;
        let method = from_value::<Method>(method).map_err(D::Error::custom)?;
        let params = params
            .map(|params| from_value::<Params>(params).map_err(D::Error::custom))
            .transpose()?;
        let id = id
            .map(|id| from_value::<Id>(id).map_err(D::Error::custom))
            .transpose()?;

        Ok(Self {
            jsonrpc,
            method,
            params,
            id,
        })
    }
}

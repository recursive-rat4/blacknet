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
use serde::{Deserialize, Serialize};

/// A JSON-RPC request that must be replied
/// with a [Response][crate::Response] if an [Id][crate::Id] is present,
/// otherwise it's a notification.
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
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

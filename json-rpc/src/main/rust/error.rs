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
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Type for the `error` field in a [Response][crate::Response].
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Error {
    code: i16,
    message: Cow<'static, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

impl Error {
    /// Construct a custom error.
    pub fn custom<M: Into<Cow<'static, str>>, D: Into<Value>>(
        code: i16,
        message: M,
        data: Option<D>,
    ) -> Self {
        Self {
            code,
            message: message.into(),
            data: data.map(Into::into),
        }
    }

    /// Invalid JSON was received by the server.
    ///
    /// An error occurred on the server while parsing the JSON text.
    pub fn parse_error<T: Into<Value>>(data: Option<T>) -> Self {
        Self::custom(-32700, "Parse error", data)
    }

    /// The JSON sent is not a valid Request object.
    pub fn invalid_request<T: Into<Value>>(data: Option<T>) -> Self {
        Self::custom(-32600, "Invalid Request", data)
    }

    /// The method does not exist / is not available.
    pub fn method_not_found<T: Into<Value>>(data: Option<T>) -> Self {
        Self::custom(-32601, "Method not found", data)
    }

    /// Invalid method parameter(s).
    pub fn invalid_params<T: Into<Value>>(data: Option<T>) -> Self {
        Self::custom(-32602, "Invalid params", data)
    }

    /// Internal JSON-RPC error.
    pub fn internal_error<T: Into<Value>>(data: Option<T>) -> Self {
        Self::custom(-32603, "Internal error", data)
    }

    // Implementation-defined server-errors from -32099 to -32000

    /// An application-defined error.
    pub fn application_error<T: Into<Value>>(data: Option<T>) -> Self {
        Self::custom(-1, "Application error", data.map(Into::into))
    }
}

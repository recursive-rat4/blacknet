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

use core::{fmt, str};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};

const JSON_RPC: &str = "2.0";

/// Type for the `jsonrpc` field in a [Request][crate::Request] or [Response][crate::Response].
#[derive(Default, Eq, PartialEq)]
pub struct JsonRpc;

impl JsonRpc {
    /// Construct the new JSON-RPC version.
    pub const fn new() -> Self {
        Self
    }
}

impl fmt::Debug for JsonRpc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("JsonRpc").field(&JSON_RPC).finish()
    }
}

impl Serialize for JsonRpc {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(JSON_RPC)
    }
}

impl<'de> Deserialize<'de> for JsonRpc {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        if <&str>::deserialize(deserializer)? == JSON_RPC {
            Ok(Self)
        } else {
            Err(D::Error::custom("Unsupported JSON-RPC version"))
        }
    }
}

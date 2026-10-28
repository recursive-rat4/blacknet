/*
 * Copyright (c) 2026 Pavel Vasin
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

use crate::{Request, Response};
use alloc::vec::Vec;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as _, ser::Error as _};
use serde_json::{Value, from_value};

/// A batch of [Request][crate::Request]s.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BatchRequest {
    Object(Request),
    Array(Vec<Request>),
}

impl Serialize for BatchRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Object(request) => request.serialize(serializer),
            Self::Array(requests) => {
                if !requests.is_empty() {
                    requests.serialize(serializer)
                } else {
                    Err(S::Error::custom("Empty batch request"))
                }
            }
        }
    }
}

impl<'de> Deserialize<'de> for BatchRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(deserializer)?;
        match value {
            Value::Object(_) => Ok(Self::Object(
                from_value::<Request>(value).map_err(D::Error::custom)?,
            )),
            Value::Array(_) => {
                let requests = from_value::<Vec<Request>>(value).map_err(D::Error::custom)?;
                if !requests.is_empty() {
                    Ok(Self::Array(requests))
                } else {
                    Err(D::Error::custom("Empty batch request"))
                }
            }
            _ => Err(D::Error::custom(
                "Batch request is neither object nor array",
            )),
        }
    }
}

/// A batch of [Response][crate::Response]s.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BatchResponse {
    Object(Response),
    Array(Vec<Response>),
}

impl Serialize for BatchResponse {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Object(response) => response.serialize(serializer),
            Self::Array(responses) => {
                if !responses.is_empty() {
                    responses.serialize(serializer)
                } else {
                    Err(S::Error::custom("Empty batch response"))
                }
            }
        }
    }
}

impl<'de> Deserialize<'de> for BatchResponse {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(deserializer)?;
        match value {
            Value::Object(_) => Ok(Self::Object(
                from_value::<Response>(value).map_err(D::Error::custom)?,
            )),
            Value::Array(_) => {
                let responses = from_value::<Vec<Response>>(value).map_err(D::Error::custom)?;
                if !responses.is_empty() {
                    Ok(Self::Array(responses))
                } else {
                    Err(D::Error::custom("Empty batch response"))
                }
            }
            _ => Err(D::Error::custom(
                "Batch response is neither object nor array",
            )),
        }
    }
}

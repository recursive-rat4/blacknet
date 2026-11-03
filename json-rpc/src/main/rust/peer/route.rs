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

use crate::{Id, Method, Request, to_params};
use serde::Serialize;
use serde_json::Error;

/// A route for JSON-RPC params.
pub trait Route {
    /// Result type.
    type Result;
    /// Error type.
    type Error;

    /// Mathod name.
    const METHOD: Method;
}

/// Convert params and id to a request.
pub fn to_request<T: Route + Serialize>(t: T, id: Id) -> Result<Request, Error> {
    Ok(Request::new(T::METHOD, to_params(t)?, id))
}

/// Convert params to a notification.
pub fn to_notification<T: Route + Serialize>(t: T) -> Result<Request, Error> {
    Ok(Request::notification(T::METHOD, to_params(t)?))
}

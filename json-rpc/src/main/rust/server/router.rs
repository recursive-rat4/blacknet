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

use crate::{
    Error, Id, Method, Request, Response,
    server::{ErasedHandler, Handler, RpcHandler},
};
use alloc::{
    boxed::Box,
    string::{String, ToString},
};
use core::fmt;
use hashbrown::{HashMap, hash_map::Entry};
use serde::de::DeserializeOwned;
use serde_json::{Error as JsonError, error::Category, from_str, to_string};

/// Router for [Request][crate::Request]s.
#[derive(Default)]
pub struct Router<S> {
    table: HashMap<Method, Box<dyn ErasedHandler<S> + Send + Sync>>,
}

impl<S: Send + Sync + 'static> Router<S> {
    /// Construct an empty router.
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }

    /// Register a handler for a method.
    pub fn route<T: Handler<S> + DeserializeOwned + Send + Sync + 'static>(
        &mut self,
        method: Method,
    ) -> Result<(), OccupiedError> {
        match self.table.entry(method) {
            Entry::Vacant(vacant) => {
                vacant.insert(RpcHandler::<T, S>::erase());
                Ok(())
            }
            Entry::Occupied(_) => Err(OccupiedError),
        }
    }

    /// Handle a request.
    pub fn handle(&self, request: Request, state: &S) -> Option<Response> {
        match self.table.get(request.method()) {
            Some(handler) => {
                let (params, id) = request.into_params_id();
                let result = handler.handle(params, state);
                let id = id?;
                match result {
                    Ok(result) => Some(Response::success(result, id)),
                    Err(error) => Some(Response::error(error, id)),
                }
            }
            None => request
                .into_id()
                .map(|id| Response::error(Error::method_not_found(Option::<()>::None), id)),
        }
    }

    /// Deserialize and handle a request, then serialize a response.
    pub fn handle_str(&self, request: &str, state: &S) -> Result<Option<String>, JsonError> {
        let request = match from_str::<Request>(request) {
            Ok(request) => request,
            Err(err) => {
                return match err.classify() {
                    Category::Data => response_to_string(Some(Response::error(
                        Error::invalid_request(Some(err.to_string())),
                        Id::Null,
                    ))),
                    Category::Syntax => response_to_string(Some(Response::error(
                        Error::parse_error(Some(err.to_string())),
                        Id::Null,
                    ))),
                    Category::Io | Category::Eof => Err(err),
                };
            }
        };
        let response = self.handle(request, state);
        response_to_string(response)
    }
}

fn response_to_string(response: Option<Response>) -> Result<Option<String>, JsonError> {
    response.map(|response| to_string(&response)).transpose()
}

#[derive(Debug)]
pub struct OccupiedError;

impl fmt::Display for OccupiedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Method already has a handler")
    }
}

impl core::error::Error for OccupiedError {}

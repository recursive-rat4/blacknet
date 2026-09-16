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
    Error, Method, Request, Response,
    server::{ErasedHandler, Handler, RpcHandler},
};
use alloc::boxed::Box;
use core::fmt;
use hashbrown::{HashMap, hash_map::Entry};
use serde::de::DeserializeOwned;

/// Router for [Request][crate::Request]s.
#[derive(Default)]
pub struct Router<S> {
    table: HashMap<Method, Box<dyn ErasedHandler<S>>>,
    state: S,
}

impl<S: 'static> Router<S> {
    /// Construct an empty router.
    pub fn new(state: S) -> Self {
        Self {
            table: HashMap::new(),
            state,
        }
    }

    /// Register a handler for a method.
    pub fn route<T: Handler<S> + DeserializeOwned + 'static>(
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
    pub fn handle(&self, request: Request) -> Option<Response> {
        match self.table.get(request.method()) {
            Some(handler) => {
                let (params, id) = request.into_params_id();
                let result = handler.handle(params, &self.state);
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
}

#[derive(Debug)]
pub struct OccupiedError;

impl fmt::Display for OccupiedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Method already has a handler")
    }
}

impl core::error::Error for OccupiedError {}

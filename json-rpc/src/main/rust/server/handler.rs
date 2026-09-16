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

use crate::{Error, Params};
use alloc::{boxed::Box, string::ToString};
use core::marker::PhantomData;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, from_value, to_value};

/// A handler for a JSON-RPC request.
pub trait Handler<S> {
    /// Success type.
    type Ok: Serialize;
    /// Error type.
    type Error: Serialize;

    /// Handle a JSON-RPC request and return a result to respond.
    /// If the request is a notification then the result will be dropped.
    fn handle(self, state: &S) -> Result<Self::Ok, Self::Error>;
}

pub trait ErasedHandler<S> {
    fn handle(&self, params: Option<Params>, state: &S) -> Result<Value, Error>;
}

pub struct RpcHandler<T, S> {
    phantom_t: PhantomData<T>,
    phantom_s: PhantomData<S>,
}

impl<T: Handler<S> + DeserializeOwned + 'static, S: 'static> RpcHandler<T, S> {
    pub fn erase() -> Box<dyn ErasedHandler<S>> {
        Box::new(Self {
            phantom_t: PhantomData,
            phantom_s: PhantomData,
        })
    }
}

impl<T: Handler<S> + DeserializeOwned, S> ErasedHandler<S> for RpcHandler<T, S> {
    fn handle(&self, params: Option<Params>, state: &S) -> Result<Value, Error> {
        let params = match params {
            Some(Params::Positional(params)) => Value::Array(params),
            Some(Params::Named(params)) => Value::Object(params),
            None => Value::Null,
        };
        let result = match from_value::<T>(params) {
            Ok(handler) => handler.handle(state),
            Err(err) => return Err(Error::invalid_params(Some(err.to_string()))),
        };
        match result {
            Ok(ok) => to_value(ok).map_err(|err| Error::internal_error(Some(err.to_string()))),
            Err(err) => Err(match to_value(err) {
                Ok(err) => Error::application_error(Some(err)),
                Err(err) => Error::internal_error(Some(err.to_string())),
            }),
        }
    }
}

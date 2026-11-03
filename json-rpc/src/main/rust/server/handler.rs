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

use crate::{Error, Params, from_params, peer::Route};
use alloc::{boxed::Box, string::ToString};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, to_value};

pub fn erase_handler<T: Route<Result: Serialize, Error: Serialize> + DeserializeOwned, S>(
    handler: impl Fn(T, &S) -> Result<T::Result, T::Error> + Send + Sync + 'static,
) -> Box<dyn Fn(Option<Params>, &S) -> Result<Value, Error> + Send + Sync> {
    Box::new(move |params, state| {
        let result = match from_params::<T>(params) {
            Ok(params) => handler(params, state),
            Err(err) => return Err(Error::invalid_params(Some(err.to_string()))),
        };
        match result {
            Ok(ok) => to_value(ok).map_err(|err| Error::internal_error(Some(err.to_string()))),
            Err(err) => Err(match to_value(err) {
                Ok(err) => Error::application_error(Some(err)),
                Err(err) => Error::internal_error(Some(err.to_string())),
            }),
        }
    })
}

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

#![no_std]

extern crate alloc;

mod batch;
#[cfg(feature = "client")]
pub mod client;
mod error;
mod id;
mod jsonrpc;
mod method;
mod params;
mod request;
mod response;
#[cfg(feature = "server")]
pub mod server;

pub use batch::{BatchRequest, BatchResponse};
pub use error::Error;
pub use id::Id;
pub use jsonrpc::JsonRpc;
pub use method::Method;
pub use params::Params;
pub use request::Request;
pub use response::Response;

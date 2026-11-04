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

use core::fmt;
use serde::{Deserialize, Serialize};

#[cfg(any(feature = "client", feature = "server"))]
use blacknet_json_rpc::{Method, peer::Route};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct WalletListResult(pub Vec<String>);

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum WalletListError {}

impl fmt::Display for WalletListError {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {}
    }
}

impl core::error::Error for WalletListError {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct WalletListParams;

#[cfg(any(feature = "client", feature = "server"))]
impl Route for WalletListParams {
    type Result = WalletListResult;
    type Error = WalletListError;

    const METHOD: Method = Method::new("wallet.v1.list");
}

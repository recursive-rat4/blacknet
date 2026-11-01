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

use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CookieAuthParams {
    #[serde(
        serialize_with = "const_hex::serde::no_prefix::serialize_upper",
        deserialize_with = "const_hex::serde::no_prefix::deserialize"
    )]
    pub cookie: [u8; 32],
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CookieAuthResult;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CookieAuthError(pub Cow<'static, str>);

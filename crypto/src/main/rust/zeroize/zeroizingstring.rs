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

use crate::zeroize::zeroize_string;
use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct ZeroizingString(String);

impl Drop for ZeroizingString {
    fn drop(&mut self) {
        zeroize_string(&mut self.0)
    }
}

impl AsRef<[u8]> for ZeroizingString {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

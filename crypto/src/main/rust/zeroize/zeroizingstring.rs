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
use core::{
    mem,
    ops::{Deref, DerefMut},
    ptr,
};
use serde::{Deserialize, Serialize};

/// String that is zeroized on drop.
///
/// # Safety
/// Reallocations may not zeroize previously allocated memory.
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct ZeroizingString(String);

impl ZeroizingString {
    /// Create an empty string for the `capacity`.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self(String::with_capacity(capacity))
    }

    /// Unwrap the string without zeroizing it.
    #[must_use]
    pub const fn into_inner(self) -> String {
        let string = unsafe { ptr::read(&self.0) };
        mem::forget(self);
        string
    }
}

impl From<String> for ZeroizingString {
    #[inline]
    fn from(string: String) -> Self {
        Self(string)
    }
}

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

impl Deref for ZeroizingString {
    type Target = String;

    #[inline]
    fn deref(&self) -> &String {
        &self.0
    }
}

impl DerefMut for ZeroizingString {
    #[inline]
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}

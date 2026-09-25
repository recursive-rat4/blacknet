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

use std::path::Path;

#[cfg(target_family = "unix")]
use core::ffi::c_char;
#[cfg(target_family = "unix")]
use std::os::unix::ffi::OsStrExt;

#[cfg(target_family = "unix")]
pub struct OsString(Vec<c_char>);

#[cfg(target_family = "unix")]
impl OsString {
    pub const fn as_ptr(&self) -> *const c_char {
        self.0.as_ptr()
    }
}

#[cfg(target_family = "unix")]
impl From<&Path> for OsString {
    fn from(path: &Path) -> Self {
        Self(
            #[allow(clippy::unnecessary_cast)]
            path.as_os_str()
                .as_bytes()
                .iter()
                .copied()
                .map(|c| c as c_char)
                .collect(),
        )
    }
}

#[cfg(target_family = "windows")]
use std::os::windows::ffi::OsStrExt;

#[cfg(target_family = "windows")]
pub struct OsString(Vec<u16>);

#[cfg(target_family = "windows")]
impl OsString {
    pub const fn as_ptr(&self) -> *const u16 {
        self.0.as_ptr()
    }
}

#[cfg(target_family = "windows")]
impl From<&Path> for OsString {
    fn from(path: &Path) -> Self {
        Self(path.as_os_str().encode_wide().chain([0u16]).collect())
    }
}

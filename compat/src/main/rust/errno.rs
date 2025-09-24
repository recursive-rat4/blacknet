/*
 * Copyright (c) 2025 Pavel Vasin
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

#[cfg(target_family = "unix")]
use core::ffi::CStr;
use thiserror::Error;

#[cfg(all(target_family = "unix", not(target_os = "macos")))]
pub fn errno() -> libc::c_int {
    unsafe { *libc::__errno_location() }
}

#[cfg(target_os = "macos")]
pub fn errno() -> libc::c_int {
    unsafe { *libc::__error() }
}

#[cfg(target_family = "unix")]
pub fn strerror<'a>(errno: libc::c_int) -> &'a CStr {
    unsafe { CStr::from_ptr(libc::strerror(errno)) }
}

#[derive(Debug, Error)]
pub enum Error {
    #[cfg(target_family = "unix")]
    #[error("{}", &strerror(*.0).to_string_lossy())]
    Errno(libc::c_int),
    #[error("{0}")]
    Message(String),
}

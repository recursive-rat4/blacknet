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

use core::ffi::CStr;
use core::fmt;

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

#[derive(Debug)]
pub enum Error {
    #[cfg(target_family = "unix")]
    Errno(libc::c_int),
    Message(String),
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            #[cfg(target_family = "unix")]
            Error::Errno(errno) => formatter.write_str(&strerror(*errno).to_string_lossy()),
            Error::Message(msg) => formatter.write_str(msg),
        }
    }
}

impl core::error::Error for Error {}

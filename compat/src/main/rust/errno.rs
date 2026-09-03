/*
 * Copyright (c) 2025-2026 Pavel Vasin
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

use core::{ffi::CStr, fmt};

#[derive(Debug)]
pub struct Errno {
    errno: libc::c_int,
}

impl Errno {
    pub fn get() -> Self {
        let ptr = Self::location();
        let errno = unsafe { *ptr };
        Self { errno }
    }

    pub const fn is_interrupted(&self) -> bool {
        self.errno == libc::EINTR
    }

    fn location() -> *mut libc::c_int {
        cfg_select! {
            target_os = "android" => unsafe { libc::__errno() },
            target_os = "illumos" => unsafe { libc::___errno() },
            target_os = "freebsd" => unsafe { libc::__error() },
            target_os = "haiku" => unsafe { libc::_errnop() },
            target_os = "macos" => unsafe { libc::__error() },
            target_os = "netbsd" => unsafe { libc::__errno() },
            target_os = "openbsd" => unsafe { libc::__errno() },
            _ => unsafe { libc::__errno_location() },
        }
    }

    fn strerror<T, F: FnOnce(&CStr) -> T>(&self, f: F) -> T {
        let s = unsafe { CStr::from_ptr(libc::strerror(self.errno)) };
        f(s)
    }
}

impl fmt::Display for Errno {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.strerror(|s| f.write_str(&s.to_string_lossy()))
    }
}

impl core::error::Error for Errno {}

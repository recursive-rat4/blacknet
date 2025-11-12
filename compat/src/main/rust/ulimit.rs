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

use crate::errno::Error;
use core::ffi::c_long;

#[cfg(target_family = "unix")]
use crate::errno::errno;
#[cfg(target_family = "unix")]
use core::cmp::min;
#[cfg(target_family = "unix")]
use core::mem::MaybeUninit;

#[cfg(target_family = "unix")]
pub fn ulimit() -> Result<c_long, Error> {
    let mut rlimit = MaybeUninit::<libc::rlimit>::uninit();
    unsafe {
        if libc::getrlimit(libc::RLIMIT_NOFILE, rlimit.as_mut_ptr()) == 0 {
            let rlimit = rlimit.assume_init();
            Ok(min(rlimit.rlim_cur, c_long::MAX as libc::rlim_t) as c_long)
        } else {
            Err(Error::Errno(errno()))
        }
    }
}

#[cfg(target_family = "windows")]
pub fn ulimit() -> Result<c_long, Error> {
    Ok(512)
}

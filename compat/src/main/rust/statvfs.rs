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

use crate::errno::Error;
use core::mem::MaybeUninit;
use std::path::Path;

#[cfg(target_family = "unix")]
use crate::errno::errno;
#[cfg(target_family = "unix")]
use core::ffi::c_char;
#[cfg(target_family = "unix")]
use std::os::unix::ffi::OsStrExt;

#[cfg(target_family = "unix")]
pub fn statvfs(path: &Path) -> Result<u64, Error> {
    let path = path.as_os_str().as_bytes().as_ptr() as *const c_char;
    let mut statvfs = MaybeUninit::<libc::statvfs>::uninit();
    loop {
        let rc = unsafe { libc::statvfs(path, statvfs.as_mut_ptr()) };
        if rc == 0 {
            let statvfs = unsafe { statvfs.assume_init() };
            let available = statvfs.f_bsize.saturating_mul(statvfs.f_bavail);
            return Ok(available);
        } else {
            let errno = errno();
            if errno == libc::EINTR {
                continue;
            }
            return Err(Error::Errno(errno));
        }
    }
}

#[cfg(target_family = "windows")]
use crate::Win32Error;
#[cfg(target_family = "windows")]
use std::os::windows::ffi::OsStrExt;
#[cfg(target_family = "windows")]
use windows_sys::Win32::Storage::FileSystem::GetDiskFreeSpaceExW;

#[cfg(target_family = "windows")]
pub fn statvfs(path: &Path) -> Result<u64, Error> {
    let path: Vec<u16> = path.as_os_str().encode_wide().collect();
    let available = MaybeUninit::<u64>::uninit();
    let rc = unsafe {
        GetDiskFreeSpaceExW(
            path.as_ptr(),
            available.as_mut_ptr(),
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        )
    };
    if rc != 0 {
        Ok(unsafe { available.assume_init() })
    } else {
        Err(Error::Win32(Win32Error::last()))
    }
}

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
use core::cmp::min;

#[cfg(target_family = "unix")]
use crate::errno::errno;

#[cfg(target_family = "unix")]
const GETENTROPY_MAX: usize = 256;

#[cfg(target_family = "unix")]
pub fn getentropy<const N: usize>() -> Result<[u8; N], Error> {
    let mut buf = [0_u8; N];
    let mut offset = 0;
    let mut remain = N;
    while remain > 0 {
        let process = min(remain, GETENTROPY_MAX);
        let ptr = unsafe { (buf.as_mut_ptr() as *mut libc::c_void).add(offset) };
        let rc = unsafe { libc::getentropy(ptr, process) };
        if rc == 0 {
            remain -= process;
            offset += process;
            continue;
        } else {
            return Err(Error::Errno(errno()));
        }
    }
    Ok(buf)
}

#[cfg(target_family = "windows")]
use windows_sys::Win32::Security::Authentication::Identity::RtlGenRandom;

#[cfg(target_family = "windows")]
const GETENTROPY_MAX: usize = u32::MAX as usize;

#[cfg(target_family = "windows")]
pub fn getentropy<const N: usize>() -> Result<[u8; N], Error> {
    let mut buf = [0_u8; N];
    let mut offset = 0;
    let mut remain = N;
    while remain > 0 {
        let process = min(remain, GETENTROPY_MAX);
        let ptr = unsafe { (buf.as_mut_ptr() as *mut core::ffi::c_void).add(offset) };
        let rc = unsafe { RtlGenRandom(ptr, process as u32) };
        if rc != false {
            remain -= process;
            offset += process;
            continue;
        } else {
            return Err(Error::Message("RtlGenRandom failed"));
        }
    }
    Ok(buf)
}

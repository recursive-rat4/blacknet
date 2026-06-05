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

#[cfg(target_family = "unix")]
use core::ffi::CStr;
#[cfg(target_family = "unix")]
use core::mem::MaybeUninit;

#[cfg(target_family = "unix")]
pub fn uname() -> (String, String, String) {
    let mut utsname = MaybeUninit::<libc::utsname>::uninit();
    let rc = unsafe { libc::uname(utsname.as_mut_ptr()) };
    if rc == 0 {
        let utsname = unsafe { utsname.assume_init() };
        unsafe {
            (
                CStr::from_ptr(utsname.sysname.as_ptr())
                    .to_string_lossy()
                    .into(),
                CStr::from_ptr(utsname.release.as_ptr())
                    .to_string_lossy()
                    .into(),
                CStr::from_ptr(utsname.machine.as_ptr())
                    .to_string_lossy()
                    .into(),
            )
        }
    } else {
        ("unknown".into(), "unknown".into(), "unknown".into())
    }
}

#[cfg(target_family = "windows")]
use platform_info::{PlatformInfo, PlatformInfoAPI, UNameAPI};

#[cfg(target_family = "windows")]
pub fn uname() -> (String, String, String) {
    match PlatformInfo::new() {
        Ok(info) => (
            info.sysname().to_string_lossy().into(),
            info.release().to_string_lossy().into(),
            info.machine().to_string_lossy().into(),
        ),
        Err(..) => ("unknown".into(), "unknown".into(), "unknown".into()),
    }
}

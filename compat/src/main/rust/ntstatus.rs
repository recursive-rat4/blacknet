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

use core::{ffi::c_void, fmt, ptr, slice};
use windows_sys::Win32::{
    Foundation::{
        ERROR_SUCCESS, GetLastError, LocalFree, NTSTATUS, RtlNtStatusToDosError, STATUS_SUCCESS,
        WIN32_ERROR,
    },
    System::Diagnostics::Debug::{
        FORMAT_MESSAGE_ALLOCATE_BUFFER, FORMAT_MESSAGE_FROM_SYSTEM, FormatMessageW,
    },
};

#[derive(Debug)]
pub enum Error {
    NtStatus(NtStatus),
    Win32(Win32Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NtStatus(nt_status) => write!(f, "{nt_status}"),
            Error::Win32(win32) => write!(f, "{win32}"),
        }
    }
}

impl core::error::Error for Error {}

#[derive(Debug)]
pub struct NtStatus {
    status: NTSTATUS,
}

impl NtStatus {
    pub const fn new(status: NTSTATUS) -> Self {
        Self { status }
    }

    pub const fn is_success(&self) -> bool {
        self.status == STATUS_SUCCESS
    }
}

impl fmt::Display for NtStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NTSTATUS (0x{:08X})", self.status)
    }
}

#[derive(Debug)]
pub struct Win32Error {
    error: WIN32_ERROR,
}

impl Win32Error {
    pub const fn new(error: WIN32_ERROR) -> Self {
        Self { error }
    }

    pub const fn is_success(&self) -> bool {
        self.error == ERROR_SUCCESS
    }

    pub fn last() -> Self {
        Self {
            error: unsafe { GetLastError() },
        }
    }

    fn format_message(&self) -> String {
        let ret: String;
        unsafe {
            let flags = FORMAT_MESSAGE_ALLOCATE_BUFFER | FORMAT_MESSAGE_FROM_SYSTEM;
            let mut buffer: *mut u16 = ptr::null_mut();
            let n = FormatMessageW(
                flags,
                ptr::null(),
                self.error,
                0,
                &mut buffer as *mut _ as *mut u16,
                0,
                ptr::null(),
            );
            if n != 0 {
                let v = slice::from_raw_parts(buffer as *const u8, n as usize * size_of::<u16>());
                ret = String::from_utf16le_lossy(v);
            } else {
                ret = format!("WIN32 ERROR (0x{:08X})", self.error);
            }
            LocalFree(buffer as *mut c_void);
        }
        ret
    }
}

impl From<NtStatus> for Win32Error {
    fn from(status: NtStatus) -> Self {
        Self {
            error: unsafe { RtlNtStatusToDosError(status.status) },
        }
    }
}

impl fmt::Display for Win32Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_message())
    }
}

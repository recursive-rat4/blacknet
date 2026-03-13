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

use core::fmt;
use windows_sys::Win32::Foundation::{
    ERROR_SUCCESS, NTSTATUS, RtlNtStatusToDosError, STATUS_SUCCESS, WIN32_ERROR,
};

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
}

impl From<NtStatus> for Win32Error {
    fn from(status: NtStatus) -> Self {
        Self {
            error: unsafe { RtlNtStatusToDosError(status.status) },
        }
    }
}

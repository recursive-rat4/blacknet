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

pub mod config;
#[cfg(target_family = "unix")]
mod errno;
pub mod feerate;
mod getentropy;
mod getuid;
mod magic;
mod mode;
#[cfg(target_family = "windows")]
mod ntstatus;
mod osstring;
pub mod size;
mod statvfs;
mod ulimit;
mod uname;
mod xdgdirectories;

#[cfg(target_family = "unix")]
pub use errno::Errno;
pub use getentropy::*;
pub use getuid::*;
pub use mode::{Mode, mode};
#[cfg(target_family = "windows")]
pub use ntstatus::{Error, NtStatus, Win32Error};
pub use osstring::OsString;
pub use statvfs::*;
pub use ulimit::*;
pub use uname::*;
pub use xdgdirectories::XDGDirectories;

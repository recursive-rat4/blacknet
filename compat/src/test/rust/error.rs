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

#[cfg(target_family = "unix")]
#[test]
fn errno() {
    use blacknet_compat::Errno;

    let error = Errno::get();
    let _ = format!("{error}");
}

#[cfg(target_family = "windows")]
#[test]
fn win32() {
    use blacknet_compat::Win32Error;

    let error = Win32Error::last();
    let _ = format!("{error}");
}

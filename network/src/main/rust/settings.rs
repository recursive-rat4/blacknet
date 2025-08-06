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

use blacknet_compat::mode::Mode;

#[derive(Clone, Copy)]
pub struct Settings {
    pub port: u16,
    pub ipv4: bool,
    pub ipv6: bool,
    pub tor: bool,
    pub i2p: bool,
    pub log_endpoint: bool,
}

impl Settings {
    pub fn new(_mode: &Mode) -> Self {
        Self {
            port: 28453,
            ipv4: true,
            ipv6: true,
            tor: true,
            i2p: true,
            log_endpoint: false,
        }
    }
}

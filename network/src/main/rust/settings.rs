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

pub struct Settings {
    pub port: u16,
    pub ipv4: bool,
    pub ipv6: bool,
    pub tor: bool,
    pub i2p: bool,
    pub natpmp: bool,
    pub log_endpoint: bool,
    pub tor_control_host: String,
    pub tor_control_port: u16,
}

impl Settings {
    pub fn default(mode: &Mode) -> Self {
        Self {
            port: mode.default_p2p_port(),
            ipv4: mode.requires_network(),
            ipv6: mode.requires_network(),
            tor: mode.requires_network(),
            i2p: mode.requires_network(),
            natpmp: mode.requires_network(),
            log_endpoint: !mode.requires_network(),
            tor_control_host: "127.0.0.1".to_string(),
            tor_control_port: 9051,
        }
    }
}

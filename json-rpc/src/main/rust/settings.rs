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

use blacknet_compat::Mode;

pub struct Settings {
    pub enabled: bool,
    pub host: String,
    pub port: u16,
}

impl Settings {
    pub fn default(mode: &Mode) -> Self {
        Self {
            enabled: true,
            host: "127.0.0.1".to_owned(),
            port: mode.default_rpc_port(),
        }
    }
}

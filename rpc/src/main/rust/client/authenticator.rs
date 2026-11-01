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

use blacknet_compat::XDGDirectories;
use std::{
    fs::File,
    io::{Error as IoError, Read},
    path::PathBuf,
};

pub struct Authenticator {
    path: PathBuf,
}

impl Authenticator {
    pub fn new(dirs: &XDGDirectories) -> Self {
        Self {
            path: dirs.state().join("rpc.cookie"),
        }
    }

    pub fn read_cookie(&self) -> Result<[u8; 32], IoError> {
        let mut file = File::open(&self.path)?;
        let mut cookie = [0; 32];
        file.read_exact(&mut cookie)?;
        Ok(cookie)
    }
}

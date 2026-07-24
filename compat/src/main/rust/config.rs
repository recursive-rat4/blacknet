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

use crate::Mode;
use core::error::Error;
use serde::Deserialize;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use std::sync::Arc;
use toml::from_str;

#[derive(Deserialize)]
pub struct Config {
    pub network: Arc<Network>,
    pub rpc: RPC,
}

impl Config {
    pub fn load(config_dir: &Path) -> Result<Self, Box<dyn Error>> {
        let path = config_dir.join("blacknet.conf");
        let string = fs::read_to_string(path)?;
        Ok(from_str::<Config>(&string)?)
    }

    pub fn load_or_create(mode: &Mode, config_dir: &Path) -> Result<Self, Box<dyn Error>> {
        let path = config_dir.join("blacknet.conf");
        let string = match fs::read_to_string(&path) {
            Ok(string) => string,
            Err(err) => {
                if err.kind() == ErrorKind::NotFound {
                    let s = mode.blacknet_conf();
                    fs::write(path, s)?;
                    s.to_owned()
                } else {
                    return Err(Box::new(err));
                }
            }
        };
        Ok(from_str::<Config>(&string)?)
    }
}

#[derive(Deserialize)]
pub struct Network {
    pub port: u16,
    pub ipv4: bool,
    pub ipv6: bool,
    pub tor: bool,
    pub i2p: bool,
    pub natpmp: bool,
    pub incoming_connections: u16,
    pub outgoing_connections: u16,
    pub log_endpoint: bool,
    pub i2p_sam_host: String,
    pub i2p_sam_port: u16,
    pub tor_control_host: String,
    pub tor_control_port: u16,
    pub db_cache: u64,
    pub tx_pool_size: usize,
    pub min_relay_fee_rate: u64,
}

#[derive(Deserialize)]
pub struct RPC {
    pub enabled: bool,
    pub host: String,
    pub port: u16,
}

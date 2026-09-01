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
use core::fmt;
use serde::Deserialize;
use std::fs;
use std::io::{Error as IoError, ErrorKind};
use std::path::Path;
use std::sync::Arc;
use toml::{de::Error as TomlError, from_str};

#[derive(Deserialize)]
pub struct Config {
    pub network: Arc<Network>,
    pub rpc: RPC,
}

impl Config {
    pub fn load(config_dir: &Path) -> Result<Self> {
        let config_file = config_dir.join("blacknet.conf");
        let conf = fs::read_to_string(config_file)?;
        Self::parse(&conf)
    }

    pub fn load_or_create(mode: &Mode, config_dir: &Path) -> Result<Self> {
        let config_file = config_dir.join("blacknet.conf");
        let conf = match fs::read_to_string(&config_file) {
            Ok(conf) => conf,
            Err(err) => {
                if err.kind() == ErrorKind::NotFound {
                    let conf = mode.blacknet_conf();
                    fs::write(config_file, conf)?;
                    conf.to_owned()
                } else {
                    return Err(Error::Io(err));
                }
            }
        };
        Self::parse(&conf)
    }

    pub fn parse(string: &str) -> Result<Self> {
        Ok(from_str::<Config>(string)?)
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
    pub db_cache: u64,
    pub soft_block_size_limit: u32,
    pub tx_pool_size: usize,
    pub min_relay_fee_rate: u64,
    pub proxy: Option<HostPort>,
    pub i2p_sam: HostPort,
    pub tor_proxy: HostPort,
    pub tor_control: HostPort,
}

#[derive(Deserialize)]
pub struct RPC {
    pub enabled: bool,
    pub bind: HostPort,
}

#[derive(Deserialize)]
pub struct HostPort {
    pub host: String,
    pub port: u16,
}

#[derive(Debug)]
pub enum Error {
    Io(IoError),
    Toml(TomlError),
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Error::Io(err)
    }
}

impl From<TomlError> for Error {
    fn from(err: TomlError) -> Self {
        Error::Toml(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "{err}"),
            Self::Toml(err) => write!(f, "{err}"),
        }
    }
}

impl core::error::Error for Error {}

pub type Result<T, E = Error> = core::result::Result<T, E>;

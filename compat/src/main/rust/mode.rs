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

use crate::magic::*;
use std::env::VarError;

/**
 * An abstract mode of operation: production or various research, development, testing.
 */
pub struct Mode {
    subdirectory: Option<&'static str>,
    address_prefix: Option<&'static str>,
    requires_network: bool,
    agent_name: String,
    default_p2p_port: u16,
    network_magic: u32,
}

impl Mode {
    fn new(
        ordinal: u8,
        agent_suffix: Option<&'static str>,
        subdirectory: Option<&'static str>,
        address_prefix: Option<&'static str>,
        requires_network: bool,
    ) -> Self {
        Self {
            subdirectory,
            address_prefix,
            requires_network,
            agent_name: match agent_suffix {
                Some(val) => format!("{AGENT_NAME}{val}"),
                None => AGENT_NAME.to_string(),
            },
            default_p2p_port: DEFAULT_P2P_PORT + ordinal as u16,
            network_magic: NETWORK_MAGIC + ordinal as u32,
        }
    }

    /**
     * The main network. It's the production mode.
     */
    fn mainnet() -> Self {
        Self::new(0, None, None, None, true)
    }

    #[allow(dead_code)]
    fn testnet() -> Self {
        Self::new(1, Some("-TestNet"), Some("TestNet"), Some("t"), true)
    }

    /**
     * A regression testing mode. Usually it's a sole offline node,
     * or else it can be a tiny private network.
     */
    fn regtest() -> Self {
        Self::new(3, Some("-RegTest"), Some("RegTest"), Some("r"), false)
    }

    fn default() -> Self {
        Self::mainnet()
    }

    /**
     * A subdirectory to separate data.
     */
    pub fn subdirectory(&self) -> Option<&'static str> {
        self.subdirectory
    }
    /**
     * An address prefix to designate a different network.
     */
    pub fn address_prefix(&self) -> Option<&'static str> {
        self.address_prefix
    }
    /**
     * Whether the node requires network peers.
     */
    pub fn requires_network(&self) -> bool {
        self.requires_network
    }

    /**
     * An agent name for network indication.
     */
    pub fn agent_name(&self) -> &str {
        &self.agent_name
    }

    pub fn default_p2p_port(&self) -> u16 {
        self.default_p2p_port
    }

    pub fn network_magic(&self) -> u32 {
        self.network_magic
    }
}

/**
 * A `Mode` the program is running in. By default, it's `MainNet`.
 */
pub fn mode() -> Result<Mode, String> {
    match std::env::var("BLACKNET_MODE") {
        Err(VarError::NotPresent) => Ok(Mode::default()),
        Ok(val) => match val.as_str() {
            "MainNet" => Ok(Mode::mainnet()),
            "TestNet" => Err("TestNet was not tested".into()),
            "RegTest" => Ok(Mode::regtest()),
            _ => Err(format!(
                "Unrecognized mode: {val}. Possible values: MainNet, RegTest."
            )),
        },
        Err(VarError::NotUnicode(_)) => {
            Err("Not unicode data in environment variable BLACKNET_MODE".into())
        }
    }
}

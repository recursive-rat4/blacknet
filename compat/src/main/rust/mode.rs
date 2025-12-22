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
    address_readable_part: String,
    message_sign_name: String,
    requires_network: bool,
    agent_name: String,
    default_p2p_port: u16,
    default_rpc_port: u16,
    network_magic: u32,
    builtin_peers: &'static str,
    genesis_json: &'static str,
}

impl Mode {
    fn new(
        ordinal: u8,
        agent_suffix: Option<&'static str>,
        subdirectory: Option<&'static str>,
        address_prefix: Option<&'static str>,
        sign_suffix: Option<&'static str>,
        requires_network: bool,
        builtin_peers: &'static str,
        genesis_json: &'static str,
    ) -> Self {
        Self {
            subdirectory,
            address_readable_part: match address_prefix {
                Some(val) => format!("{val}{ADDRESS_READABLE_PART}"),
                None => ADDRESS_READABLE_PART.to_owned(),
            },
            message_sign_name: match sign_suffix {
                Some(val) => format!("{MESSAGE_SIGN_NAME}{val}"),
                None => MESSAGE_SIGN_NAME.to_owned(),
            },
            requires_network,
            agent_name: match agent_suffix {
                Some(val) => format!("{AGENT_NAME}{val}"),
                None => AGENT_NAME.to_owned(),
            },
            default_p2p_port: DEFAULT_P2P_PORT + ordinal as u16,
            default_rpc_port: DEFAULT_RPC_PORT + ordinal as u16,
            network_magic: NETWORK_MAGIC + ordinal as u32,
            builtin_peers,
            genesis_json,
        }
    }

    /**
     * The main network. It's the production mode.
     */
    pub fn mainnet() -> Self {
        let peers_txt = include_str!("../../../../kernel/src/main/resources/peers.txt");
        let genesis_json = include_str!("../../../../kernel/src/main/resources/genesis.json");
        Self::new(0, None, None, None, None, true, peers_txt, genesis_json)
    }

    pub fn testnet() -> Self {
        Self::new(
            1,
            Some("-TestNet"),
            Some("TestNet"),
            Some("t"),
            Some(" TestNet"),
            true,
            "",
            "",
        )
    }

    /**
     * A regression testing mode. Usually it's a sole offline node,
     * or else it can be a tiny private network.
     */
    pub fn regtest() -> Self {
        let genesis_json = include_str!("../../../../kernel/src/main/resources/regtest.json");
        Self::new(
            3,
            Some("-RegTest"),
            Some("RegTest"),
            Some("r"),
            Some(" RegTest"),
            false,
            "",
            genesis_json,
        )
    }

    fn default() -> Self {
        Self::mainnet()
    }

    /**
     * A subdirectory to separate data.
     */
    pub const fn subdirectory(&self) -> Option<&'static str> {
        self.subdirectory
    }
    /**
     * An address readable part to designate a different network.
     */
    pub fn address_readable_part(&self) -> &str {
        &self.address_readable_part
    }
    /**
     * A message sign name to personalize a digital text.
     */
    pub fn message_sign_name(&self) -> &str {
        &self.message_sign_name
    }
    /**
     * Whether the node requires network peers.
     */
    pub const fn requires_network(&self) -> bool {
        self.requires_network
    }

    /**
     * An agent name for network indication.
     */
    pub fn agent_name(&self) -> &str {
        &self.agent_name
    }

    pub const fn default_p2p_port(&self) -> u16 {
        self.default_p2p_port
    }

    pub const fn default_rpc_port(&self) -> u16 {
        self.default_rpc_port
    }

    pub const fn network_magic(&self) -> u32 {
        self.network_magic
    }

    pub const fn builtin_peers(&self) -> &'static str {
        self.builtin_peers
    }

    pub const fn genesis_json(&self) -> &'static str {
        self.genesis_json
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

/*
 * Copyright (c) 2018-2025 Pavel Vasin
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

use crate::v2::EndpointInfo;
use blacknet_network::node::{Node, PROTOCOL_VERSION};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct NodeInfo {
    agent: String,
    name: String,
    version: String,
    protocolVersion: u32,
    outgoing: u32,
    incoming: u32,
    listening: Vec<EndpointInfo>,
    warnings: Vec<String>,
}

impl NodeInfo {
    pub fn new(node: &Node) -> Self {
        let listening = node.listening().read().unwrap();
        Self {
            agent: node.agent_string().to_owned(),
            name: node.agent_name().to_owned(),
            version: node.agent_version().to_owned(),
            protocolVersion: PROTOCOL_VERSION,
            outgoing: node.outgoing() as u32,
            incoming: node.incoming() as u32,
            listening: listening.iter().copied().map(EndpointInfo::from).collect(),
            warnings: node.warnings(),
        }
    }
}

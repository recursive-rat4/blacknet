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

use blacknet_compat::Mode;
use blacknet_kernel::amount::Amount;

pub struct Settings {
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
    pub min_relay_fee_rate: Amount,
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
            incoming_connections: 128,
            outgoing_connections: 8,
            log_endpoint: !mode.requires_network(),
            i2p_sam_host: "127.0.0.1".to_owned(),
            i2p_sam_port: 7656,
            tor_control_host: "127.0.0.1".to_owned(),
            tor_control_port: 9051,
            db_cache: 256 * 1024 * 1024,
            tx_pool_size: 128 * 1024 * 1024,
            min_relay_fee_rate: Amount::new(100000), // 0.001
        }
    }
}

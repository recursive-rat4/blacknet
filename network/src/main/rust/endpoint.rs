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

use core::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use core::str::FromStr;
use data_encoding::Encoding;
use data_encoding_macro::new_encoding;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[repr(u8)]
pub enum Endpoint {
    IPv4 { port: u16, address: [u8; 4] } = 128,
    IPv6 { port: u16, address: [u8; 16] } = 129,
    TORv2 { port: u16, address: [u8; 10] } = 130,
    TORv3 { port: u16, address: [u8; 32] } = 131,
    I2P { port: u16, address: [u8; 32] } = 132,
}

impl Endpoint {
    #[rustfmt::skip]
    pub fn parse(string: &str, port: u16) -> Option<Self> {
        parse_i2p(string, port).or_else(||
        parse_torv3(string, port).or_else(||
        parse_ipv6(string, port).or_else(||
        parse_ipv4(string, port))))
    }

    pub fn is_permissionless(self) -> bool {
        match self {
            Endpoint::IPv4 { .. } => false,
            Endpoint::IPv6 { .. } => false,
            Endpoint::TORv2 { .. } => true,
            Endpoint::TORv3 { .. } => true,
            Endpoint::I2P { .. } => true,
        }
    }

    pub fn is_local(self) -> bool {
        match self {
            Endpoint::IPv4 { port: _, address } => is_local_ipv4(address),
            Endpoint::IPv6 { port: _, address } => is_local_ipv6(address),
            Endpoint::TORv2 { .. } => false,
            Endpoint::TORv3 { .. } => false,
            Endpoint::I2P { .. } => false,
        }
    }

    pub fn is_private(self) -> bool {
        match self {
            Endpoint::IPv4 { port: _, address } => is_private_ipv4(address),
            Endpoint::IPv6 { port: _, address } => is_private_ipv6(address),
            Endpoint::TORv2 { .. } => false,
            Endpoint::TORv3 { .. } => false,
            Endpoint::I2P { .. } => false,
        }
    }

    pub fn to_rust(self) -> Option<SocketAddr> {
        match self {
            Endpoint::IPv4 { port, address } => Some(SocketAddr::V4(SocketAddrV4::new(
                Ipv4Addr::from(address),
                port,
            ))),
            Endpoint::IPv6 { port, address } => Some(SocketAddr::V6(SocketAddrV6::new(
                Ipv6Addr::from(address),
                port,
                0,
                0,
            ))),
            Endpoint::TORv2 { .. } => None,
            Endpoint::TORv3 { .. } => None,
            Endpoint::I2P { .. } => None,
        }
    }

    pub fn to_host(self) -> String {
        match self {
            Endpoint::IPv4 { port: _, address } => to_host_ipv4(address),
            Endpoint::IPv6 { port: _, address } => to_host_ipv6(address),
            Endpoint::TORv2 { port: _, address } => format!("{address:?}"),
            Endpoint::TORv3 { port: _, address } => to_host_torv3(address),
            Endpoint::I2P { port: _, address } => to_host_i2p(address),
        }
    }

    pub fn to_log(self, detail: bool) -> String {
        match self {
            Endpoint::IPv4 { port, address } => to_log_ipv4(port, address, detail),
            Endpoint::IPv6 { port, address } => to_log_ipv6(port, address, detail),
            Endpoint::TORv2 { .. } => "TORv2".to_string(),
            Endpoint::TORv3 { port, address } => to_log_torv3(port, address, detail),
            Endpoint::I2P { port, address } => to_log_i2p(port, address, detail),
        }
    }
}

fn parse_ipv4(string: &str, port: u16) -> Option<Endpoint> {
    if let Ok(addr) = Ipv4Addr::from_str(string) {
        Some(Endpoint::IPv4 {
            port,
            address: addr.octets(),
        })
    } else {
        None
    }
}

fn is_local_ipv4(address: [u8; 4]) -> bool {
    // 0.0.0.0 – 0.255.255.255
    if address[0] == 0 {
        return true;
    }
    // 127.0.0.0 – 127.255.255.255
    if address[0] == 127 {
        return true;
    }
    // 169.254.0.0 – 169.254.255.255
    if address[0] == 169 && address[1] == 254 {
        return true;
    }

    false
}

fn is_private_ipv4(address: [u8; 4]) -> bool {
    // 10.0.0.0 – 10.255.255.255
    if address[0] == 10 {
        return true;
    }
    // 100.64.0.0 – 100.127.255.255
    if address[0] == 100 && address[1] >= 64 && address[1] <= 127 {
        return true;
    }
    // 172.16.0.0 – 172.31.255.255
    if address[0] == 172 && address[1] >= 16 && address[1] <= 31 {
        return true;
    }
    // 192.0.0.0 – 192.0.0.255
    if address[0] == 192 && address[1] == 0 && address[2] == 0 {
        return true;
    }
    // 192.168.0.0 – 192.168.255.255
    if address[0] == 192 && address[1] == 168 {
        return true;
    }
    // 198.18.0.0 – 198.19.255.255
    if address[0] == 198 && address[1] >= 18 && address[1] <= 19 {
        return true;
    }

    false
}

fn to_host_ipv4(address: [u8; 4]) -> String {
    let addr = Ipv4Addr::from(address);
    format!("{addr}")
}

fn to_log_ipv4(port: u16, address: [u8; 4], detail: bool) -> String {
    if detail {
        format!("{}:{}", to_host_ipv4(address), port)
    } else {
        "IPv4 endpoint".to_string()
    }
}

const IPV4_ANY_ADDRESS: [u8; 4] = [0, 0, 0, 0];
const IPV4_LOOPBACK_ADDRESS: [u8; 4] = [127, 0, 0, 1];

fn parse_ipv6(string: &str, port: u16) -> Option<Endpoint> {
    if let Ok(addr) = Ipv6Addr::from_str(string) {
        Some(Endpoint::IPv6 {
            port,
            address: addr.octets(),
        })
    } else {
        None
    }
}

fn is_local_ipv6(address: [u8; 16]) -> bool {
    // ::
    if address == IPV6_ANY_ADDRESS {
        return true;
    }
    // ::1
    if address == IPV6_LOOPBACK_ADDRESS {
        return true;
    }
    // fe80:: - febf:ffff:ffff:ffff:ffff:ffff:ffff:ffff
    if address[0] == 0xFE
        && address[1] == 0x80
        && address[2] == 0x00
        && address[3] == 0x00
        && address[4] == 0x00
        && address[5] == 0x00
        && address[6] == 0x00
        && address[7] == 0x00
    {
        return true;
    }

    false
}

fn is_private_ipv6(address: [u8; 16]) -> bool {
    // 0200:: - 03ff:ffff:ffff:ffff:ffff:ffff:ffff:ffff
    if (address[0] & 0xFE) == 0x02 {
        return true;
    }
    // fc00:: - fdff:ffff:ffff:ffff:ffff:ffff:ffff:ffff
    if (address[0] & 0xFE) == 0xFC {
        return true;
    }

    false
}

fn to_host_ipv6(address: [u8; 16]) -> String {
    let addr = Ipv6Addr::from(address);
    format!("{addr}")
}

fn to_log_ipv6(port: u16, address: [u8; 16], detail: bool) -> String {
    if detail {
        format!("[{}]:{}", to_host_ipv6(address), port)
    } else {
        "IPv6 endpoint".to_string()
    }
}

const IPV6_ANY_ADDRESS: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
const IPV6_LOOPBACK_ADDRESS: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];

const BASE32: Encoding = new_encoding! {
    symbols: "abcdefghijklmnopqrstuvwxyz234567",
    padding: None,
};

fn checksum_torv3(public_key: [u8; 32]) -> [u8; 2] {
    let mut hasher = Sha3_256::new();
    hasher.update(b".onion checksum");
    hasher.update(public_key);
    hasher.update([3_u8]);
    let hash = hasher.finalize();
    hash.as_slice()[0..2].try_into().expect("hash.len() == 32")
}

fn parse_torv3(string: &str, port: u16) -> Option<Endpoint> {
    if !string.ends_with(TOR_SUFFIX) {
        return None;
    }
    let defix = &string.as_bytes()[..string.len() - TOR_SUFFIX.len()];
    if let Ok(bytes) = BASE32.decode(defix) {
        if bytes.len() != 35 {
            return None;
        }
        let vrsn = bytes[34];
        if vrsn != 3 {
            return None;
        }
        let pubkey: [u8; 32] = bytes[0..32].try_into().expect("bytes.len() == 35");
        let chksum = &bytes[32..34];
        if checksum_torv3(pubkey) != chksum {
            return None;
        }
        Some(Endpoint::TORv3 {
            port,
            address: pubkey,
        })
    } else {
        None
    }
}

fn to_host_torv3(address: [u8; 32]) -> String {
    let chksum = checksum_torv3(address);
    let mut bytes: [u8; 35] = [0; 35];
    bytes[0..32].copy_from_slice(&address);
    bytes[32..34].copy_from_slice(&chksum);
    bytes[34] = 3;
    format!("{}{}", BASE32.encode(&bytes), TOR_SUFFIX)
}

fn to_log_torv3(port: u16, address: [u8; 32], detail: bool) -> String {
    if detail {
        format!("{}:{}", to_host_torv3(address), port)
    } else {
        "TORv3 endpoint".to_string()
    }
}

const TOR_SUFFIX: &str = ".onion";

fn parse_i2p(string: &str, port: u16) -> Option<Endpoint> {
    if !string.ends_with(I2P_SUFFIX) {
        return None;
    }
    let defix = &string.as_bytes()[..string.len() - I2P_SUFFIX.len()];
    if let Ok(bytes) = BASE32.decode(defix) {
        if bytes.len() != 32 {
            return None;
        }
        let address: [u8; 32] = bytes.try_into().expect("bytes.len() == 32");
        Some(Endpoint::I2P { port, address })
    } else {
        None
    }
}

fn to_host_i2p(address: [u8; 32]) -> String {
    format!("{}{}", BASE32.encode(&address), I2P_SUFFIX)
}

fn to_log_i2p(port: u16, address: [u8; 32], detail: bool) -> String {
    if detail {
        format!("{}:{}", to_host_i2p(address), port)
    } else {
        "I2P endpoint".to_string()
    }
}

const I2P_SUFFIX: &str = ".b32.i2p";

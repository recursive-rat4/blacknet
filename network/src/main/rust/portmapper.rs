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

use crate::endpoint::Endpoint;
use blacknet_io::Error as IoError;
use core::{
    fmt,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
};
use igd_next::{
    AddPortError, Error as IgdError, GetExternalIpError, PortMappingProtocol, RemovePortError,
    SearchOptions,
    aio::{
        Gateway,
        tokio::{Tokio, search_gateway},
    },
};
use std::net::UdpSocket;
use tokio::time::{Duration, sleep};

const DURATION: u32 = 3600; // 1 hour

pub struct PortMapper {
    gateway: Gateway<Tokio>,
}

impl PortMapper {
    pub async fn new() -> Result<Self, IgdError> {
        let gateway = search_gateway(SearchOptions::default()).await?;
        Ok(Self { gateway })
    }

    pub async fn add_port(&self, port: u16) -> Result<Endpoint, Error> {
        self.gateway
            .add_port(
                PortMappingProtocol::TCP,
                port,
                SocketAddrV4::new(self.internal_addr()?, port).into(),
                DURATION,
                "Blacknet",
            )
            .await?;
        let external = self.gateway.get_external_ip().await?;
        Ok((external, port).into())
    }

    pub async fn remove_port(&self, port: u16) -> Result<(), RemovePortError> {
        self.gateway
            .remove_port(PortMappingProtocol::TCP, port)
            .await
    }

    pub async fn sleep(&self) {
        sleep(Duration::from_secs(DURATION.into())).await
    }

    fn internal_addr(&self) -> Result<Ipv4Addr, Error> {
        let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0))?;
        socket.connect(self.gateway.addr)?;
        match socket.local_addr()? {
            SocketAddr::V4(addr) => Ok(*addr.ip()),
            SocketAddr::V6(..) => Err(Error::InternalAddr),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Io(IoError),
    Igd(IgdError),
    InternalAddr,
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Self::Io(err)
    }
}

impl From<IgdError> for Error {
    fn from(err: IgdError) -> Self {
        Self::Igd(err)
    }
}

impl From<AddPortError> for Error {
    fn from(err: AddPortError) -> Self {
        Self::Igd(err.into())
    }
}

impl From<GetExternalIpError> for Error {
    fn from(err: GetExternalIpError) -> Self {
        Self::Igd(err.into())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "{err}"),
            Self::Igd(err) => write!(f, "{err}"),
            Self::InternalAddr => f.write_str("Failed to get internal IPv4"),
        }
    }
}

impl core::error::Error for Error {}

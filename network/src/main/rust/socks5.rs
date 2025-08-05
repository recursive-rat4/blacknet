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

use crate::endpoint::Endpoint;
use core::fmt;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufStream};
use tokio::net::TcpStream;

const VERSION: u8 = 5;
const NO_AUTHENTICATION: u8 = 0;
const TCP_CONNECTION: u8 = 1;
const REQUEST_GRANTED: u8 = 0;
const IPV4_ADDRESS: u8 = 1;
const DOMAIN_NAME: u8 = 3;
const IPV6_ADDRESS: u8 = 4;

pub async fn socks5(proxy: Endpoint, destination: Endpoint) -> Result<BufStream<TcpStream>, Error> {
    let endpoint = proxy.to_rust().ok_or(Error::Endpoint)?;
    let socket = TcpStream::connect(endpoint).await?;
    let mut stream = BufStream::new(socket);

    stream.write_u8(VERSION).await?;
    stream.write_u8(1).await?; // number of authentication methods supported
    stream.write_u8(NO_AUTHENTICATION).await?;
    stream.flush().await?;

    let version = stream.read_u8().await?;
    if version != VERSION {
        return Err(Error::Version(version));
    }
    let authentication = stream.read_u8().await?;
    if authentication != NO_AUTHENTICATION {
        return Err(Error::Authentication(authentication));
    }

    stream.write_u8(VERSION).await?;
    stream.write_u8(TCP_CONNECTION).await?;
    stream.write_u8(0).await?; // reserved
    let port = match destination {
        Endpoint::IPv4 { port, address } => {
            stream.write_u8(IPV4_ADDRESS).await?;
            stream.write_all(&address).await?;
            port
        }
        Endpoint::IPv6 { port, address } => {
            stream.write_u8(IPV6_ADDRESS).await?;
            stream.write_all(&address).await?;
            port
        }
        Endpoint::TORv3 { port, address: _ } => {
            let domainname = destination.to_host();
            let bytes = domainname.as_bytes();
            if bytes.len() <= u8::MAX.into() {
                stream.write_u8(DOMAIN_NAME).await?;
                stream.write_u8(bytes.len() as u8).await?;
                stream.write_all(bytes).await?;
                port
            } else {
                return Err(Error::Destination);
            }
        }
        _ => return Err(Error::Destination),
    };
    stream.write_u16(port).await?;
    stream.flush().await?;

    let version = stream.read_u8().await?;
    if version != VERSION {
        return Err(Error::Version(version));
    }
    let reply = stream.read_u8().await?;
    if reply != REQUEST_GRANTED {
        return Err(Error::NotGranted(reply));
    }
    let reserved = stream.read_u8().await?;
    if reserved != 0 {
        return Err(Error::Reserved(reserved));
    }
    let address_type = stream.read_u8().await?;
    match address_type {
        IPV4_ADDRESS => stream.read_exact(&mut [0u8; 4 + 2]).await?,
        IPV6_ADDRESS => stream.read_exact(&mut [0u8; 16 + 2]).await?,
        DOMAIN_NAME => {
            let mut buf = [0u8; u8::MAX as usize];
            let len = stream.read_u8().await?;
            let slice = &mut buf[..len as usize + 2];
            stream.read_exact(slice).await?
        }
        _ => return Err(Error::Unknown(address_type)),
    };

    Ok(stream)
}

#[derive(Debug)]
pub enum Error {
    Endpoint,
    Destination,
    Version(u8),
    Authentication(u8),
    NotGranted(u8),
    Unknown(u8),
    Reserved(u8),
    Io(std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Endpoint => formatter.write_str("Unsupported proxy endpoint"),
            Error::Destination => formatter.write_str("Unsupported destination endpoint"),
            Error::Version(version) => write!(formatter, "Unknown socks version {version}"),
            Error::Authentication(authentication) => {
                write!(formatter, "Authentication not accepted ({authentication})")
            }
            Error::NotGranted(reply) => write!(formatter, "Access not granted ({reply})"),
            Error::Unknown(unknown) => write!(formatter, "Unknown socks reply ({unknown})"),
            Error::Reserved(reserved) => write!(formatter, "Reserved socks reply ({reserved})"),
            Error::Io(err) => write!(formatter, "{err}"),
        }
    }
}

impl core::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

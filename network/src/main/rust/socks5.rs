/*
 * Copyright (c) 2018-2026 Pavel Vasin
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
use std::io::Error as IoError;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::TcpStream;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

const VERSION: u8 = 5;
const NO_AUTHENTICATION: u8 = 0;
const TCP_CONNECTION: u8 = 1;
const REQUEST_GRANTED: u8 = 0;
const IPV4_ADDRESS: u8 = 1;
const DOMAIN_NAME: u8 = 3;
const IPV6_ADDRESS: u8 = 4;

pub async fn socks5(
    proxy: Endpoint,
    destination: Endpoint,
) -> Result<(BufReader<OwnedReadHalf>, BufWriter<OwnedWriteHalf>), Error> {
    let endpoint = proxy.to_rust().ok_or(Error::Endpoint)?;
    let socket = TcpStream::connect(endpoint).await?;
    let (tcp_read, tcp_write) = socket.into_split();
    let (mut buf_reader, mut buf_writer) = (BufReader::new(tcp_read), BufWriter::new(tcp_write));

    buf_writer.write_u8(VERSION).await?;
    buf_writer.write_u8(1).await?; // number of authentication methods supported
    buf_writer.write_u8(NO_AUTHENTICATION).await?;
    buf_writer.flush().await?;

    let version = buf_reader.read_u8().await?;
    if version != VERSION {
        return Err(Error::Version(version));
    }
    let authentication = buf_reader.read_u8().await?;
    if authentication != NO_AUTHENTICATION {
        return Err(Error::Authentication(authentication));
    }

    buf_writer.write_u8(VERSION).await?;
    buf_writer.write_u8(TCP_CONNECTION).await?;
    buf_writer.write_u8(0).await?; // reserved
    let port = match destination {
        Endpoint::IPv4 { port, address } => {
            buf_writer.write_u8(IPV4_ADDRESS).await?;
            buf_writer.write_all(&address).await?;
            port
        }
        Endpoint::IPv6 { port, address } => {
            buf_writer.write_u8(IPV6_ADDRESS).await?;
            buf_writer.write_all(&address).await?;
            port
        }
        Endpoint::TORv3 { port, address: _ } => {
            let domainname = destination.to_host();
            let bytes = domainname.as_bytes();
            if bytes.len() <= u8::MAX.into() {
                buf_writer.write_u8(DOMAIN_NAME).await?;
                buf_writer.write_u8(bytes.len() as u8).await?;
                buf_writer.write_all(bytes).await?;
                port
            } else {
                return Err(Error::Destination);
            }
        }
        _ => return Err(Error::Destination),
    };
    buf_writer.write_u16(port).await?;
    buf_writer.flush().await?;

    let version = buf_reader.read_u8().await?;
    if version != VERSION {
        return Err(Error::Version(version));
    }
    let reply = buf_reader.read_u8().await?;
    if reply != REQUEST_GRANTED {
        return Err(Error::NotGranted(reply));
    }
    let reserved = buf_reader.read_u8().await?;
    if reserved != 0 {
        return Err(Error::Reserved(reserved));
    }
    let address_type = buf_reader.read_u8().await?;
    match address_type {
        IPV4_ADDRESS => buf_reader.read_exact(&mut [0u8; 4 + 2]).await?,
        IPV6_ADDRESS => buf_reader.read_exact(&mut [0u8; 16 + 2]).await?,
        DOMAIN_NAME => {
            let mut buf = [0u8; u8::MAX as usize];
            let len = buf_reader.read_u8().await?;
            let slice = &mut buf[..len as usize + 2];
            buf_reader.read_exact(slice).await?
        }
        _ => return Err(Error::Unknown(address_type)),
    };

    Ok((buf_reader, buf_writer))
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
    Io(IoError),
}

impl From<IoError> for Error {
    fn from(error: IoError) -> Self {
        Self::Io(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Endpoint => f.write_str("Unsupported proxy endpoint"),
            Self::Destination => f.write_str("Unsupported destination endpoint"),
            Self::Version(octet) => write!(f, "Unknown socks version {octet}"),
            Self::Authentication(octet) => write!(f, "Authentication not accepted ({octet})"),
            Self::NotGranted(octet) => write!(f, "Access not granted ({octet})"),
            Self::Unknown(octet) => write!(f, "Unknown socks reply ({octet})"),
            Self::Reserved(octet) => write!(f, "Reserved socks reply ({octet})"),
            Self::Io(err) => write!(f, "{err}"),
        }
    }
}

impl core::error::Error for Error {}

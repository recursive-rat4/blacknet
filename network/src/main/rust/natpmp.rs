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

use crate::endpoint::Endpoint;
use natpmp::{Protocol, Response, new_tokio_natpmp};
use thiserror::Error;

//TODO lifetime

pub async fn natpmp_forward(port: u16) -> Result<Endpoint, Error> {
    let mut natpmp = new_tokio_natpmp().await?;

    natpmp.send_public_address_request().await?;
    let response = natpmp.read_response_or_retry().await?;
    let public_address = if let Response::Gateway(gateway) = response {
        *gateway.public_address()
    } else {
        return Err(Error::UnexpectedResponseKind);
    };

    natpmp
        .send_port_mapping_request(Protocol::TCP, port, port, 42)
        .await?;
    let response = natpmp.read_response_or_retry().await?;
    let public_port = if let Response::TCP(mapping) = response {
        mapping.public_port()
    } else {
        return Err(Error::UnexpectedResponseKind);
    };

    Ok(Endpoint::IPv4 {
        port: public_port,
        address: public_address.octets(),
    })
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unexpected response kind")]
    UnexpectedResponseKind,
    #[error("{0}")]
    NATPMP(#[from] natpmp::Error),
}

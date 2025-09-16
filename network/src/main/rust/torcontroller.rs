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
use crate::settings::Settings;
use blacknet_compat::XDGDirectories;
use blacknet_io::file::replace;
use blacknet_log::{Error as LogError, LogManager, error, info, warn};
use core::fmt;
use spdlog::Logger;
use std::io::{Error as IoError, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufStream};
use tokio::net::TcpStream;

// https://spec.torproject.org/control-spec/

const FILE_NAME: &str = "private_key.tor";
const TRANSIENT_KEY: &str = "NEW:ED25519-V3";

pub struct TorController {
    logger: Logger,
    settings: Arc<Settings>,
    data_dir: PathBuf,
    private_key: String,
    endpoint: Endpoint,
}

impl TorController {
    pub fn new(
        dirs: &XDGDirectories,
        log_manager: &LogManager,
        settings: Arc<Settings>,
    ) -> Result<Self> {
        let endpoint = match Endpoint::parse(&settings.tor_control_host, settings.tor_control_port)
        {
            Some(endpoint) => endpoint,
            None => return Err("Can't parse settings.tor_control_host".into()),
        };

        let data_dir = dirs.data().to_owned();
        let private_key = Self::read_private_key_or_transient(&data_dir);

        Ok(TorController {
            logger: log_manager.logger("TorController")?,
            settings,
            data_dir,
            private_key,
            endpoint,
        })
    }

    pub async fn create_session(&mut self) -> Result<TorSession> {
        let mut connection = TorConnection::new(self.logger.clone(), self.endpoint).await?;
        connection.authenticate().await?;
        //XXX port
        let (service_id, new_key) = connection
            .add_onion(&self.private_key, self.settings.port)
            .await?;
        let local_endpoint = Endpoint::parse(&(service_id + ".onion"), self.settings.port)
            .ok_or("Failed to parse Onion Service ID")?;
        if self.private_key.starts_with("NEW:") {
            if !new_key.is_empty() {
                self.save_private_key(new_key);
            } else {
                return Err("Failed to get new private key".into());
            }
        }
        info!(self.logger, "Created session");
        Ok(TorSession {
            logger: self.logger.clone(),
            endpoint: local_endpoint,
            connection,
        })
    }

    fn read_private_key_or_transient(data_dir: &Path) -> String {
        let file_path = data_dir.join(FILE_NAME);
        if let Ok(private_key) = std::fs::read_to_string(file_path)
            && private_key.starts_with("ED25519-V3:")
        {
            return private_key;
        }
        TRANSIENT_KEY.to_string()
    }

    fn save_private_key(&mut self, new_key: String) {
        self.private_key = new_key;
        info!(self.logger, "Saving Tor private key");
        if let Err(err) = replace(&self.data_dir, FILE_NAME, |buffered| {
            buffered.write_all(self.private_key.as_bytes())
        }) {
            error!(self.logger, "Can't write {FILE_NAME}: {err}");
        }
    }
}

pub struct TorSession {
    logger: Logger,
    endpoint: Endpoint,
    connection: TorConnection,
}

impl TorSession {
    pub fn endpoint(&self) -> Endpoint {
        self.endpoint
    }

    pub async fn hung(&mut self) {
        if self.connection.read().await.is_ok() {
            warn!(self.logger, "Unknown Tor reply line");
        }
    }
}

struct TorConnection {
    _logger: Logger,
    stream: BufStream<TcpStream>,
}

impl TorConnection {
    async fn new(logger: Logger, endpoint: Endpoint) -> Result<Self> {
        let endpoint = endpoint.to_rust().ok_or("Not TCP/IP endpoint")?;
        let socket = TcpStream::connect(endpoint).await?;
        let stream = BufStream::new(socket);
        Ok(Self {
            _logger: logger,
            stream,
        })
    }

    async fn authenticate(&mut self) -> Result<()> {
        //TODO cookie, password
        let request = "AUTHENTICATE\r\n";
        let reply_line = self.request(request).await?;
        if reply_line == "250 OK\r\n" {
            return Ok(());
        }
        Err("Unknown Tor reply line".into())
    }

    async fn add_onion(&mut self, private_key: &str, tor_port: u16) -> Result<(String, String)> {
        let request = format!("ADD_ONION {private_key} Port={tor_port}\r\n");
        self.write(&request).await?;
        let mut service_id = String::new();
        let mut new_key = String::new();
        loop {
            let reply_line = self.read().await?;
            if reply_line == "250 OK\r\n" {
                break;
            } else if reply_line.starts_with("250-ServiceID=") && reply_line.ends_with("\r\n") {
                service_id = reply_line[14..reply_line.len() - 14 - 2].to_owned();
            } else if reply_line.starts_with("250-PrivateKey=") && reply_line.ends_with("\r\n") {
                new_key = reply_line[15..reply_line.len() - 15 - 2].to_owned();
            } else if !reply_line.starts_with("250-") {
                return Err("Unknown Tor reply line".into());
            }
        }
        Ok((service_id, new_key))
    }

    async fn request(&mut self, request: &str) -> Result<String> {
        self.write(request).await?;
        self.read().await
    }

    async fn write(&mut self, message: &str) -> Result<()> {
        // debug!(self._logger, "-> {:?}", message);
        Ok(self.stream.write_all(message.as_bytes()).await?)
    }

    async fn read(&mut self) -> Result<String> {
        let mut message = String::new();
        self.stream.read_line(&mut message).await?;
        // debug!(self._logger, "<- {:?}", message);
        Ok(message)
    }
}

type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Message(String),
    Io(IoError),
    Log(LogError),
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Message(msg) => write!(formatter, "{msg}"),
            Error::Io(err) => write!(formatter, "{err}"),
            Error::Log(err) => write!(formatter, "{err}"),
        }
    }
}

impl core::error::Error for Error {}

impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Error::Message(err.to_owned())
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Error::Io(err)
    }
}

impl From<LogError> for Error {
    fn from(err: LogError) -> Self {
        Error::Log(err)
    }
}

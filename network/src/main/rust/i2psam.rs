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
use crate::settings::Settings;
use blacknet_compat::{Mode, XDGDirectories};
use blacknet_crypto::random::{Distribution, FAST_RNG, FastRNG, UniformIntDistribution};
use blacknet_io::file::replace;
use blacknet_log::{Error as LogError, LogManager, Logger, error, info, warn};
use data_encoding::{DecodeError, Encoding};
use data_encoding_macro::new_encoding;
use sha2::{Digest, Sha256};
use std::io::{Error as IoError, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use thiserror::Error;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufStream};
use tokio::net::TcpStream;

// https://geti2p.net/en/docs/api/samv3

const FILE_NAME: &str = "private_key.i2p";
const TRANSIENT_KEY: &str = "TRANSIENT";

const BASE64: Encoding = new_encoding! {
    symbols: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-~",
    padding: '=',
};

pub struct Answer {
    raw: String,
}

impl Answer {
    pub const fn new(raw: String) -> Self {
        Self { raw }
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        let key_pattern = format!(" {}=", key);
        let i = self.raw.find(&key_pattern)?;
        let value_start = i + key_pattern.len();
        if value_start == self.raw.len() {
            return Some("");
        }
        if self.raw[value_start..value_start + 1] == *"\"" {
            let value_end = self.raw[value_start + 1..].find('"');
            if let Some(value_end) = value_end {
                Some(&self.raw[value_start + 1..value_start + 1 + value_end])
            } else {
                None
            }
        } else {
            let value_end = self.raw[value_start..].find(' ');
            if let Some(value_end) = value_end {
                Some(&self.raw[value_start..value_start + value_end])
            } else {
                Some(&self.raw[value_start..self.raw.len() - 1])
            }
        }
    }

    pub fn ok(&self) -> Result<(), String> {
        if let Some(result) = self.get("RESULT") {
            if result.is_empty() {
                Err("Empty RESULT".to_owned())
            } else if result != "OK" {
                if let Some(message) = self.get("MESSAGE") {
                    if message.is_empty() {
                        Err(result.to_owned())
                    } else {
                        Err(format!("{} {}", result, message))
                    }
                } else {
                    Err(result.to_owned())
                }
            } else {
                Ok(())
            }
        } else {
            Err("No RESULT".to_owned())
        }
    }

    pub fn hash(destination: &str) -> Result<[u8; 32], DecodeError> {
        let decoded = BASE64.decode(destination.as_bytes())?;
        let mut hasher = Sha256::new();
        hasher.update(decoded);
        Ok(hasher.finalize().into())
    }
}

pub struct Connection {
    _logger: Logger,
    stream: BufStream<TcpStream>,
}

impl Connection {
    async fn new(logger: Logger, endpoint: Endpoint) -> Result<Self, Error> {
        let endpoint = endpoint.to_rust().ok_or("Not TCP/IP endpoint")?;
        let socket = TcpStream::connect(endpoint).await?;
        let stream = BufStream::new(socket);
        let mut connection = Self {
            _logger: logger,
            stream,
        };
        connection
            .request("HELLO VERSION MIN=3.2 MAX=3.3\n")
            .await?;
        Ok(connection)
    }

    async fn create_session(
        &mut self,
        session_id: &str,
        private_key: &str,
        agent_name: &str,
    ) -> Result<Answer, Error> {
        // i2cp.leaseSetEncType 0 for connectivity with `Node::PROTOCOL_VERSION` <= 15
        let request = format!(
            "SESSION CREATE STYLE=STREAM ID={0} DESTINATION={1} SIGNATURE_TYPE=EdDSA_SHA512_Ed25519 inbound.nickname={2} outbound.nickname={2} i2cp.leaseSetEncType=4,0\n",
            session_id, private_key, agent_name
        );
        self.request(&request).await
    }

    async fn lookup(&mut self, name: &str) -> Result<String, Error> {
        let request = format!("NAMING LOOKUP NAME={}\n", name);
        let answer = self.request(&request).await?;
        match answer.get("VALUE") {
            Some(value) => Ok(value.to_owned()),
            None => Err(Error::Message("lookup answer contains no VALUE".to_owned())),
        }
    }

    async fn write(&mut self, message: &str) -> Result<(), IoError> {
        // debug!(self._logger, "-> {:?}", message);
        self.stream.write_all(message.as_bytes()).await?;
        self.stream.flush().await?;
        Ok(())
    }

    async fn read(&mut self) -> Result<String, IoError> {
        let mut message = String::new();
        self.stream.read_line(&mut message).await?;
        // debug!(self._logger, "<- {:?}", message);
        Ok(message)
    }

    async fn request(&mut self, request: &str) -> Result<Answer, Error> {
        self.write(request).await?;
        let raw = self.read().await?;
        let answer = Answer::new(raw);
        answer.ok()?;
        Ok(answer)
    }
}

pub struct Session {
    logger: Logger,
    id: String,
    local_endpoint: Endpoint,
    sam_endpoint: Endpoint,
    connection: Connection,
}

impl Session {
    pub async fn hung(&mut self) {
        loop {
            match self.connection.read().await {
                Ok(mut message) => {
                    if message.starts_with("PING") {
                        unsafe {
                            message.as_bytes_mut()[1] = b'O';
                        }
                        if let Err(err) = self.connection.write(&message).await {
                            warn!(self.logger, "{}", err);
                            break;
                        }
                    } else if message.starts_with("PONG") {
                        warn!(self.logger, "Unexpected PONG message");
                    } else {
                        let answer = Answer::new(message);
                        if let Err(err) = answer.ok() {
                            warn!(self.logger, "{}", err);
                            break;
                        }
                    }
                }
                Err(err) => {
                    warn!(self.logger, "{}", err);
                    break;
                }
            }
        }
    }

    pub async fn accept(&self) -> Result<Connection, Error> {
        let mut connection = Connection::new(self.logger.clone(), self.sam_endpoint).await?;
        let request = format!("STREAM ACCEPT ID={}\n", self.id);
        connection.request(&request).await?;
        let message = connection.read().await?;
        if message.starts_with("STREAM STATUS") {
            let answer = Answer::new(message);
            answer.ok()?;
        }
        Ok(connection)
    }

    pub const fn endpoint(&self) -> Endpoint {
        self.local_endpoint
    }
}

pub struct SAM {
    logger: Logger,
    settings: Arc<Settings>,
    data_dir: PathBuf,
    private_key: String,
    endpoint: Endpoint,
    agent_name: String,
}

impl SAM {
    pub fn new(
        mode: &Mode,
        dirs: &XDGDirectories,
        log_manager: &LogManager,
        settings: Arc<Settings>,
    ) -> Result<Self, Error> {
        let endpoint = match Endpoint::parse(&settings.i2p_sam_host, settings.i2p_sam_port) {
            Some(endpoint) => endpoint,
            None => return Err("Can't parse settings.i2p_sam_host".into()),
        };

        let data_dir = dirs.data().to_owned();
        let private_key = Self::read_private_key_or_transient(&data_dir);

        Ok(Self {
            logger: log_manager.logger("I2PSAM")?,
            settings,
            data_dir,
            private_key,
            endpoint,
            agent_name: mode.agent_name().to_owned(),
        })
    }

    pub async fn create_session(&mut self) -> Result<Session, Error> {
        let session_id = Self::generate_id();
        let mut connection = Connection::new(self.logger.clone(), self.endpoint).await?;
        let answer = connection
            .create_session(&session_id, &self.private_key, &self.agent_name)
            .await?;
        let destination = connection.lookup("ME").await?;
        let local_endpoint = Endpoint::I2P {
            port: self.settings.port,
            address: Answer::hash(&destination)?,
        };
        if self.private_key == TRANSIENT_KEY {
            self.save_private_key(
                answer
                    .get("DESTINATION")
                    .ok_or(Error::Message("session returned no destination".to_owned()))?
                    .to_owned(),
            );
        }
        let session = Session {
            logger: self.logger.clone(),
            id: session_id,
            local_endpoint,
            sam_endpoint: self.endpoint,
            connection,
        };
        info!(self.logger, "Created session {}", session.id);
        Ok(session)
    }

    fn generate_id() -> String {
        const LEN: usize = 8;
        const ALPHABET: &[char] = &[
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        ];
        let mut id = String::with_capacity(LEN);
        let mut dst = UniformIntDistribution::<usize, FastRNG>::new(..ALPHABET.len());
        FAST_RNG.with_borrow_mut(|rng| {
            for _ in 0..LEN {
                id.push(ALPHABET[dst.sample(rng)]);
            }
        });
        id
    }

    fn read_private_key_or_transient(data_dir: &Path) -> String {
        let file_path = data_dir.join(FILE_NAME);
        if let Ok(private_key) = std::fs::read_to_string(file_path) {
            private_key
        } else {
            TRANSIENT_KEY.to_owned()
        }
    }

    fn save_private_key(&mut self, new_key: String) {
        self.private_key = new_key;
        info!(self.logger, "Saving I2P private key");
        if let Err(err) = replace(&self.data_dir, FILE_NAME, |buffered| {
            buffered.write_all(self.private_key.as_bytes())
        }) {
            error!(self.logger, "Can't write {FILE_NAME}: {err}");
        }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Message(String),
    #[error("{0}")]
    Decode(#[from] DecodeError),
    #[error("{0}")]
    Io(#[from] IoError),
    #[error("{0}")]
    Log(#[from] LogError),
}

impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Error::Message(err.to_owned())
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Error::Message(err)
    }
}

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
use arc_swap::{ArcSwap, ArcSwapOption};
use blacknet_compat::config::Network as Config;
use blacknet_compat::{Mode, XDGDirectories};
use blacknet_crypto::random::{Distribution, FAST_RNG, UniformIntDistribution};
use blacknet_io::file::replace;
use blacknet_log::{Error as LogError, LogManager, Logger, error, info, warn};
use core::fmt;
use core::ops::Deref;
use data_encoding::{DecodeError, Encoding};
use data_encoding_macro::new_encoding;
use sha2::{Digest, Sha256};
use std::borrow::Cow;
use std::io::{Error as IoError, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::TcpStream;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

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
        let hash: [u8; 32] = Sha256::digest(decoded).into();
        Ok(hash)
    }
}

pub struct Connection {
    _logger: Logger,
    buf_reader: BufReader<OwnedReadHalf>,
    buf_writer: BufWriter<OwnedWriteHalf>,
}

impl Connection {
    async fn new(logger: Logger, endpoint: Endpoint) -> Result<Self, Error> {
        let endpoint = endpoint.to_rust().ok_or("Not TCP/IP endpoint")?;
        let socket = TcpStream::connect(endpoint).await?;
        let (tcp_read, tcp_write) = socket.into_split();
        let mut connection = Self {
            _logger: logger,
            buf_reader: BufReader::new(tcp_read),
            buf_writer: BufWriter::new(tcp_write),
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
            None => Err(Error::message("lookup answer contains no VALUE")),
        }
    }

    async fn write(&mut self, message: &str) -> Result<(), IoError> {
        // debug!(self._logger, "-> {:?}", message);
        self.buf_writer.write_all(message.as_bytes()).await?;
        self.buf_writer.flush().await?;
        Ok(())
    }

    async fn read(&mut self) -> Result<String, IoError> {
        let mut message = String::new();
        self.buf_reader.read_line(&mut message).await?;
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
}

pub struct SAM {
    logger: Logger,
    config: Arc<Config>,
    data_dir: PathBuf,
    private_key: ArcSwap<String>,
    endpoint: Endpoint,
    agent_name: String,
    session: ArcSwapOption<(String, Endpoint)>,
}

impl SAM {
    pub fn new(
        mode: &Mode,
        dirs: &XDGDirectories,
        log_manager: &LogManager,
        config: Arc<Config>,
    ) -> Result<Self, Error> {
        let endpoint = match Endpoint::parse(&config.i2p_sam.host, config.i2p_sam.port) {
            Some(endpoint) => endpoint,
            None => return Err("Can't parse config.i2p_sam_host".into()),
        };

        let data_dir = dirs.data().to_owned();
        let private_key = Self::read_private_key_or_transient(&data_dir);

        Ok(Self {
            logger: log_manager.logger("I2PSAM")?,
            config,
            data_dir,
            private_key: ArcSwap::new(Arc::new(private_key)),
            endpoint,
            agent_name: mode.agent_name().to_owned(),
            session: ArcSwapOption::empty(),
        })
    }

    pub async fn create_session(&self) -> Result<(Session, Endpoint), Error> {
        let session_id = Self::generate_id();
        let mut connection = Connection::new(self.logger.clone(), self.endpoint).await?;
        let private_key = self.private_key.load();
        let answer = connection
            .create_session(&session_id, &private_key, &self.agent_name)
            .await?;
        let destination = connection.lookup("ME").await?;
        let local_endpoint = Endpoint::I2P {
            port: self.config.port,
            address: Answer::hash(&destination)?,
        };
        if **private_key == TRANSIENT_KEY {
            self.save_private_key(
                answer
                    .get("DESTINATION")
                    .ok_or(Error::message("session returned no destination"))?
                    .to_owned(),
            );
        }
        let session = Session {
            logger: self.logger.clone(),
            connection,
        };
        info!(self.logger, "Created session {}", session_id);
        self.session
            .store(Some(Arc::new((session_id, local_endpoint))));
        Ok((session, local_endpoint))
    }

    pub fn close_session(&self) {
        self.session.store(None);
    }

    pub async fn accept(
        &self,
    ) -> Result<
        (
            BufReader<OwnedReadHalf>,
            BufWriter<OwnedWriteHalf>,
            Endpoint,
        ),
        Error,
    > {
        let mut connection = Connection::new(self.logger.clone(), self.endpoint).await?;
        let request = format!(
            "STREAM ACCEPT ID={}\n",
            self.session
                .load()
                .deref()
                .as_ref()
                .ok_or(Error::message("No session id"))?
                .0
        );
        connection.request(&request).await?;
        let message = connection.read().await?;
        if message.starts_with("STREAM STATUS") {
            let answer = Answer::new(message.clone());
            answer.ok()?;
        }
        let destination = message
            .split_once(' ')
            .ok_or_else(|| Error::message("Can't parse destination line"))?
            .0;
        let remote_endpoint = Endpoint::I2P {
            port: self.config.port,
            address: Answer::hash(destination)?,
        };
        Ok((
            connection.buf_reader,
            connection.buf_writer,
            remote_endpoint,
        ))
    }

    pub async fn connect(
        &self,
        remote_endpoint: Endpoint,
    ) -> Result<
        (
            BufReader<OwnedReadHalf>,
            BufWriter<OwnedWriteHalf>,
            Endpoint,
        ),
        Error,
    > {
        let session = self.session.load();
        let Some(session) = session.deref() else {
            return Err(Error::message("No session"));
        };
        let mut connection = Connection::new(self.logger.clone(), self.endpoint).await?;
        let destination = connection.lookup(&remote_endpoint.to_host()).await?;
        let request = format!(
            "STREAM CONNECT ID={} DESTINATION={}\n",
            session.0, destination
        );
        let answer = connection.request(&request).await?;
        answer.ok()?;
        Ok((connection.buf_reader, connection.buf_writer, session.1))
    }

    fn generate_id() -> String {
        const LEN: usize = 8;
        const ALPHABET: &[char] = &[
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        ];
        let mut id = String::with_capacity(LEN);
        let mut dst = UniformIntDistribution::<usize>::new(..ALPHABET.len());
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

    fn save_private_key(&self, new_key: String) {
        let new_key = Arc::new(new_key);
        self.private_key.store(new_key.clone());
        info!(self.logger, "Saving I2P private key");
        if let Err(err) = replace(&self.data_dir, FILE_NAME, |buffered| {
            buffered.write_all(new_key.as_bytes())
        }) {
            error!(self.logger, "Can't write {FILE_NAME}: {err}");
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Message(Cow<'static, str>),
    Decode(DecodeError),
    Io(IoError),
    Log(LogError),
}

impl Error {
    pub fn message<T>(msg: T) -> Self
    where
        Cow<'static, str>: From<T>,
    {
        Error::Message(msg.into())
    }
}

impl From<&'static str> for Error {
    fn from(err: &'static str) -> Self {
        Error::message(err)
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Error::message(err)
    }
}

impl From<DecodeError> for Error {
    fn from(err: DecodeError) -> Self {
        Error::Decode(err)
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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Message(msg) => f.write_str(msg),
            Self::Decode(err) => write!(f, "{err}"),
            Self::Io(err) => write!(f, "{err}"),
            Self::Log(err) => write!(f, "{err}"),
        }
    }
}

impl core::error::Error for Error {}

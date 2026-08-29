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

use crate::{
    endpoint::{Endpoint, ipv4_any, ipv6_any},
    i2psam::SAM,
    natpmp::natpmp_forward,
    peertable::PeerTable,
    socks5::socks5,
    torcontroller::TorController,
};
use blacknet_compat::{
    config::Network as Config,
    {Mode, XDGDirectories},
};
use blacknet_log::{LogManager, Logger, info, warn};
use core::{
    cmp::{max, min},
    error::Error,
    net::SocketAddr,
    ops::ControlFlow,
};
use std::{
    collections::HashSet,
    sync::{Arc, RwLock},
};
use tokio::{
    io::{BufReader, BufWriter},
    net::{
        TcpListener, TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
    runtime::{Handle, Runtime},
    sync::{Mutex, mpsc},
    time::{Duration, sleep},
};

pub type Notification = (
    BufReader<OwnedReadHalf>,
    BufWriter<OwnedWriteHalf>,
    Endpoint,
    Endpoint,
);
pub type Notifier = mpsc::Receiver<Notification>;
pub type Subscriber = mpsc::Sender<Notification>;

pub struct Router {
    logger: Logger,
    runtime: Handle,
    config: Arc<Config>,
    listens: RwLock<HashSet<Endpoint>>,
    peer_table: Arc<PeerTable>,
    socks_proxy: Option<Endpoint>,
    tor_proxy: Option<Endpoint>,
    i2p_sam: SAM,
    tor_controller: Mutex<TorController>,
    subscriber: Subscriber,
}

impl Router {
    pub fn new(
        mode: &Mode,
        dirs: &XDGDirectories,
        log_manager: &LogManager,
        runtime: &Runtime,
        config: &Arc<Config>,
        peer_table: Arc<PeerTable>,
    ) -> Result<(Arc<Self>, Notifier), Box<dyn Error>> {
        // mpsc bounded channel requires buffer > 0
        let incoming_connections = max(config.incoming_connections, 1);

        let (subscriber, notifier) = mpsc::channel(incoming_connections as usize);

        let router = Arc::new(Self {
            logger: log_manager.logger("Router")?,
            runtime: runtime.handle().clone(),
            config: config.clone(),
            listens: RwLock::new(HashSet::new()),
            peer_table,
            socks_proxy: config
                .proxy
                .as_ref()
                .and_then(|proxy| Endpoint::parse(&proxy.host, proxy.port)),
            tor_proxy: Endpoint::parse(&config.tor_proxy.host, config.tor_proxy.port),
            i2p_sam: SAM::new(mode, dirs, log_manager, config.clone())?,
            tor_controller: Mutex::new(TorController::new(dirs, log_manager, config.clone())?),
            subscriber,
        });

        if config.ipv6 || config.ipv4 {
            runtime.spawn(router.clone().listen_ip());
            if config.natpmp {
                runtime.spawn(router.clone().forward_natpmp());
            }
        }
        if config.tor {
            runtime.spawn(router.clone().listen_tor());
        }
        if config.i2p {
            runtime.spawn(router.clone().listen_i2p());
        }

        Ok((router, notifier))
    }

    pub async fn connect(
        &self,
        endpoint: Endpoint,
    ) -> Option<(
        BufReader<OwnedReadHalf>,
        BufWriter<OwnedWriteHalf>,
        Endpoint,
    )> {
        match endpoint {
            Endpoint::IPv4 {
                port: _,
                address: _,
            } => {
                if !self.config.ipv4 {
                    return None;
                }
                match self.socks_proxy {
                    Some(proxy) => {
                        let (buf_reader, buf_writer) = socks5(proxy, endpoint).await.ok()?;
                        Some((buf_reader, buf_writer, proxy))
                    }
                    None => {
                        let endpoint = endpoint.to_rust()?;
                        let socket = TcpStream::connect(endpoint).await.ok()?;
                        let local_endpoint = Endpoint::from(socket.local_addr().ok()?);
                        let (tcp_read, tcp_write) = socket.into_split();
                        let (buf_reader, buf_writer) =
                            (BufReader::new(tcp_read), BufWriter::new(tcp_write));
                        if !local_endpoint.is_local() {
                            let mut local_endpoint = local_endpoint;
                            local_endpoint.set_port(self.config.port);
                            self.add_listener(local_endpoint);
                        }
                        Some((buf_reader, buf_writer, local_endpoint))
                    }
                }
            }
            Endpoint::IPv6 {
                port: _,
                address: _,
            } => {
                if !self.config.ipv6 {
                    return None;
                }
                match self.socks_proxy {
                    Some(proxy) => {
                        let (buf_reader, buf_writer) = socks5(proxy, endpoint).await.ok()?;
                        Some((buf_reader, buf_writer, proxy))
                    }
                    None => {
                        let endpoint = endpoint.to_rust()?;
                        let socket = TcpStream::connect(endpoint).await.ok()?;
                        let local_endpoint = Endpoint::from(socket.local_addr().ok()?);
                        let (tcp_read, tcp_write) = socket.into_split();
                        let (buf_reader, buf_writer) =
                            (BufReader::new(tcp_read), BufWriter::new(tcp_write));
                        if !local_endpoint.is_local() {
                            let mut local_endpoint = local_endpoint;
                            local_endpoint.set_port(self.config.port);
                            self.add_listener(local_endpoint);
                        }
                        Some((buf_reader, buf_writer, local_endpoint))
                    }
                }
            }
            Endpoint::TORv3 {
                port: _,
                address: _,
            } => {
                if !self.config.tor {
                    return None;
                }
                match self.tor_proxy {
                    Some(proxy) => {
                        let (buf_reader, buf_writer) = socks5(proxy, endpoint).await.ok()?;
                        Some((buf_reader, buf_writer, proxy))
                    }
                    None => None,
                }
            }
            Endpoint::I2P {
                port: _,
                address: _,
            } => {
                if !self.config.i2p {
                    return None;
                }
                self.i2p_sam.connect(endpoint).await.ok()
            }
            Endpoint::TORv2 {
                port: _,
                address: _,
            } => {
                // obsolete
                None
            }
        }
    }

    async fn listen_ip(self: Arc<Self>) {
        let mut timeout = Self::INIT_TIMEOUT;
        let endpoint = if self.config.ipv6 {
            ipv6_any(self.config.port)
        } else if self.config.ipv4 {
            ipv4_any(self.config.port)
        } else {
            panic!("Both IPv4 and IPv6 are disabled");
        };
        loop {
            match TcpListener::bind(endpoint.to_rust().expect("TCP/IP")).await {
                Ok(listener) => {
                    timeout = Self::INIT_TIMEOUT;
                    self.add_listener(endpoint);
                    loop {
                        match listener.accept().await {
                            Ok((socket, addr)) => {
                                if self.accept_ip(socket, addr).await.is_break() {
                                    break;
                                }
                            }
                            Err(msg) => {
                                warn!(self.logger, "{msg}");
                                break;
                            }
                        }
                    }
                    self.remove_listener(endpoint);
                }
                Err(msg) => {
                    warn!(self.logger, "{msg}");
                }
            }

            sleep(timeout).await;
            timeout = min(timeout * 2, Self::MAX_TIMEOUT);
        }
    }

    async fn accept_ip(&self, tcp_stream: TcpStream, addr: SocketAddr) -> ControlFlow<()> {
        let remote_endpoint: Endpoint = addr.into();
        let local_endpoint: Endpoint = match tcp_stream.local_addr() {
            Ok(addr) => addr.into(),
            Err(err) => {
                warn!(self.logger, "local_addr: {err}");
                return ControlFlow::Continue(());
            }
        };
        if !local_endpoint.is_local() {
            self.add_listener(local_endpoint)
        }
        let (tcp_read, tcp_write) = tcp_stream.into_split();
        let (buf_reader, buf_writer) = (BufReader::new(tcp_read), BufWriter::new(tcp_write));
        if self
            .subscriber
            .send((buf_reader, buf_writer, remote_endpoint, local_endpoint))
            .await
            .is_ok()
        {
            ControlFlow::Continue(())
        } else {
            ControlFlow::Break(())
        }
    }

    async fn accept_i2p(self: Arc<Self>, local_endpoint: Endpoint) {
        loop {
            match self.i2p_sam.accept().await {
                Ok((buf_reader, buf_writer, remote_endpoint)) => {
                    if self
                        .subscriber
                        .send((buf_reader, buf_writer, remote_endpoint, local_endpoint))
                        .await
                        .is_ok()
                    {
                        continue;
                    } else {
                        break;
                    }
                }
                Err(err) => {
                    warn!(self.logger, "accept_i2p: {err}");
                    break;
                }
            }
        }
    }

    async fn listen_tor(self: Arc<Self>) {
        let mut timeout = Self::INIT_TIMEOUT;
        let mut tor_controller = self.tor_controller.lock().await;
        loop {
            match tor_controller.create_session().await {
                Ok(mut session) => {
                    timeout = Self::INIT_TIMEOUT;
                    self.add_listener(session.endpoint());
                    session.hung().await;
                    info!(self.logger, "Closing TOR session");
                    self.remove_listener(session.endpoint());
                }
                Err(msg) => {
                    warn!(self.logger, "{msg}");
                }
            }

            sleep(timeout).await;
            timeout = min(timeout * 2, Self::MAX_TIMEOUT);
        }
    }

    async fn listen_i2p(self: Arc<Self>) {
        let mut timeout = Self::INIT_TIMEOUT;
        loop {
            match self.i2p_sam.create_session().await {
                Ok((mut session, local_endpoint)) => {
                    timeout = Self::INIT_TIMEOUT;
                    self.add_listener(local_endpoint);
                    self.runtime.spawn(self.clone().accept_i2p(local_endpoint));
                    session.hung().await;
                    info!(self.logger, "Closing I2P session");
                    self.remove_listener(local_endpoint);
                    self.i2p_sam.close_session();
                }
                Err(msg) => {
                    warn!(self.logger, "{msg}");
                }
            }

            sleep(timeout).await;
            timeout = min(timeout * 2, Self::MAX_TIMEOUT);
        }
    }

    async fn forward_natpmp(self: Arc<Self>) {
        match natpmp_forward(self.config.port).await {
            Ok(endpoint) => {
                self.add_listener(endpoint);
            }
            Err(msg) => {
                info!(self.logger, "NAT-PMP: {msg}");
            }
        }
    }

    fn add_listener(&self, endpoint: Endpoint) {
        info!(
            self.logger,
            "Listening on {}",
            endpoint.to_log(self.config.log_endpoint)
        );
        let inserted = {
            let mut listens = self.listens.write().unwrap();
            listens.insert(endpoint)
        };
        if inserted {
            self.peer_table.contacted(endpoint)
        }
    }
    fn remove_listener(&self, endpoint: Endpoint) {
        info!(
            self.logger,
            "Lost binding to {}",
            endpoint.to_log(self.config.log_endpoint)
        );
        let removed = {
            let mut listens = self.listens.write().unwrap();
            listens.remove(&endpoint)
        };
        if removed {
            self.peer_table.discontacted(endpoint)
        }
    }
    pub const fn listening(&self) -> &RwLock<HashSet<Endpoint>> {
        &self.listens
    }

    const INIT_TIMEOUT: Duration = Duration::from_secs(60);
    const MAX_TIMEOUT: Duration = Duration::from_secs(2 * 60 * 60);
}

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

use crate::endpoint::{Endpoint, ipv4_any, ipv6_any};
use crate::i2psam::SAM;
use crate::natpmp::natpmp_forward;
use crate::node::Node;
use crate::peertable::PeerTable;
use crate::torcontroller::TorController;
use blacknet_compat::config::Network as Config;
use blacknet_compat::{Mode, XDGDirectories};
use blacknet_log::{LogManager, Logger, info, warn};
use core::cmp::min;
use core::error::Error;
use core::net::SocketAddr;
use core::ops::ControlFlow;
use std::collections::HashSet;
use std::sync::{Arc, OnceLock, RwLock, Weak};
use tokio::net::{TcpListener, TcpStream};
use tokio::runtime::{Handle, Runtime};
use tokio::sync::Mutex;
use tokio::time::{Duration, sleep};

pub struct Router {
    logger: Logger,
    runtime: Handle,
    config: Arc<Config>,
    listens: RwLock<HashSet<Endpoint>>,
    peer_table: Arc<PeerTable>,
    i2p_sam: SAM,
    tor_controller: Mutex<TorController>,
    node: OnceLock<Weak<Node>>,
}

impl Router {
    pub fn new(
        mode: &Mode,
        dirs: &XDGDirectories,
        log_manager: &LogManager,
        runtime: &Runtime,
        config: &Arc<Config>,
        peer_table: Arc<PeerTable>,
    ) -> Result<Arc<Self>, Box<dyn Error>> {
        let router = Arc::new(Self {
            logger: log_manager.logger("Router")?,
            runtime: runtime.handle().clone(),
            config: config.clone(),
            listens: RwLock::new(HashSet::new()),
            peer_table,
            i2p_sam: SAM::new(mode, dirs, log_manager, config.clone())?,
            tor_controller: Mutex::new(TorController::new(dirs, log_manager, config.clone())?),
            node: OnceLock::new(),
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

        Ok(router)
    }

    pub fn set_node(&self, node: Weak<Node>) {
        self.node.set(node).expect("Node constructor")
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
                                if self.accept_ip(socket, addr).is_break() {
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

    fn accept_ip(&self, tcp_stream: TcpStream, addr: SocketAddr) -> ControlFlow<()> {
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
        match self.node.get().expect("Router initialized").upgrade() {
            Some(node) => node.accept_ip(tcp_stream, remote_endpoint, local_endpoint),
            None => return ControlFlow::Break(()),
        }
        ControlFlow::Continue(())
    }

    async fn accept_i2p(self: Arc<Self>, session_id: String) {
        loop {
            #[expect(unused_variables)]
            match self.i2p_sam.accept(&session_id).await {
                Ok((stream, remote_endpoint)) => {
                    todo!();
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
                Ok(mut session) => {
                    timeout = Self::INIT_TIMEOUT;
                    self.add_listener(session.endpoint());
                    self.runtime.spawn(self.clone().accept_i2p(session.id()));
                    session.hung().await;
                    info!(self.logger, "Closing I2P session");
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

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

use crate::endpoint::{Endpoint, ipv4_any, ipv6_any};
use crate::i2psam::SAM;
use crate::natpmp::natpmp_forward;
use crate::peertable::PeerTable;
use crate::settings::Settings;
use crate::torcontroller::TorController;
use blacknet_compat::{Mode, XDGDirectories};
use blacknet_log::{LogManager, info, warn};
use spdlog::Logger;
use std::cmp::min;
use std::collections::HashSet;
use std::error::Error;
use std::sync::{Arc, RwLock};
use tokio::net::TcpListener;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;
use tokio::time::{Duration, sleep};

pub struct Router {
    logger: Logger,
    settings: Arc<Settings>,
    listens: RwLock<HashSet<Endpoint>>,
    peer_table: Arc<PeerTable>,
    i2p_sam: Mutex<SAM>,
    tor_controller: Mutex<TorController>,
}

impl Router {
    pub fn new(
        mode: &Mode,
        dirs: &XDGDirectories,
        log_manager: &LogManager,
        runtime: &Runtime,
        settings: Arc<Settings>,
        peer_table: Arc<PeerTable>,
    ) -> Result<Arc<Self>, Box<dyn Error>> {
        let router = Arc::new(Self {
            logger: log_manager.logger("Router")?,
            settings: settings.clone(),
            listens: RwLock::new(HashSet::new()),
            peer_table,
            i2p_sam: Mutex::new(SAM::new(mode, dirs, log_manager, settings.clone())?),
            tor_controller: Mutex::new(TorController::new(dirs, log_manager, settings.clone())?),
        });

        if settings.ipv6 || settings.ipv4 {
            runtime.spawn(router.clone().listen_ip());
            if settings.natpmp {
                runtime.spawn(router.clone().forward_natpmp());
            }
        }
        if settings.tor {
            runtime.spawn(router.clone().listen_tor());
        }
        if settings.i2p {
            runtime.spawn(router.clone().listen_i2p());
        }

        Ok(router)
    }

    async fn listen_ip(self: Arc<Self>) {
        let mut timeout = Self::INIT_TIMEOUT;
        let endpoint = if self.settings.ipv6 {
            ipv6_any(self.settings.port)
        } else if self.settings.ipv4 {
            ipv4_any(self.settings.port)
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
                            Ok((_socket, _addr)) => todo!(),
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
        let mut i2p_sam = self.i2p_sam.lock().await;
        loop {
            match i2p_sam.create_session().await {
                Ok(mut session) => {
                    timeout = Self::INIT_TIMEOUT;
                    self.add_listener(session.endpoint());
                    session.hung().await;
                    //TODO accept
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
        match natpmp_forward(self.settings.port).await {
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
            endpoint.to_log(self.settings.log_endpoint)
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
            endpoint.to_log(self.settings.log_endpoint)
        );
        let removed = {
            let mut listens = self.listens.write().unwrap();
            listens.remove(&endpoint)
        };
        if removed {
            self.peer_table.discontacted(endpoint)
        }
    }

    const INIT_TIMEOUT: Duration = Duration::from_secs(60);
    const MAX_TIMEOUT: Duration = Duration::from_secs(2 * 60 * 60);
}

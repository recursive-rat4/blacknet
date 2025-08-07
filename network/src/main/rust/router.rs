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
use crate::peertable::PeerTable;
use crate::settings::Settings;
use blacknet_log::logmanager::LogManager;
use blacknet_log::{info, warn};
use spdlog::Logger;
use std::cmp::min;
use std::collections::HashSet;
use std::error::Error;
use std::sync::{Arc, RwLock};
use tokio::net::TcpListener;
use tokio::runtime::Runtime;
use tokio::time::{Duration, sleep};

pub struct Router {
    logger: Logger,
    settings: Arc<Settings>,
    listens: RwLock<HashSet<Endpoint>>,
    peer_table: Arc<PeerTable>,
}

impl Router {
    pub fn new(
        log_manager: &LogManager,
        runtime: &Runtime,
        settings: Arc<Settings>,
        peer_table: Arc<PeerTable>,
    ) -> Result<Arc<Self>, Box<dyn Error>> {
        let router = Arc::new(Self {
            logger: log_manager.logger("Router")?,
            settings,
            listens: RwLock::new(HashSet::new()),
            peer_table,
        });

        if router.settings.ipv6 || router.settings.ipv4 {
            runtime.spawn(router.clone().listen_ip());
        }
        if router.settings.tor {
            runtime.spawn(router.clone().listen_tor());
        }
        if router.settings.i2p {
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
        todo!();
    }

    async fn listen_i2p(self: Arc<Self>) {
        todo!();
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

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

use crate::endpoint::{ipv4_any, ipv6_any};
use blacknet_log::logmanager::LogManager;
use blacknet_log::{info, warn};
use spdlog::Logger;
use std::cmp::min;
use std::error::Error;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::runtime::Runtime;
use tokio::time::{Duration, sleep};

pub struct Settings {
    port: u16,
    ipv4: bool,
    ipv6: bool,
    tor: bool,
    i2p: bool,
}

pub struct Router {
    logger: Logger,
    settings: Settings,
}

impl Router {
    pub fn new(log_manager: &LogManager, runtime: &Runtime) -> Result<Arc<Self>, Box<dyn Error>> {
        let router = Arc::new(Self {
            logger: log_manager.logger("Router")?,
            settings: Settings {
                port: 28453,
                ipv4: true,
                ipv6: true,
                tor: true,
                i2p: true,
            },
        });
        if router.settings.ipv6 || router.settings.ipv4 {
            runtime.spawn(router.clone().listen_ip());
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
                }
                Err(msg) => {
                    warn!(self.logger, "{msg}");
                }
            }

            sleep(timeout).await;
            timeout = min(timeout * 2, Self::MAX_TIMEOUT);
        }
    }

    const INIT_TIMEOUT: Duration = Duration::from_secs(60);
    const MAX_TIMEOUT: Duration = Duration::from_secs(2 * 60 * 60);
}

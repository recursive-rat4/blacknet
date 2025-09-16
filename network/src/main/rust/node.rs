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

use crate::connection::Connection;
use crate::peertable::PeerTable;
use crate::router::Router;
use crate::settings::Settings;
use blacknet_compat::{Mode, XDGDirectories, getuid, uname};
use blacknet_log::{LogManager, info, warn};
use core::num::NonZero;
use std::error::Error;
use std::sync::atomic::AtomicU64;
use std::sync::{Arc, RwLock};
use tokio::runtime::Runtime;
use tokio::time::{Duration, sleep};

pub struct Node {
    settings: Arc<Settings>,
    next_peer_id: AtomicU64,
    connections: RwLock<Vec<Connection>>,
    peer_table: Arc<PeerTable>,
    router: Arc<Router>,
}

impl Node {
    pub fn new(
        mode: &Mode,
        dirs: &XDGDirectories,
        log_manager: &LogManager,
        runtime: &Runtime,
    ) -> Result<Arc<Self>, Box<dyn Error>> {
        let (os_name, os_version, os_machine) = uname();
        let (agent_name, agent_version) = (mode.agent_name(), env!("CARGO_PKG_VERSION"));
        let cpu_cores = std::thread::available_parallelism()
            .map(NonZero::get)
            .unwrap_or(0);

        let logger = log_manager.logger("Node")?;
        info!(logger, "Starting up {agent_name} node {agent_version}");
        info!(logger, "CPU: {cpu_cores} cores {os_machine}");
        info!(logger, "OS: {os_name} version {os_version}");
        info!(logger, "Using config directory {}", dirs.config().display());
        info!(logger, "Using data directory {}", dirs.data().display());
        info!(logger, "Using state directory {}", dirs.state().display());

        if getuid() == 0 {
            warn!(logger, "Running as root");
        }

        let settings = Arc::new(Settings::default(mode));
        let peer_table = PeerTable::new(mode, dirs, log_manager, settings.clone())?;
        let node = Arc::new(Self {
            settings: settings.clone(),
            next_peer_id: AtomicU64::new(1),
            connections: RwLock::new(Vec::new()),
            peer_table: peer_table.clone(),
            router: Router::new(mode, dirs, log_manager, runtime, settings, peer_table)?,
        });

        runtime.spawn(node.clone().rotator());

        Ok(node)
    }

    pub fn is_online(&self) -> bool {
        let connections = self.connections.read().unwrap();
        connections.iter().any(Connection::is_established)
    }

    async fn rotator(self: Arc<Self>) {
        loop {
            sleep(Duration::from_secs(60 * 60)).await;

            // Await while node gets online
            if !self.is_online() {
                continue;
            }

            self.peer_table.clone().rotate().await;
        }
    }
}

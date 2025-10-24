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

use crate::connection::{Connection, State};
use crate::endpoint::Endpoint;
use crate::peertable::PeerTable;
use crate::router::Router;
use crate::settings::Settings;
use blacknet_compat::{Mode, XDGDirectories, getuid, uname};
use blacknet_io::Write;
use blacknet_io::file::replace;
use blacknet_log::{LogManager, Logger, error, info, warn};
use blacknet_serialization::format::to_write;
use core::num::NonZero;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::PathBuf;
use std::sync::atomic::AtomicU64;
use std::sync::{Arc, RwLock};
use tokio::runtime::Runtime;
use tokio::time::{Duration, sleep};

pub struct Node {
    logger: Logger,
    settings: Arc<Settings>,
    state_dir: PathBuf,
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
            logger,
            settings: settings.clone(),
            state_dir: dirs.state().to_owned(),
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

impl Drop for Node {
    fn drop(&mut self) {
        let mut connections = self.connections.write().unwrap();
        info!(self.logger, "Closing {} p2p connections", connections.len());
        let mut peers = Vec::with_capacity(connections.len());
        for connection in connections.iter() {
            // probers ain't interesting
            if connection.state() == State::OutgoingConnected {
                peers.push(connection.remote_endpoint());
            }
            connection.close();
        }
        connections.clear();
        info!(self.logger, "Saving node state");
        let persistent = Persistent { peers };
        if let Err(err) = replace(&self.state_dir, DATA_FILENAME, |buffered| {
            let version = DATA_VERSION.to_be_bytes();
            buffered.write_all(&version)?;
            to_write(&persistent, buffered)
        }) {
            error!(self.logger, "Can't write {DATA_FILENAME}: {err}");
        }
    }
}

const DATA_VERSION: u32 = 1;
const DATA_FILENAME: &str = "node.dat";

#[derive(Deserialize, Serialize)]
struct Persistent {
    peers: Vec<Endpoint>,
}

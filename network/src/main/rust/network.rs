/*
 * Copyright (c) 2018-2026 Pavel Vasin
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

use crate::{node::Node, staker::Staker, wallet::WalletDB};
use blacknet_compat::{
    config::Network as Config,
    {Mode, XDGDirectories, getuid, uname},
};
use blacknet_log::{LogManager, info, warn};
use core::error::Error as StdError;
use std::sync::Arc;
use tokio::runtime::Runtime;

pub struct Network {
    node: Arc<Node>,
    wallet_db: Arc<WalletDB>,
    staker: Arc<Staker>,
}

impl Network {
    pub fn new(
        mode: Mode,
        dirs: &XDGDirectories,
        log_manager: &LogManager,
        runtime: &Runtime,
        config: &Arc<Config>,
    ) -> Result<Arc<Self>, Box<dyn StdError>> {
        let (os_name, os_version, os_machine) = uname();
        let (agent_name, agent_version) = (mode.agent_name(), env!("CARGO_PKG_VERSION"));

        let logger = log_manager.logger("Network")?;
        info!(logger, "Starting up {agent_name} node {agent_version}");
        match std::thread::available_parallelism() {
            Ok(cpu_cores) => info!(logger, "CPU: {cpu_cores} cores {os_machine}"),
            Err(err) => warn!(logger, "CPU: {os_machine} ({err})"),
        }
        info!(logger, "OS: {os_name} version {os_version}");
        info!(logger, "Using config directory {}", dirs.config().display());
        info!(logger, "Using data directory {}", dirs.data().display());
        info!(logger, "Using state directory {}", dirs.state().display());

        if getuid() == 0 {
            warn!(logger, "Running as root");
        }

        let (node, coin_notifier) = Node::new(mode, dirs, log_manager, runtime, config)?;
        let wallet_db = WalletDB::new(
            node.mode(),
            dirs,
            log_manager,
            runtime,
            coin_notifier,
            node.tx_pool(),
        )?;
        let staker = Staker::new(log_manager, runtime, node.clone(), &wallet_db)?;

        let network = Arc::new(Self {
            node,
            wallet_db,
            staker,
        });

        Ok(network)
    }

    pub fn mode(&self) -> &Mode {
        self.node.mode()
    }

    pub const fn node(&self) -> &Arc<Node> {
        &self.node
    }

    pub const fn wallet_db(&self) -> &Arc<WalletDB> {
        &self.wallet_db
    }

    pub const fn staker(&self) -> &Arc<Staker> {
        &self.staker
    }

    pub fn warnings(&self) -> Vec<String> {
        let mut warnings = Vec::<String>::new();
        self.node.warnings(&mut warnings);
        warnings
    }
}

impl Drop for Network {
    fn drop(&mut self) {
        self.node.dispose();
    }
}

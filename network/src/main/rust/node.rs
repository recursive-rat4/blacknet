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

use blacknet_compat::getuid::getuid;
use blacknet_compat::mode::Mode;
use blacknet_compat::uname::uname;
use blacknet_compat::xdgdirectories::XDGDirectories;
use blacknet_log::logmanager::LogManager;
use blacknet_log::{info, warn};
use core::num::NonZero;
use std::error::Error;

pub struct Node {}

impl Node {
    pub fn new(
        mode: Mode,
        dirs: XDGDirectories,
        log_manager: LogManager,
    ) -> Result<Self, Box<dyn Error>> {
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

        Ok(Self {})
    }
}

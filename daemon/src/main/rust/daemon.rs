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

use blacknet_compat::mode::mode;
use blacknet_compat::xdgdirectories::XDGDirectories;
use blacknet_log::logmanager::{LogManager, Strategy};
use blacknet_network::node::Node;
use std::env::args;
use std::process::ExitCode;

fn main() -> ExitCode {
    if args().nth(1).is_some_and(|arg| arg == "--version") {
        println!("Blacknet Daemon {}", env!("CARGO_PKG_VERSION"));
        return ExitCode::SUCCESS;
    }
    let mode = match mode() {
        Ok(mode) => mode,
        Err(msg) => {
            eprintln!("{msg}");
            return ExitCode::FAILURE;
        }
    };
    let dirs = match XDGDirectories::new(mode.subdirectory()) {
        Ok(dirs) => dirs,
        Err(msg) => {
            eprintln!("{msg}");
            return ExitCode::FAILURE;
        }
    };
    let log_manager = match LogManager::new(Strategy::Daemon, dirs.state()) {
        Ok(log_manager) => log_manager,
        Err(msg) => {
            eprintln!("{msg}");
            return ExitCode::FAILURE;
        }
    };
    match Node::new(mode, dirs, log_manager) {
        Ok(..) => ExitCode::SUCCESS,
        Err(msg) => {
            eprintln!("{msg}");
            ExitCode::FAILURE
        }
    }
}

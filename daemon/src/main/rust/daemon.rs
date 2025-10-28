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

use blacknet_compat::{XDGDirectories, mode};
use blacknet_json_rpc::{Settings as RPCSettings, rpc_server};
use blacknet_log::{LogManager, Strategy};
use blacknet_network::node::Node;
use std::env::args;
use std::error::Error;
use std::process::ExitCode;
use tokio::runtime::Runtime;
use tokio::signal::ctrl_c;
use tokio::sync::mpsc::unbounded_channel;

fn daemon() -> Result<(), Box<dyn Error>> {
    if args().nth(1).is_some_and(|arg| arg == "--version") {
        println!("Blacknet Daemon {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    let mode = mode()?;
    let dirs = XDGDirectories::new(mode.subdirectory())?;
    let log_manager = LogManager::new(Strategy::Daemon, dirs.state())?;
    let runtime = Runtime::new()?;
    let (shutdown_send, mut shutdown_recv) = unbounded_channel::<()>();
    let _node = Node::new(&mode, &dirs, &log_manager, &runtime)?;
    let rpc_settings = RPCSettings::default(&mode);
    if rpc_settings.enabled {
        runtime.spawn(async move {
            rpc_server(rpc_settings, &log_manager, shutdown_send).await;
        });
    }
    runtime.block_on(async move {
        tokio::select! {
            _ = ctrl_c() => {},
            _ = shutdown_recv.recv() => {},
        }
    });
    Ok(())
}

fn main() -> ExitCode {
    match daemon() {
        Ok(..) => ExitCode::SUCCESS,
        Err(msg) => {
            eprintln!("{msg}");
            ExitCode::FAILURE
        }
    }
}

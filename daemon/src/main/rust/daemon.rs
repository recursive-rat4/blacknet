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

use blacknet_compat::{XDGDirectories, mode};
use blacknet_json_rpc::{Settings as RPCSettings, rpc_server};
use blacknet_log::{LogManager, Strategy};
use blacknet_network::node::Node;
use std::env::args;
use std::error::Error;
use std::process::ExitCode;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::select;
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
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .name("blacknet-runtime")
        .thread_name_fn(|| {
            static THREAD_ID: AtomicU64 = AtomicU64::new(1);
            let id = THREAD_ID.fetch_add(1, Ordering::Relaxed);
            format!("tokio-thread-{}", id)
        })
        .build()?;
    let (shutdown_send, mut shutdown_recv) = unbounded_channel::<()>();
    let rpc_settings = RPCSettings::default(&mode);
    let node = Node::new(mode, &dirs, &log_manager, &runtime)?;
    if rpc_settings.enabled {
        let node = node.clone();
        runtime.spawn(async move {
            rpc_server(rpc_settings, &log_manager, node, shutdown_send).await;
        });
    }
    runtime.block_on(async move {
        select! {
            _ = ctrl_c() => {},
            _ = shutdown_recv.recv() => {},
        }
    });
    node.dispose();
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

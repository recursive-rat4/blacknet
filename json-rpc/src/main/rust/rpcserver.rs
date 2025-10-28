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

use crate::Settings;
use axum::{Router, routing::get};
use blacknet_log::{LogManager, error};
use tokio::net::TcpListener;
use tokio::sync::mpsc::UnboundedSender;

pub async fn rpc_server(
    settings: Settings,
    log_manager: &LogManager,
    shutdown_send: UnboundedSender<()>,
) {
    let router = Router::new().route(
        "/api/shutdown",
        get(|| async move { shutdown_send.send(()).unwrap() }),
    );
    let addr = format!("{}:{}", settings.host, settings.port);
    match TcpListener::bind(&addr).await {
        Ok(listener) => axum::serve(listener, router).await.unwrap(),
        Err(err) => {
            let logger = log_manager.logger("RPCServer").unwrap();
            error!(logger, "Can't bind to {addr} because {err}");
            panic!();
        }
    };
}

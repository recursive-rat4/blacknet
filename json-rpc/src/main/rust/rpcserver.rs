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

use crate::v2;
use axum::{Router, extract::ws::Message, routing::get};
use blacknet_compat::config::RPC as Config;
use blacknet_log::{LogManager, error, info};
use blacknet_network::node::Node;
use core::num::NonZero;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicU64, Ordering},
};
use tokio::net::TcpListener;
use tokio::sync::mpsc;

pub struct RPCServer {
    next_subscriber_id: AtomicU64,
    block_notify: Mutex<Vec<Subscriber>>,
    txpool_notify: Mutex<Vec<Subscriber>>,
    wallet_notify: Mutex<Vec<Subscriber>>,
}

impl RPCServer {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            next_subscriber_id: AtomicU64::new(1),
            block_notify: Mutex::new(Vec::new()),
            txpool_notify: Mutex::new(Vec::new()),
            wallet_notify: Mutex::new(Vec::new()),
        })
    }

    pub async fn serve(
        config: &Config,
        log_manager: &LogManager,
        node: Arc<Node>,
        shutdown_send: mpsc::UnboundedSender<()>,
    ) {
        let rpc_server = RPCServer::new();
        let logger = log_manager.logger("RPCServer").unwrap();
        let logger_shutdown = logger.clone();
        let router = Router::new()
            .route(
                "/api/shutdown",
                get(|| async move {
                    info!(logger_shutdown, "Shutdown requested");
                    let _ = shutdown_send.send(());
                }),
            )
            .merge(v2::routes(node, rpc_server));
        let addr = format!("{}:{}", config.bind.host, config.bind.port);
        match TcpListener::bind(&addr).await {
            Ok(listener) => {
                info!(logger, "Serving RPC at {addr}");
                let _ = axum::serve(listener, router).await;
                unreachable!();
            }
            Err(err) => {
                error!(logger, "Can't bind to {addr} because {err}");
            }
        }
    }

    pub(super) fn subscribe_block(&self, subscriber: &Subscriber) {
        let mut block_notify = self.block_notify.lock().unwrap();
        block_notify.push(subscriber.clone());
    }

    pub(super) fn subscribe_txpool(&self, subscriber: &Subscriber) {
        let mut txpool_notify = self.txpool_notify.lock().unwrap();
        txpool_notify.push(subscriber.clone());
    }

    pub(super) fn subscribe_wallet(&self, subscriber: &Subscriber) {
        let mut wallet_notify = self.wallet_notify.lock().unwrap();
        wallet_notify.push(subscriber.clone());
    }

    pub(super) fn unsubscribe_block(&self, subscriber: &Subscriber) {
        let mut block_notify = self.block_notify.lock().unwrap();
        if let Some(index) = block_notify
            .iter()
            .map(Subscriber::id)
            .position(|id| id == subscriber.id)
        {
            block_notify.swap_remove(index);
        }
    }

    pub(super) fn unsubscribe_txpool(&self, subscriber: &Subscriber) {
        let mut txpool_notify = self.txpool_notify.lock().unwrap();
        if let Some(index) = txpool_notify
            .iter()
            .map(Subscriber::id)
            .position(|id| id == subscriber.id)
        {
            txpool_notify.swap_remove(index);
        }
    }

    pub(super) fn unsubscribe_wallet(&self, subscriber: &Subscriber) {
        let mut wallet_notify = self.wallet_notify.lock().unwrap();
        if let Some(index) = wallet_notify
            .iter()
            .map(Subscriber::id)
            .position(|id| id == subscriber.id)
        {
            wallet_notify.swap_remove(index);
        }
    }

    pub(super) fn create_subscriber(&self) -> (Subscriber, mpsc::Receiver<Message>) {
        const BUFFER: usize = 65536;
        let (sender, receiver) = mpsc::channel(BUFFER);
        let id = self.next_subscriber_id();
        let subscriber = Subscriber { id, sender };
        (subscriber, receiver)
    }

    fn next_subscriber_id(&self) -> NonZero<u64> {
        let n = self.next_subscriber_id.fetch_add(1, Ordering::Relaxed);
        NonZero::<u64>::new(n).expect("64-bit id is enough")
    }
}

#[expect(dead_code)]
#[derive(Clone)]
pub(super) struct Subscriber {
    id: NonZero<u64>,
    sender: mpsc::Sender<Message>,
}

impl Subscriber {
    const fn id(&self) -> NonZero<u64> {
        self.id
    }
}

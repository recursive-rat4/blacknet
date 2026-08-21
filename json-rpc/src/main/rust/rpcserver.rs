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
use blacknet_network::{db::BlockNotifier, node::Node, txpool::Notifier as TxPoolNotifier};
use blacknet_wallet::address::AddressCodec;
use core::num::NonZero;
use serde_json::to_string;
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};
use tokio::{
    net::TcpListener,
    runtime::Runtime,
    sync::{Mutex, mpsc},
};

pub struct RPCServer {
    next_subscriber_id: AtomicU64,
    block_subscribers: Mutex<Vec<Subscriber>>,
    txpool_subscribers: Mutex<Vec<Subscriber>>,
    wallet_subscribers: Mutex<Vec<Subscriber>>,
    address_codec: AddressCodec,
}

impl RPCServer {
    pub fn new(runtime: &Runtime, node: &Node) -> Arc<Self> {
        let rpc_server = Arc::new(Self {
            next_subscriber_id: AtomicU64::new(1),
            block_subscribers: Mutex::new(Vec::new()),
            txpool_subscribers: Mutex::new(Vec::new()),
            wallet_subscribers: Mutex::new(Vec::new()),
            address_codec: AddressCodec::new(node.mode()).unwrap(),
        });
        runtime.spawn(RPCServer::block_observer(
            rpc_server.clone(),
            node.block_db().subscribe(),
        ));
        runtime.spawn(RPCServer::txpool_observer(
            rpc_server.clone(),
            node.tx_pool().read().unwrap().subscribe(),
        ));
        //TODO wallet
        rpc_server
    }

    pub async fn serve(
        self: Arc<Self>,
        config: &Config,
        log_manager: &LogManager,
        node: Arc<Node>,
        shutdown_send: mpsc::UnboundedSender<()>,
    ) {
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
            .merge(v2::routes(node, self));

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

    pub(super) async fn subscribe_block(&self, subscriber: &Subscriber) {
        let mut block_subscribers = self.block_subscribers.lock().await;
        block_subscribers.push(subscriber.clone());
    }

    pub(super) async fn subscribe_txpool(&self, subscriber: &Subscriber) {
        let mut txpool_subscribers = self.txpool_subscribers.lock().await;
        txpool_subscribers.push(subscriber.clone());
    }

    pub(super) async fn subscribe_wallet(&self, subscriber: &Subscriber) {
        let mut wallet_subscribers = self.wallet_subscribers.lock().await;
        wallet_subscribers.push(subscriber.clone());
    }

    pub(super) async fn unsubscribe_block(&self, subscriber: &Subscriber) {
        let mut block_subscribers = self.block_subscribers.lock().await;
        if let Some(index) = block_subscribers
            .iter()
            .map(Subscriber::id)
            .position(|id| id == subscriber.id)
        {
            block_subscribers.swap_remove(index);
        }
    }

    pub(super) async fn unsubscribe_txpool(&self, subscriber: &Subscriber) {
        let mut txpool_subscribers = self.txpool_subscribers.lock().await;
        if let Some(index) = txpool_subscribers
            .iter()
            .map(Subscriber::id)
            .position(|id| id == subscriber.id)
        {
            txpool_subscribers.swap_remove(index);
        }
    }

    pub(super) async fn unsubscribe_wallet(&self, subscriber: &Subscriber) {
        let mut wallet_subscribers = self.wallet_subscribers.lock().await;
        if let Some(index) = wallet_subscribers
            .iter()
            .map(Subscriber::id)
            .position(|id| id == subscriber.id)
        {
            wallet_subscribers.swap_remove(index);
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

    async fn block_observer(self: Arc<Self>, mut block_notifier: BlockNotifier) {
        while let Some(notification) = block_notifier.recv().await {
            let mut subscribers = self.block_subscribers.lock().await;
            if subscribers.is_empty() {
                continue;
            }
            let notification =
                v2::BlockNotification::new(&notification, &self.address_codec).unwrap();
            let notification = v2::WebSocketNotification::with_block(notification).unwrap();
            let notification = to_string(&notification).unwrap();
            let notification = Message::Text(notification.into());
            let mut i = 0;
            while i < subscribers.len() {
                if subscribers[i].sender.try_send(notification.clone()).is_ok() {
                    i += 1;
                } else {
                    subscribers.swap_remove(i);
                }
            }
        }
    }

    async fn txpool_observer(self: Arc<Self>, mut txpool_notifier: TxPoolNotifier) {
        while let Some(notification) = txpool_notifier.recv().await {
            let mut subscribers = self.txpool_subscribers.lock().await;
            if subscribers.is_empty() {
                continue;
            }
            let notification =
                v2::TransactionNotification::new(&notification, &self.address_codec).unwrap();
            let notification = v2::WebSocketNotification::with_transaction(notification).unwrap();
            let notification = to_string(&notification).unwrap();
            let notification = Message::Text(notification.into());
            let mut i = 0;
            while i < subscribers.len() {
                if subscribers[i].sender.try_send(notification.clone()).is_ok() {
                    i += 1;
                } else {
                    subscribers.swap_remove(i);
                }
            }
        }
    }
}

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

/*
 * Copyright (c) 2018-2025 Pavel Vasin
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
use crate::node::NETWORK_TIMEOUT;
use crate::packet::{GetTransactions, Inventory, MAX_INVENTORY, MAX_TRANSACTIONS};
use crate::txpool::TxPool;
use blacknet_kernel::blake2b::Hash;
use blacknet_time::{Milliseconds, SystemClock};
use core::cmp::min;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock, Weak};
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};
use tokio::time::sleep;

const MAX_REQUESTS: usize = 16 * MAX_INVENTORY;

pub struct TxFetcher {
    inventory_send: UnboundedSender<(Weak<Connection>, Inventory)>,
    requests: Mutex<HashMap<Hash, (u64, Milliseconds)>>,
    tx_pool: Weak<RwLock<TxPool>>,
}

impl TxFetcher {
    pub fn new(runtime: &Runtime, tx_pool: Weak<RwLock<TxPool>>) -> Arc<Self> {
        let (inventory_send, inventory_recv) = unbounded_channel();
        let tx_fetcher = Arc::new(Self {
            inventory_send,
            requests: Mutex::new(HashMap::new()),
            tx_pool,
        });

        runtime.spawn(tx_fetcher.clone().implementation(inventory_recv));
        runtime.spawn(tx_fetcher.clone().watchdog());

        tx_fetcher
    }

    pub fn offer(&self, connection: Weak<Connection>, inventory: Inventory) {
        self.inventory_send.send((connection, inventory)).unwrap();
    }

    pub fn fetched(&self, connection: &Connection, hash: Hash) -> bool {
        let mut requests = self.requests.lock().unwrap();
        if let Some(&(id, _)) = requests.get(&hash)
            && connection.id() == id
        {
            return requests.remove(&hash).is_some();
        }
        false
    }

    async fn implementation(
        self: Arc<Self>,
        mut inventory_recv: UnboundedReceiver<(Weak<Connection>, Inventory)>,
    ) {
        loop {
            let (connection, inventory) = inventory_recv.recv().await.unwrap();
            let connection = if let Some(connection) = connection.upgrade() {
                connection
            } else {
                continue;
            };

            let mut requests = self.requests.lock().unwrap();
            if requests.len() >= MAX_REQUESTS {
                continue;
            }

            let capacity = min(inventory.len(), MAX_TRANSACTIONS);
            let mut request = GetTransactions::with_capacity(capacity);
            let tx_pool_strong = self.tx_pool.upgrade().unwrap();
            let tx_pool = tx_pool_strong.read().unwrap();
            let now = SystemClock::millis();

            for hash in inventory {
                if requests.contains_key(&hash) {
                    continue;
                }

                if tx_pool.is_interesting(hash) {
                    requests.insert(hash, (connection.id(), now));
                    request.push(hash);
                }

                if request.len() == MAX_TRANSACTIONS {
                    connection.send_packet(&request);
                    request.clear();
                }
            }

            if !request.is_empty() {
                connection.send_packet(&request);
            }
        }
    }

    async fn watchdog(self: Arc<Self>) {
        loop {
            sleep(NETWORK_TIMEOUT.try_into().unwrap()).await;

            let mut requests = self.requests.lock().unwrap();
            let now = SystemClock::millis();
            requests.retain(|_, &mut (_, time)| now < time + NETWORK_TIMEOUT);
        }
    }
}

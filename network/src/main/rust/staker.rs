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

use crate::{
    db::{BlockNotifier, CoinDB, State as CoinDBState},
    node::Node,
    wallet::WalletDB,
};
use blacknet_kernel::{
    amount::Amount,
    blake2b::Hash,
    block::Block,
    ed25519::{PublicKey, SecretKey},
    proofofstake::{MAX_DIFFICULTY, target_block_time, time_slot, verify},
};
use blacknet_log::{Error as LogError, LogManager, Logger, error, info, warn};
use blacknet_time::{Milliseconds, Seconds, SystemClock};
use core::fmt;
use std::sync::{Arc, Mutex};
use tokio::{
    runtime::{Handle, Runtime},
    task::{AbortHandle, JoinHandle},
    time::sleep,
};

pub struct Staker {
    logger: Logger,
    inner: Mutex<Inner>,
    node: Arc<Node>,
    runtime: Handle,
}

impl Staker {
    pub fn new(
        log_manager: &LogManager,
        runtime: &Runtime,
        node: Arc<Node>,
        wallet_db: &WalletDB,
    ) -> Result<Arc<Self>, LogError> {
        let logger = log_manager.logger("Staker")?;
        let staker = Arc::new(Self {
            logger: logger.clone(),
            inner: Mutex::new(Inner::new(logger)),
            node,
            runtime: runtime.handle().clone(),
        });

        for (public_key, wallet) in wallet_db.wallets() {
            match wallet.is_staking() {
                Ok(true) => match wallet.secret_key() {
                    Ok(secret_key) => {
                        staker.start_staking(public_key, &secret_key);
                    }
                    Err(err) => error!(staker.logger, "{err}"),
                },
                Ok(false) => continue,
                Err(err) => error!(staker.logger, "{err}"),
            }
        }

        runtime.spawn(Staker::block_observer(
            staker.clone(),
            staker.node.block_db().subscribe(),
        ));

        Ok(staker)
    }

    pub fn start_staking(self: &Arc<Self>, public_key: &PublicKey, secret_key: &SecretKey) -> bool {
        let mut inner = self.inner.lock().unwrap();

        if inner
            .holders
            .iter()
            .any(|holder| holder.public_key == *public_key)
        {
            info!(self.logger, "Stakeholder is already active");
            return false;
        }

        let mut holder = Holder::new(*public_key, *secret_key);
        let coin_db = self.node.coin_db();
        holder.update(coin_db, &coin_db.state().load());
        if holder.stake == Amount::ZERO {
            warn!(self.logger, "Stakeholder has zero active balance");
        }

        if inner.holders.is_empty() {
            inner.worker = Some(self.runtime.spawn(self.clone().run()));
            inner.set_state(State::Started);
        }
        inner.holders.push(holder);
        true
    }

    pub fn stop_staking(&self, public_key: &PublicKey) -> bool {
        let mut inner = self.inner.lock().unwrap();

        if let Some(idx) = inner
            .holders
            .iter()
            .position(|holder| holder.public_key == *public_key)
        {
            inner.holders.swap_remove(idx);
        } else {
            info!(self.logger, "Stakeholder is not active");
            return false;
        }

        if inner.holders.is_empty() {
            inner.worker.as_ref().expect("worker").abort();
            inner.worker = None;
            inner.waiter = None;
            inner.set_state(State::Stopped);
        }
        true
    }

    pub fn is_staking(&self, public_key: &PublicKey) -> bool {
        let inner = self.inner.lock().unwrap();
        inner
            .holders
            .iter()
            .any(|holder| holder.public_key == *public_key)
    }

    pub fn stats(&self, public_key: &Option<PublicKey>) -> StakerStats {
        let mut stats = StakerStats::new();
        let inner = self.inner.lock().unwrap();

        if let Some(public_key) = public_key {
            if let Some(holder) = inner
                .holders
                .iter()
                .find(|holder| holder.public_key == *public_key)
            {
                stats.staking_accounts = 1;
                stats.hash_rate += holder.hash_rate();
                stats.weight += holder.stake;
            }
        } else {
            stats.staking_accounts = inner.holders.len() as u32;
            for holder in &inner.holders {
                stats.hash_rate += holder.hash_rate();
                stats.weight += holder.stake;
            }
        }

        let state = self.node.coin_db().state().load();
        let pos_version = state.pos_version();
        let k = (MAX_DIFFICULTY / state.difficulty()).limbs()[0];
        let target_block_time = target_block_time(pos_version).value() as u64;
        let time_slot = time_slot(pos_version).value() as u64;
        stats.network_weight = Amount::new(k / target_block_time * time_slot);
        stats.expected_time = if stats.weight != Amount::ZERO {
            Seconds::new((stats.network_weight * target_block_time / stats.weight) as i64)
        } else {
            Seconds::ZERO
        };
        stats
    }

    async fn run(self: Arc<Self>) {
        loop {
            let enter_time = SystemClock::secs();
            let pos_time_slot = time_slot(self.node.coin_db().state().load().pos_version());
            let next_time_slot = enter_time - enter_time % pos_time_slot + pos_time_slot;
            let d = next_time_slot.to_millis() - SystemClock::millis();
            if d > Milliseconds::ZERO {
                let waiter = self.runtime.spawn(sleep(d.try_into().unwrap()));
                self.inner.lock().unwrap().waiter = Some(waiter.abort_handle());
                let _ = waiter.await;
            }

            let Some((mut block, signer)) = self.search() else {
                continue;
            };
            self.node.tx_pool().read().unwrap().fill(&mut block);
            let (hash, bytes) = block.sign(signer);
            info!(self.logger, "Staked {hash}");
            if self.node.broadcast_block(hash, bytes.into()).await {
                continue;
            } else {
                let state = self.node.coin_db().state().load();
                if block.time() <= state.block_time() {
                    continue;
                }
                if block.is_empty() {
                    continue;
                }
                block.clear();
                if self.node.tx_pool().write().unwrap().check().is_err() {
                    self.node.tx_pool().read().unwrap().fill(&mut block);
                    if !block.is_empty() {
                        let (hash, bytes) = block.sign(signer);
                        warn!(self.logger, "Retry {hash}");
                        if self.node.broadcast_block(hash, bytes.into()).await {
                            continue;
                        } else {
                            block.clear()
                        }
                    }
                }
                let (hash, bytes) = block.sign(signer);
                warn!(self.logger, "Empty {hash}");
                self.node.broadcast_block(hash, bytes.into()).await;
            }
        }
    }

    fn search(&self) -> Option<(Block, SecretKey)> {
        let mut inner = self.inner.lock().unwrap();
        inner.waiter = None;

        if self.node.mode().requires_network() {
            if !self.node.is_online() {
                inner.set_state(State::AwaitingOnline);
                return None;
            }
            if self.node.is_initial_synchronization() {
                inner.set_state(State::AwaitingSync);
                return None;
            }
        }

        inner.set_state(State::Staking);

        let state = self.node.coin_db().state().load();
        let curr_time = SystemClock::secs();
        let curr_time_slot = curr_time - curr_time % time_slot(state.pos_version());
        if curr_time_slot <= state.block_time() {
            return None;
        }

        for holder in &mut inner.holders {
            if holder.last_block != state.block_hash() {
                holder.update(self.node.coin_db(), &state);
            }
            holder.hash_counter += 1;
            let pos_version = state.pos_version();
            if verify(
                pos_version,
                curr_time_slot,
                holder.public_key,
                state.nxtrng(),
                state.difficulty(),
                state.block_time(),
                holder.stake,
            )
            .is_ok()
            {
                return Some((
                    Block::new(state.block_hash(), curr_time_slot, holder.public_key),
                    holder.secret_key,
                ));
            }
        }

        None
    }

    async fn block_observer(self: Arc<Self>, mut block_notifier: BlockNotifier) {
        while block_notifier.recv().await.is_some() {
            if let Some(waiter) = self.inner.lock().unwrap().waiter.take() {
                waiter.abort()
            }
        }
    }
}

impl Drop for Staker {
    fn drop(&mut self) {
        let mut inner = self.inner.lock().unwrap();
        inner.set_state(State::Terminating);
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum State {
    Initializing,
    Terminating,
    AwaitingOnline,
    AwaitingSync,
    Staking,
    Started,
    Stopped,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::Initializing => f.write_str("Initializing staker"),
            State::Terminating => f.write_str("Terminating staker"),
            State::AwaitingOnline => f.write_str("Awaiting to get online"),
            State::AwaitingSync => f.write_str("Awaiting to get synchronized"),
            State::Staking => f.write_str("Staking"),
            State::Started => f.write_str("Started staker"),
            State::Stopped => f.write_str("Stopped staker"),
        }
    }
}

struct Inner {
    logger: Logger,
    state: State,
    holders: Vec<Holder>,
    waiter: Option<AbortHandle>,
    worker: Option<JoinHandle<()>>,
}

impl Inner {
    const fn new(logger: Logger) -> Self {
        Self {
            logger,
            state: State::Initializing,
            holders: Vec::new(),
            waiter: None,
            worker: None,
        }
    }

    fn set_state(&mut self, state: State) {
        if self.state == state {
            return;
        }
        self.state = state;
        info!(self.logger, "{state}");
    }
}

struct Holder {
    public_key: PublicKey,
    secret_key: SecretKey,
    start_time: Seconds,
    hash_counter: u64,
    last_block: Hash,
    stake: Amount,
}

impl Holder {
    fn new(public_key: PublicKey, secret_key: SecretKey) -> Self {
        Self {
            public_key,
            secret_key,
            start_time: SystemClock::secs(),
            hash_counter: 0,
            last_block: Hash::ZERO,
            stake: Amount::ZERO,
        }
    }

    fn hash_rate(&self) -> f64 {
        let time = SystemClock::secs() - self.start_time;
        if time != Seconds::ZERO {
            self.hash_counter as f64 / time.value() as f64
        } else {
            0.0
        }
    }

    fn update(&mut self, coin_db: &CoinDB, state: &CoinDBState) {
        self.last_block = state.block_hash();
        self.stake = coin_db
            .account(self.public_key)
            .map(|account| account.staking_balance(state.height()))
            .unwrap_or(Amount::ZERO);
    }
}

pub struct StakerStats {
    staking_accounts: u32,
    hash_rate: f64,
    weight: Amount,
    network_weight: Amount,
    expected_time: Seconds,
}

impl StakerStats {
    const fn new() -> Self {
        Self {
            staking_accounts: 0,
            hash_rate: 0.0,
            weight: Amount::ZERO,
            network_weight: Amount::ZERO,
            expected_time: Seconds::ZERO,
        }
    }

    pub const fn staking_accounts(&self) -> u32 {
        self.staking_accounts
    }

    pub const fn hash_rate(&self) -> f64 {
        self.hash_rate
    }

    pub const fn weight(&self) -> Amount {
        self.weight
    }

    pub const fn network_weight(&self) -> Amount {
        self.network_weight
    }

    pub const fn expected_time(&self) -> Seconds {
        self.expected_time
    }
}

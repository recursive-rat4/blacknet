/*
 * Copyright (c) 2026 Pavel Vasin
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

mod blockdb;
mod blockindex;
mod coindb;
mod dbversion;
mod dbview;
mod fjall;
pub mod genesis;
mod undoblock;
mod writebatch;

use coindb::Update;
use dbversion::DBVersionKey;
use dbview::DBView;
use undoblock::UndoBlock;
use writebatch::WriteBatch;

pub use blockdb::{
    BlockDB, BlockDBCheck, Notification as BlockNotification, Notifier as BlockNotifier,
    Subscriber as BlockSubscriber,
};
pub use blockindex::BlockIndex;
pub use coindb::{CoinDB, CoinDBCheck, State};
pub use dbversion::DBVersion;
pub use fjall::Fjall;

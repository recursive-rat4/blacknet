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

use crate::amount::Amount;
use crate::ed25519::PublicKey;
use crate::hashlock::HashLock;
use crate::timelock::TimeLock;
use serde::{Deserialize, Serialize};

pub type HashTimeLockContractId = [u8; 32];

#[derive(Deserialize, Serialize)]
pub struct CreateHTLC {
    amount: Amount,
    to: PublicKey,
    time_lock: TimeLock,
    hash_lock: HashLock,
}

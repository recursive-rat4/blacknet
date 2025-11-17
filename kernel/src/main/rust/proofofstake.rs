/*
 * Copyright (c) 2014-2025 Pavel Vasin
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
use blacknet_crypto::bigint::UInt256;
use blacknet_time::Seconds;

#[derive(Clone, Copy)]
pub enum Version {
    V4,
    V4_1,
}

pub fn guess_initial_synchronization(
    version: Version,
    external: Seconds,
    internal: Seconds,
) -> bool {
    external > internal + target_block_time(version) * (ROLLBACK_LIMIT as i64)
}

/**
 * Length of time slot
 */
pub fn time_slot(version: Version) -> Seconds {
    match version {
        Version::V4 => 16.into(),
        Version::V4_1 => 4.into(),
    }
}

/**
 * Expected block time
 */
pub fn target_block_time(version: Version) -> Seconds {
    4 * time_slot(version)
}

/**
 * Recommended number of confirmations that is not enforced by protocol
 */
pub const DEFAULT_CONFIRMATIONS: u32 = 10;

/**
 * Number of confirmations to make coins eligible for staking
 */
pub const MATURITY: u32 = 1350;

/**
 * Depth of rolling checkpoint
 */
pub const ROLLBACK_LIMIT: usize = 1350;

/**
 * Minimum amount that can be leased out for cold staking
 */
pub const MIN_LEASE: Amount = Amount::new(1000 * Amount::COIN.value());

/**
 * Difficulty of genesis block
 */
pub const INITIAL_DIFFICULTY: UInt256 =
    UInt256::from_hex("00000000000000AFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");

/**
 * Maximum value of difficulty
 */
pub const MAX_DIFFICULTY: UInt256 = UInt256::MAX;

/**
 * Reserved from maximum block size
 */
pub const BLOCK_RESERVED_SIZE: u32 = 100;

/**
 * Minimum maximum block size
 */
pub const DEFAULT_MAX_BLOCK_SIZE: u32 = 100000;

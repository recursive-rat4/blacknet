/*
 * Copyright (c) 2014-2026 Pavel Vasin
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
use crate::blake2b::Hash;
use crate::ed25519::PublicKey;
use crate::error::{Error, Result};
use alloc::boxed::Box;
use blacknet_crypto::{
    bigint::{UInt256, UInt320},
    symmetric::Blake2b256,
};
use blacknet_time::Seconds;
use core::cmp::min;

#[derive(Clone, Copy)]
pub enum Version {
    V4,
    V4_1,
}

pub fn mint(version: Version, supply: Amount) -> Amount {
    supply / 100u64 / blocks_in_year(version)
}

pub fn nxtrng(nxtrng: Hash, generator: PublicKey) -> Hash {
    let mut hasher = Blake2b256::new();
    hasher.update(nxtrng);
    hasher.update(generator);
    hasher.finalize().into()
}

pub fn verify(
    version: Version,
    time: Seconds,
    generator: PublicKey,
    nxtrng: Hash,
    difficulty: UInt256,
    prev_time: Seconds,
    stake: Amount,
) -> Result<()> {
    if stake <= Amount::ZERO {
        return Err(Error::invalid("Invalid stake amount"));
    }
    if time % time_slot(version) != Seconds::ZERO {
        return Err(Error::invalid("Invalid time slot"));
    }
    let mut hasher = Blake2b256::new();
    hasher.update(nxtrng);
    hasher.update(prev_time.to_be_bytes());
    hasher.update(generator);
    hasher.update(time.to_be_bytes());
    let hash: [u8; 32] = hasher.finalize();
    let hash: UInt320 = UInt256::from_be_bytes(hash).extend();
    let target: UInt320 = difficulty.widening_mul_limb(stake.value());
    if hash < target {
        Ok(())
    } else {
        Err(Error::invalid("Proof of stake doesn't match difficulty"))
    }
}

pub fn is_too_far_in_future(version: Version, external: Seconds, internal: Seconds) -> bool {
    internal >= external + time_slot(version)
}

pub fn next_difficulty(
    version: Version,
    difficulty: UInt256,
    prev_block_time: Seconds,
    block_time: Seconds,
) -> UInt256 {
    let d_time = min(
        block_time - prev_block_time,
        target_block_time(version) * SPACING,
    );
    let (a1, a2) = (a1(version), a2(version));
    let k = (a2 + 2 * d_time) / a1;
    let next: UInt320 = difficulty.widening_mul_limb(k as u64);
    let max: UInt320 = MAX_DIFFICULTY.extend();
    min(next, max).truncate()
}

pub fn cumulative_difficulty(cumulative_difficulty: UInt256, difficulty: UInt256) -> UInt256 {
    cumulative_difficulty + (ONE_SHL_256 / difficulty.extend()).truncate()
}

pub fn guess_initial_synchronization(
    version: Version,
    external: Seconds,
    internal: Seconds,
) -> bool {
    external > internal + target_block_time(version) * (ROLLBACK_LIMIT as i64)
}

pub fn max_block_size(block_sizes: &[u32]) -> u32 {
    if block_sizes.len() != BLOCK_SIZE_SPAN {
        return DEFAULT_MAX_BLOCK_SIZE;
    }

    let mut sizes: Box<[u32]> = block_sizes.into();
    sizes.sort();
    let median = sizes[BLOCK_SIZE_SPAN / 2];
    let size = median.saturating_mul(2);
    size.clamp(DEFAULT_MAX_BLOCK_SIZE, MAX_BLOCK_SIZE)
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
 * Expected number of blocks in year
 */
pub fn blocks_in_year(version: Version) -> u64 {
    let year = Seconds::new(365 * 24 * 60 * 60);
    (year / target_block_time(version)) as u64
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
 * Sequence of blocks to activate fork
 */
pub const UPGRADE_THRESHOLD: u16 = 1350;

/**
 * Number of blocks used to calculate the maximum block size
 */
pub const BLOCK_SIZE_SPAN: usize = 1351;

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

/**
 * Maximum block size
 */
pub const MAX_BLOCK_SIZE: u32 = i32::MAX as u32 - BLOCK_RESERVED_SIZE;

const INTERVAL: i64 = 15;
const SPACING: i64 = 10;
fn a1(version: Version) -> Seconds {
    target_block_time(version) * (INTERVAL + 1)
}
fn a2(version: Version) -> Seconds {
    target_block_time(version) * (INTERVAL - 1)
}
const ONE_SHL_256: UInt320 = UInt320::from_hex(
    "00000000000000010000000000000000000000000000000000000000000000000000000000000000",
);

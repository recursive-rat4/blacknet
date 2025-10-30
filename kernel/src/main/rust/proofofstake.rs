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
 * Reserved from maximum block size
 */
pub const BLOCK_RESERVED_SIZE: u32 = 100;

/**
 * Minimum maximum block size
 */
pub const DEFAULT_MAX_BLOCK_SIZE: u32 = 100000;

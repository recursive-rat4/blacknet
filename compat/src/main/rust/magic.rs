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

// Various magic and personalization values that must be changed if you are a fork.

/**
 * This name is used in network protocols and logs.
 */
pub(crate) const AGENT_NAME: &str = "Blacknet";

/**
 * The name of subdirectory, where data is stored.
 */
pub const XDG_SUBDIRECTORY: &str = "Blacknet";

/**
 * This name is used to prevent replay of message signatures.
 */
pub const MESSAGE_SIGN_NAME: &str = "Blacknet";

/**
 * The prefix for addresses.
 */
pub const ADDRESS_READABLE_PART: &str = "blacknet";

/**
 * The default port for peer-to-peer networking.
 */
pub(crate) const DEFAULT_P2P_PORT: u16 = 28453;

/**
 * The nonce that is used for quickly distinguishing networks.
 */
pub(crate) const NETWORK_MAGIC: u32 = 0x17895E7D;

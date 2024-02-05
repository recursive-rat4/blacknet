/*
 * Copyright (c) 2018-2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet

// Various magic and personalization values that must be changed if you are a fork.

/**
 * This name is used in network protocols and logs.
 */
const val AGENT_NAME = "Blacknet"

/**
 * The name of subdirectory, where data is stored.
 */
const val XDG_SUBDIRECTORY = "Blacknet"

/**
 * This name is used to prevent replay of message signatures.
 */
const val MESSAGE_SIGN_NAME = "Blacknet"

/**
 * The prefix for addresses.
 */
const val BECH32_HRP = "blacknet"

/**
 * The default port for peer-to-peer networking.
 */
const val DEFAULT_P2P_PORT: Short = 28453

/**
 * The nonce that is used for quickly distinguishing ledgers.
 */
const val NETWORK_MAGIC = 0x17895E7D

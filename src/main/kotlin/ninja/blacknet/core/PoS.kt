/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

import mu.KotlinLogging
import ninja.blacknet.crypto.Blake2b
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PublicKey

private val logger = KotlinLogging.logger {}

object PoS {
    fun reward(supply: Long): Long {
        val blocks = 365 * 24 * 60 * 60 / BLOCK_TIME
        return supply / 100 / blocks
    }

    fun nxtrng(nxtrng: Hash, generator: PublicKey): Hash {
        return (Blake2b.hasher() + nxtrng.bytes.array + generator.bytes.array).hash()
    }

    fun check(block: Block): Boolean {
        if (block.time and TIMESTAMP_MASK != 0L) {
            logger.info("invalid timestamp mask")
            return false
        }
        return false//TODO
    }

    const val BLOCK_TIME = 64
    const val TIMESTAMP_MASK = 15L
    const val MATURITY = 1350
    const val BLOCK_SIZE_SPAN = 1351
    const val COIN = 100000000L
    const val MIN_LEASE = 1000 * COIN
}
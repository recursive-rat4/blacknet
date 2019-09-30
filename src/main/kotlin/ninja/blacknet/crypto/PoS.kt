/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

import mu.KotlinLogging
import ninja.blacknet.Runtime
import ninja.blacknet.db.LedgerDB
import ninja.blacknet.util.byteArrayOfInts
import kotlin.math.min

private val logger = KotlinLogging.logger {}

object PoS {
    fun reward(supply: Long): Long {
        val blocks = 365 * 24 * 60 * 60 / TARGET_BLOCK_TIME
        return supply / 100 / blocks
    }

    fun nxtrng(nxtrng: Hash, generator: PublicKey): Hash {
        return (Blake2b.Hasher() + nxtrng.bytes + generator.bytes).hash()
    }

    fun check(time: Long, generator: PublicKey, nxtrng: Hash, difficulty: BigInt, prevTime: Long, stake: Long): Boolean {
        if (stake <= 0) {
            logger.info("invalid stake amount $stake")
            return false
        }
        if (time and TIMESTAMP_MASK != 0L) {
            logger.info("invalid timestamp mask")
            return false
        }
        val hash = (Blake2b.Hasher() + nxtrng.bytes + prevTime + generator.bytes + time).hash()
        val x = BigInt(hash) / stake
        return x < difficulty
    }

    fun isTooFarInFuture(time: Long): Boolean {
        return time > Runtime.time() + MAX_FUTURE_DRIFT
    }

    fun nextDifficulty(difficulty: BigInt, prevBlockTime: Long, blockTime: Long): BigInt {
        val dTime = min(blockTime - prevBlockTime, TARGET_BLOCK_TIME * SPACING)
        return difficulty * (A2 + 2 * dTime) / A1
    }

    fun cumulativeDifficulty(cumulativeDifficulty: BigInt, difficulty: BigInt): BigInt {
        return cumulativeDifficulty + ONE_SHL_256 / difficulty
    }

    fun guessInitialSynchronization(): Boolean {
        return Runtime.time() > LedgerDB.blockTime() + TARGET_BLOCK_TIME * MATURITY
    }

    /**
     * Expected block time
     */
    const val TARGET_BLOCK_TIME = 64L
    /**
     * Default number of confirmations
     */
    const val DEFAULT_CONFIRMATIONS = 10
    /**
     * Maximum number of blocks that can be rolled back
     */
    const val MATURITY = 1350
    /**
     * Number of blocks used to calculate the maximum block size
     */
    const val BLOCK_SIZE_SPAN = 1351
    /**
     * Satoshi
     */
    const val COIN = 100000000L
    /**
     * Minimum amount that can be leased out for cold staking
     */
    const val MIN_LEASE = 1000 * COIN

    const val TARGET_TIMESPAN = 960L
    const val INTERVAL = TARGET_TIMESPAN / TARGET_BLOCK_TIME
    const val SPACING = 10
    const val TIMESTAMP_MASK = 15L
    const val MAX_FUTURE_DRIFT = 15L
    val A1 = BigInt((INTERVAL + 1) * TARGET_BLOCK_TIME)
    val A2 = BigInt((INTERVAL - 1) * TARGET_BLOCK_TIME)
    val INITIAL_DIFFICULTY = BigInt(byteArrayOfInts(0x00, 0xAF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF))
    val ONE_SHL_256 = BigInt.ONE shl 256
}

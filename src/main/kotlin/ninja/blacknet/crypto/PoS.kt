/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

import ninja.blacknet.Runtime
import ninja.blacknet.core.Accepted
import ninja.blacknet.core.Invalid
import ninja.blacknet.core.Status
import ninja.blacknet.db.LedgerDB
import ninja.blacknet.util.byteArrayOfInts
import kotlin.math.min

object PoS {
    fun reward(supply: Long): Long {
        val blocks = 365 * 24 * 60 * 60 / TARGET_BLOCK_TIME
        return supply / 100 / blocks
    }

    fun nxtrng(nxtrng: Hash, generator: PublicKey): Hash {
        return Blake2b.hasher { this + nxtrng.bytes + generator.bytes }
    }

    fun check(time: Long, generator: PublicKey, nxtrng: Hash, difficulty: BigInt, prevTime: Long, stake: Long): Status {
        if (stake <= 0) {
            return Invalid("Invalid stake amount")
        }
        if (time % TIME_SLOT != 0L) {
            return Invalid("Invalid time slot")
        }
        val hash = Blake2b.hasher { this + nxtrng.bytes + prevTime + generator.bytes + time }
        val valid = BigInt(hash) / stake < difficulty
        return if (valid)
            Accepted
        else
            Invalid("Invalid proof of stake hash")
    }

    fun isTooFarInFuture(time: Long): Boolean {
        return time >= Runtime.time() + TIME_SLOT
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

    const val TIME_SLOT = 16L
    /**
     * Expected block time
     */
    const val TARGET_BLOCK_TIME = 4 * TIME_SLOT
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
    val A1 = BigInt((INTERVAL + 1) * TARGET_BLOCK_TIME)
    val A2 = BigInt((INTERVAL - 1) * TARGET_BLOCK_TIME)
    /**
     * Difficulty of genesis block
     */
    val INITIAL_DIFFICULTY = BigInt(byteArrayOfInts(0x00, 0xAF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF))
    /**
     * Maximum value of difficulty
     */
    val MAX_DIFFICULTY = BigInt(byteArrayOfInts(0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF))
    val ONE_SHL_256 = BigInt.ONE shl 256
}

/*
 * Copyright (c) 2018-2019 Pavel Vasin
 * Copyright (c) 2019 Blacknet Team
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

import kotlinx.coroutines.Job
import kotlinx.coroutines.launch
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.sync.withLock
import mu.KotlinLogging
import ninja.blacknet.Config
import ninja.blacknet.Config.mnemonics
import ninja.blacknet.Runtime
import ninja.blacknet.crypto.*
import ninja.blacknet.db.BlockDB
import ninja.blacknet.db.LedgerDB
import ninja.blacknet.network.Node
import ninja.blacknet.util.SynchronizedArrayList
import ninja.blacknet.util.delay

private val logger = KotlinLogging.logger {}

object Staker {
    private class State(
            val publicKey: PublicKey,
            val privateKey: PrivateKey,
            var lastBlock: Hash = Hash.ZERO,
            var stake: Long = 0
    ) {
        fun update() {
            lastBlock = LedgerDB.blockHash()
            stake = LedgerDB.get(publicKey)?.stakingBalance(LedgerDB.height()) ?: 0
        }
    }

    private val stakers = SynchronizedArrayList<State>()

    init {
        if (Config.contains(mnemonics)) {
            runBlocking {
                Config[mnemonics].forEachIndexed { index, mnemonic ->
                    val privateKey = Mnemonic.fromString(mnemonic)
                    if (privateKey != null) {
                        startStaking(privateKey)
                    } else {
                        logger.warn("Invalid mnemonic $index")
                    }
                }
                val n = stakers.list.size
                if (n == 1)
                    logger.info("Started staking")
                else if (n > 1)
                    logger.info("Started staking with $n accounts")
            }
        }
    }

    private var job: Job? = null
    private suspend fun staker() {
        while (true) {
            delay(1)

            if (Node.isOffline())
                continue

            if (Node.isInitialSynchronization())
                continue

            val currTime = Runtime.time()
            val timeSlot = currTime - currTime % PoS.TIME_SLOT
            if (timeSlot <= LedgerDB.blockTime())
                continue

            @Suppress("LABEL_NAME_CLASH")
            val block = stakers.mutex.withLock {
                BlockDB.mutex.withLock {
                    for (i in stakers.list.indices) {
                        val state = stakers.list[i]

                        if (state.lastBlock != LedgerDB.blockHash())
                            state.update()

                        if (state.stake > 0 && PoS.check(timeSlot, state.publicKey, LedgerDB.nxtrng(), LedgerDB.difficulty(), LedgerDB.blockTime(), state.stake)) {
                            val block = Block.create(LedgerDB.blockHash(), timeSlot, state.publicKey)
                            TxPool.fill(block)
                            return@withLock block.sign(state.privateKey)
                        }
                    }
                    return@withLock null
                }
            }

            if (block != null) {
                logger.info("Staked ${block.first}")
                Node.broadcastBlock(block.first, block.second)
            }
        }
    }

    suspend fun startStaking(privateKey: PrivateKey): Boolean = stakers.mutex.withLock {
        val publicKey = privateKey.toPublicKey()

        if (stakers.list.find { it.publicKey == publicKey } != null) {
            logger.info("${Address.encode(publicKey)} is already staking")
            return false
        }

        val state = State(publicKey, privateKey)
        BlockDB.mutex.withLock {
            state.update()
        }
        if (state.stake == 0L)
            logger.warn("${Address.encode(publicKey)} has 0 staking balance")

        stakers.list.add(state)
        if (stakers.list.size == 1)
            job = Runtime.launch { staker() }
        return true
    }

    suspend fun stopStaking(privateKey: PrivateKey): Boolean = stakers.mutex.withLock {
        val publicKey = privateKey.toPublicKey()
        val i = stakers.list.indexOfFirst { it.publicKey == publicKey }
        if (i != -1) {
            stakers.list.removeAt(i)
        } else {
            logger.info("${Address.encode(publicKey)} is not staking")
            return false
        }
        if (stakers.list.size == 0) {
            job!!.cancel()
            job = null
        }
        return true
    }

    suspend fun isStaking(privateKey: PrivateKey): Boolean = stakers.mutex.withLock {
        return stakers.list.find { it.privateKey == privateKey } != null
    }
}

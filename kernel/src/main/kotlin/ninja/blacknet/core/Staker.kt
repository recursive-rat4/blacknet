/*
 * Copyright (c) 2018-2024 Pavel Vasin
 * Copyright (c) 2019 Blacknet Team
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

import io.github.oshai.kotlinlogging.KotlinLogging
import java.lang.Thread.sleep
import java.util.concurrent.CopyOnWriteArrayList
import ninja.blacknet.Kernel
import kotlin.concurrent.withLock
import ninja.blacknet.ShutdownHooks
import ninja.blacknet.crypto.*
import ninja.blacknet.db.LedgerDB
import ninja.blacknet.mode
import ninja.blacknet.network.Node
import ninja.blacknet.rpc.v2.StakingInfo
import ninja.blacknet.time.currentTimeMillis
import ninja.blacknet.time.currentTimeSeconds
import ninja.blacknet.util.rotate
import ninja.blacknet.util.startInterruptible

private val logger = KotlinLogging.logger {}

/**
 * 權益采塊器
 */
object Staker {
    private class StakerState(
            val publicKey: PublicKey,
            val privateKey: ByteArray
    ) {
        val startTime = currentTimeMillis()
        var hashCounter = 0
        var lastBlock = Hash.ZERO
        var stake = 0L

        fun hashRate(): Double {
            val time = currentTimeMillis() - startTime
            return if (time != 0L)
                hashCounter.toDouble() / (time / 1000L)
            else
                0.0
        }

        fun updateImpl(state: LedgerDB.State) {
            lastBlock = state.blockHash
            stake = LedgerDB.get(publicKey)?.stakingBalance(state.height) ?: 0
        }
    }

    private val stakers = CopyOnWriteArrayList<StakerState>()
    private var state: String = "Initializing staker"
        set(value) {
            if (field === value)
                return
            field = value
            logger.info { value }
        }

    init {
        Kernel.config().mnemonics?.let { mnemonics ->
            mnemonics.forEach { mnemonic ->
                val privateKey = mnemonic
                startStaking(privateKey)
            }
            Kernel.config().mnemonics = null
            ShutdownHooks.add {
                vThread?.let {
                    state = "Terminating staker"
                    it.interrupt()
                }
            }
        }
    }

    @Volatile
    var awaitsNextTimeSlot: Thread? = null
    private var vThread: Thread? = null
    private fun implementation() {
        val enterTime = currentTimeSeconds()
        val nextTimeSlot = enterTime - enterTime % PoS.TIME_SLOT + PoS.TIME_SLOT
        val d = nextTimeSlot * 1000L - currentTimeMillis()
        if (d > 0) {
            val job = startInterruptible("Staker::awaitNextTimeSlot") {
                sleep(d)
            }
            awaitsNextTimeSlot = job
            job.join()
            awaitsNextTimeSlot = null
        }

        if (mode.requiresNetwork) {
            if (Node.isOffline()) {
                state = "Awaiting to get online"
                return
            }

            if (Node.isInitialSynchronization()) {
                state = "Awaiting to get synchronized"
                return
            }
        }

        state = "Staking"

        var state = LedgerDB.state()
        val currTime = currentTimeSeconds()
        val timeSlot = currTime - currTime % PoS.TIME_SLOT
        if (timeSlot <= state.blockTime)
            return

        stakers.forEach { staker ->
            if (staker.lastBlock != state.blockHash) {
                Kernel.blockDB().reentrant.readLock().withLock {
                    state = LedgerDB.state()
                    staker.updateImpl(state)
                }
            }
            staker.hashCounter += 1
            val pos = PoS.check(timeSlot, staker.publicKey, state.nxtrng, state.difficulty, state.blockTime, staker.stake)
            if (pos == Accepted) {
                val block = Block.create(state.blockHash, timeSlot, staker.publicKey)
                Kernel.txPool().fill(block)
                val (hash, bytes) = block.sign(staker.privateKey)
                logger.info { "Staked $hash" }
                if (Node.broadcastBlock(hash, bytes)) {
                    return
                } else @Suppress("NAME_SHADOWING") {
                    state = LedgerDB.state()
                    if (timeSlot <= state.blockTime)
                        return
                    if (block.transactions.isEmpty())
                        return
                    block.transactions.clear()
                    if (Kernel.txPool().check() == false) {
                        Kernel.txPool().fill(block)
                        if (block.transactions.isNotEmpty()) {
                            val (hash, bytes) = block.sign(staker.privateKey)
                            logger.warn { "Retry $hash" }
                            if (Node.broadcastBlock(hash, bytes))
                                return
                            else
                                block.transactions.clear()
                        }
                    }
                    val (hash, bytes) = block.sign(staker.privateKey)
                    logger.warn { "Empty $hash" }
                    Node.broadcastBlock(hash, bytes)
                }
            }
        }
    }

    fun startStaking(privateKey: ByteArray): Boolean = synchronized(stakers) {
        val publicKey = Ed25519.toPublicKey(privateKey)

        if (stakers.find { it.publicKey == publicKey } != null) {
            logger.info { "Stakeholder is already active" }
            return false
        }

        val staker = StakerState(publicKey, privateKey)
        Kernel.blockDB().reentrant.readLock().withLock {
            val state = LedgerDB.state()
            staker.updateImpl(state)
        }
        if (staker.stake == 0L) {
            logger.warn { "Stakeholder has zero active balance" }
        }

        stakers.add(staker)
        if (stakers.size == 1) {
            vThread = rotate("Staker::implementation", ::implementation)
            state = "Started staker"
        }
        return true
    }

    fun stopStaking(privateKey: ByteArray): Boolean = synchronized(stakers) {
        val publicKey = Ed25519.toPublicKey(privateKey)
        val i = stakers.indexOfFirst { it.publicKey == publicKey }
        if (i != -1) {
            stakers.removeAt(i)
        } else {
            logger.info { "Stakeholder is not active" }
            return false
        }
        if (stakers.size == 0) {
            vThread!!.interrupt()
            vThread = null
            awaitsNextTimeSlot = null
            state = "Stopped staker"
        }
        return true
    }

    fun isStaking(privateKey: ByteArray): Boolean {
        return stakers.find { it.privateKey.contentEquals(privateKey) } != null
    }

    fun info(publicKey: PublicKey?): StakingInfo {
        var nAccounts: Int = 0
        var hashRate: Double = 0.0
        var weight: Long = 0L
        if (publicKey == null) {
            stakers.forEach {
                nAccounts += 1
                hashRate += it.hashRate()
                weight = Math.addExact(weight, it.stake)
            }
        } else {
            stakers.find { it.publicKey == publicKey }?.let {
                nAccounts = 1
                hashRate = it.hashRate()
                weight = it.stake
            }
        }
        val state = LedgerDB.state()
        val networkWeight = (PoS.MAX_DIFFICULTY / state.difficulty).toLong() / PoS.TARGET_BLOCK_TIME * PoS.TIME_SLOT
        val expectedTime = if (weight != 0L) PoS.TARGET_BLOCK_TIME * networkWeight / weight else 0L
        return StakingInfo(nAccounts, hashRate, weight.toString(), networkWeight.toString(), expectedTime)
    }
}

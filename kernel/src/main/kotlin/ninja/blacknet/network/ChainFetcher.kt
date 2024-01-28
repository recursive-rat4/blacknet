/*
 * Copyright (c) 2018-2024 Pavel Vasin
 * Copyright (c) 2019 Blacknet Team
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.network

import io.github.oshai.kotlinlogging.KotlinLogging
import java.math.BigInteger
import java.util.concurrent.CompletableFuture
import kotlinx.coroutines.*
import kotlinx.coroutines.channels.Channel
import kotlinx.coroutines.channels.ClosedSendChannelException
import kotlinx.coroutines.sync.withLock
import ninja.blacknet.Kernel
import ninja.blacknet.Runtime
import ninja.blacknet.ShutdownHooks
import ninja.blacknet.core.*
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PoS
import ninja.blacknet.db.LedgerDB
import ninja.blacknet.network.packet.Blocks
import ninja.blacknet.network.packet.ChainAnnounce
import ninja.blacknet.network.packet.ChainFork
import ninja.blacknet.network.packet.GetBlocks
import ninja.blacknet.network.packet.PacketType
import ninja.blacknet.util.rotate

private val logger = KotlinLogging.logger {}

/**
 * 區塊鏈獲取器
 */
object ChainFetcher {
    //TODO review capacity
    private val announces = Channel<Pair<Connection, ChainAnnounce>?>(16)
    private val deferChannel = Channel<Triple<Connection, Blocks, BigInteger>>(16)
    @Volatile
    private var request: CompletableDeferred<Blocks>? = null
    private var connectedBlocks = 0
    @Volatile
    private var stakedBlock: Triple<Hash, ByteArray, CompletableFuture<Pair<Status, Int>>>? = null
    @Volatile
    private var syncConnection: Connection? = null
    private var originalChain: Hash? = null
    private var rollbackTo: Hash? = null
    private var undoDifficulty = BigInteger.ZERO
    private var undoRollback: List<Hash>? = null

    @Volatile
    private var shutdown = false
    private val coroutine = Runtime.rotate(::implementation)

    init {
        ShutdownHooks.add {
            logger.info { "Interrupting ChainFetcher" }
            shutdown = true
            coroutine.cancel()
        }
    }

    fun join(): Unit = runBlocking {
        coroutine.join()
        if (!shutdown)
            throw RuntimeException("ChainFetcher exited")
    }

    fun isSynchronizing(): Boolean {
        return syncConnection != null
    }

    fun disconnected(connection: Connection) {
        if (syncConnection != connection)
            return

        request?.cancel(CancellationException("Connection closed"))
    }

    fun offer(connection: Connection, announce: ChainAnnounce) {
        if (announce.cumulativeDifficulty <= LedgerDB.state().cumulativeDifficulty)
            return

        announces.trySend(Pair(connection, announce))
    }

    fun stakedBlock(hash: Hash, bytes: ByteArray): Pair<Status, Int> {
        val future = CompletableFuture<Pair<Status, Int>>()
        stakedBlock = Triple(hash, bytes, future)
        request?.cancel(CancellationException("Staked new block"))
        announces.trySend(null)
        return future.get()
    }

    private suspend fun implementation() {
        val data = announces.receive()

        stakedBlock?.let { (hash, bytes, future) ->
            val status = Kernel.blockDB().mutex.withLock {
                Kernel.blockDB().processImpl(hash, bytes)
            }
            val n = when (status) {
                Accepted -> Node.announceChain(hash, LedgerDB.state().cumulativeDifficulty)
                else -> 0
            }

            stakedBlock = null
            future.complete(Pair(status, n))
        }

        deferChannel.tryReceive().getOrNull()?.let { (connection, answer, requestedDifficulty) ->
            if (requestedDifficulty <= LedgerDB.state().cumulativeDifficulty)
                return@let

            processDeferred(connection, answer)
        }

        if (data == null)
            return

        val (connection, announce) = data

        if (connection.requestedBlocks) {
            return
        }

        var state = LedgerDB.state()
        if (announce.cumulativeDifficulty <= state.cumulativeDifficulty)
            return

        Kernel.blockDB().mutex.withLock {
            if (Kernel.blockDB().isRejectedImpl(announce.chain)) {
                connection.dos("Rejected chain")
                return
            }

            logger.info { "Fetching ${announce.chain}" }
            syncConnection = connection
            originalChain = state.blockHash

            try {
                requestBlocks(connection, state.blockHash, state.rollingCheckpoint, announce.cumulativeDifficulty)

                requestLoop@ while (true) {
                    val answer = withTimeout(timeout()) {
                        request!!.await()
                    }
                    if (answer.blocks.isNotEmpty()) {
                        val accepted = processBlocks(connection, answer)
                        if (!accepted)
                            break

                        state = LedgerDB.state()

                        if (announce.cumulativeDifficulty > state.cumulativeDifficulty) {
                            requestBlocks(connection, state.blockHash, state.rollingCheckpoint, announce.cumulativeDifficulty)
                        } else {
                            break
                        }
                    } else if (answer.hashes.isNotEmpty()) {
                        if (rollbackTo != null || connectedBlocks != 0) {
                            connection.dos("Unexpected rollback")
                            break
                        }
                        var prev = state.rollingCheckpoint
                        for (hash in answer.hashes) {
                            if (Kernel.blockDB().isRejectedImpl(hash)) {
                                connection.dos("Rejected chain")
                                break@requestLoop
                            }
                            val chainIndex = LedgerDB.chainIndexes.get(hash.bytes)
                            if (chainIndex == null)
                                break
                            if (chainIndex.height < state.height - PoS.ROLLBACK_LIMIT) {
                                connection.dos("Rollback to ${chainIndex.height}")
                                break@requestLoop
                            }
                            prev = hash
                        }
                        rollbackTo = prev
                        requestBlocks(connection, prev, state.rollingCheckpoint, announce.cumulativeDifficulty)
                    } else {
                        break
                    }
                }
            } catch (e: ClosedSendChannelException) {
            } catch (e: TimeoutCancellationException) {
                connection.dos("Fetching cancelled: ${e.message}")
            } catch (e: CancellationException) {
                logger.info { "Fetching cancelled: ${e.message}" }
            } catch (e: Throwable) {
                logger.error(e) { "Exception in processBlocks ${connection.debugName()}" }
                connection.close()
            }

            undoRollback?.let {
                if (undoDifficulty >= state.cumulativeDifficulty) {
                    logger.info { "Reconnecting ${it.size} blocks" }
                    val toRemove = LedgerDB.undoRollbackImpl(rollbackTo!!, it)
                    Kernel.blockDB().deleteImpl(toRemove)
                } else {
                    logger.debug { "Deleting ${it.size} blocks from db" }
                    Kernel.blockDB().deleteImpl(it)
                }
                undoRollback = null
            }
        }

        state = LedgerDB.state()
        if (state.blockHash != originalChain!!) {
            Node.announceChain(state.blockHash, state.cumulativeDifficulty, connection)
            connection.lastBlockTime = connection.lastPacketTime
        }

        if (connection.isClosed())
            logger.info { "Fetched $connectedBlocks blocks from disconnected ${connection.debugName()}" }
        else
            logger.info { "Fetched $connectedBlocks blocks from ${connection.debugName()}" }

        request?.let {
            it.cancel()
            request = null
        }
        syncConnection = null
        connectedBlocks = 0
        originalChain = null
        rollbackTo = null
        undoDifficulty = BigInteger.ZERO
    }

    @Suppress("UNUSED_PARAMETER")
    fun chainFork(connection: Connection, chainFork: ChainFork) {
        if (!connection.requestedBlocks) {
            connection.dos("Unexpected packet ChainFork")
            return
        }

        connection.close()

        if (syncConnection != connection)
            return

        request?.cancel(CancellationException("Fork longer than the rolling checkpoint"))
    }

    suspend fun blocks(connection: Connection, blocks: Blocks) {
        val requestedDifficulty = connection.requestedDifficulty.also {
            connection.requestedDifficulty = BigInteger.ZERO
        }

        if (requestedDifficulty == BigInteger.ZERO) {
            connection.dos("Unexpected packet Blocks")
            return
        }

        if (request == null || syncConnection != connection) {
            deferChannel.send(Triple(connection, blocks, requestedDifficulty))
            announces.trySend(null)
            return
        }

        request!!.complete(blocks)
    }

    private suspend fun processBlocks(connection: Connection, answer: Blocks): Boolean {
        if (rollbackTo != null && undoRollback == null) {
            undoDifficulty = LedgerDB.state().cumulativeDifficulty
            undoRollback = LedgerDB.rollbackToImpl(rollbackTo!!)
            logger.info { "Disconnected ${undoRollback!!.size} blocks" }
        }
        for (i in answer.blocks) {
            val hash = Block.hash(i)
            if (undoRollback?.contains(hash) == true) {
                connection.dos("Rollback contains $hash")
                return false
            }
            val status = Kernel.blockDB().processImpl(hash, i)
            if (status != Accepted) {
                connection.dos(status.toString())
                return false
            }
        }
        if (undoRollback == null)
            LedgerDB.pruneImpl()
        connectedBlocks += answer.blocks.size
        if (answer.blocks.size >= 10)
            logger.info { "Connected ${answer.blocks.size} blocks" }
        return true
    }

    // Blocks were received after timeout. During lags, processing these helps to stay in sync.
    private suspend fun processDeferred(connection: Connection, answer: Blocks) {
        if (answer.blocks.isNotEmpty()) {
            logger.info { "Mongering ${answer.blocks.size} deferred blocks from ${connection.debugName()}" }
            Kernel.blockDB().mutex.withLock {
                for (i in answer.blocks) {
                    val hash = Block.hash(i)
                    val status = try {
                        Kernel.blockDB().processImpl(hash, i)
                    } catch (e: Throwable) {
                        logger.error(e) { "Exception in processDeferred ${connection.debugName()}" }
                        connection.close()
                        return
                    }
                    when (status) {
                        Accepted -> {
                            // Continue catching up
                            logger.info { "Accepted $hash" }
                            continue
                        }
                        is AlreadyHave -> {
                            // Perhaps sequent blocks will be useful
                            logger.debug { "AlreadyHave $hash" }
                            continue
                        }
                        is InFuture, is Invalid -> {
                            // No way
                            connection.dos(status.toString())
                            break
                        }
                        is NotOnThisChain -> {
                            // Ain't useful without fork resolution
                            logger.debug { "NotOnThisChain $hash" }
                            break
                        }
                    }
                }
            }
        } else if (answer.hashes.isNotEmpty()) {
            //XXX Can be used somehow?
            logger.debug { "Skipped ${answer.hashes.size} deferred hashes" }
        } else {
            // Must not happen
            logger.error { "Invalid packet Blocks" }
        }
    }

    private fun requestBlocks(
            connection: Connection,
            hash: Hash,
            checkpoint: Hash,
            difficulty: BigInteger,
    ) {
        request = CompletableDeferred()
        connection.requestedDifficulty = difficulty
        // 緊湊型區塊轉發
        connection.sendPacket(PacketType.GetBlocks, GetBlocks(hash, checkpoint))
    }

    private fun timeout(): Long {
        return if (!PoS.guessInitialSynchronization())
            4 * 1000L
        else
            10 * 1000L
    }
}

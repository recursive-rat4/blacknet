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
import java.util.concurrent.PriorityBlockingQueue
import java.util.concurrent.TimeUnit
import java.util.concurrent.TimeoutException
import kotlin.concurrent.withLock
import kotlinx.coroutines.*
import kotlinx.coroutines.channels.ClosedSendChannelException
import ninja.blacknet.Kernel
import ninja.blacknet.ShutdownHooks
import ninja.blacknet.core.*
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PoS
import ninja.blacknet.db.CoinDB
import ninja.blacknet.network.packet.BlockAnnounce
import ninja.blacknet.network.packet.Blocks
import ninja.blacknet.network.packet.ConsensusFault
import ninja.blacknet.network.packet.GetBlocks
import ninja.blacknet.network.packet.PacketType
import ninja.blacknet.util.rotate

private val logger = KotlinLogging.logger {}

/**
 * 區塊有向無環圖獲取器
 */
object BlockFetcher {
    //TODO review capacity
    private val blocksQueue = PriorityBlockingQueue<BlockSource>()
    @Volatile
    private var request: CompletableFuture<Blocks>? = null
    private var connectedBlocks = 0
    @Volatile
    private var syncConnection: Connection? = null
    private var originalChain: Hash? = null
    private var rollbackTo: Hash? = null
    private var undoDifficulty = BigInteger.ZERO
    private var undoRollback: List<Hash>? = null

    @Volatile
    private var shutdown = false
    private val vThread = rotate("BlockFetcher::implementation", ::implementation)

    init {
        ShutdownHooks.add {
            logger.info { "Interrupting BlockFetcher" }
            shutdown = true
            vThread.interrupt()
        }
    }

    fun join() {
        vThread.join()
        if (!shutdown)
            throw Error("BlockFetcher exited")
    }

    fun isSynchronizing(): Boolean {
        return syncConnection != null
    }

    fun disconnected(connection: Connection) {
        if (syncConnection != connection)
            return

        request?.completeExceptionally(CancellationException("Connection closed"))
    }

    fun offer(connection: Connection, announce: BlockAnnounce) {
        if (announce.cumulativeDifficulty <= CoinDB.state().cumulativeDifficulty)
            return

        blocksQueue.offer(Remote(connection, announce))
    }

    fun stakedBlock(hash: Hash, bytes: ByteArray): Pair<Status, Int> {
        val future = CompletableFuture<Pair<Status, Int>>()
        blocksQueue.offer(Staked(hash, bytes, future))
        request?.completeExceptionally(CancellationException("Staked new block"))
        return future.get()
    }

    private fun implementation() {
        val source = blocksQueue.take()

        if (source is Staked) {
            val status = Kernel.blockDB().reentrant.writeLock().withLock {
                Kernel.blockDB().processImpl(source.hash, source.bytes)
            }
            val n = when (status) {
                Accepted -> Node.announceBlock(source.hash, CoinDB.state().cumulativeDifficulty)
                else -> 0
            }
            source.future.complete(Pair(status, n))
            return
        }

        if (source is Deferred) {
            if (source.requestedDifficulty <= CoinDB.state().cumulativeDifficulty)
                return

            processDeferred(source.connection, source.answer)
            return
        }

        source as Remote
        val connection = source.connection
        val announce = source.announce

        if (connection.requestedBlocks) {
            return
        }

        var state = CoinDB.state()
        if (announce.cumulativeDifficulty <= state.cumulativeDifficulty)
            return

        Kernel.blockDB().reentrant.writeLock().withLock {
            if (Kernel.blockDB().isRejectedImpl(announce.hash)) {
                connection.dos("Rejected block")
                return
            }

            logger.info { "Fetching ${announce.hash}" }
            syncConnection = connection
            originalChain = state.blockHash

            try {
                requestBlocks(connection, state.blockHash, state.rollingCheckpoint, announce.cumulativeDifficulty)

                requestLoop@ while (true) {
                    val answer = request!!.get(timeout(), TimeUnit.MILLISECONDS)
                    if (answer.blocks.isNotEmpty()) {
                        val accepted = processBlocks(connection, answer)
                        if (!accepted)
                            break

                        state = CoinDB.state()

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
                                connection.dos("Rejected block")
                                break@requestLoop
                            }
                            val blockIndex = CoinDB.blockIndexes.get(hash.bytes)
                            if (blockIndex == null)
                                break
                            if (blockIndex.height < state.height - PoS.ROLLBACK_LIMIT) {
                                connection.dos("Rollback to ${blockIndex.height}")
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
            } catch (e: InterruptedException) {
                //XXX stop catching Throwable everywhere
                // rethrow for graceful shutdown
                throw e
            } catch (e: TimeoutException) {
                connection.dos("Fetching cancelled: ${e.message ?: "Request timed out"}")
            } catch (e: CancellationException) {
                logger.info { "Fetching cancelled: ${e.message}" }
            } catch (e: Throwable) {
                logger.error(e) { "Exception in processBlocks ${connection.debugName()}" }
                connection.close()
            }

            undoRollback?.let {
                if (undoDifficulty >= state.cumulativeDifficulty) {
                    logger.info { "Reconnecting ${it.size} blocks" }
                    val toRemove = CoinDB.undoRollbackImpl(rollbackTo!!, it)
                    Kernel.blockDB().deleteImpl(toRemove)
                } else {
                    logger.debug { "Deleting ${it.size} blocks from db" }
                    Kernel.blockDB().deleteImpl(it)
                }
                undoRollback = null
            }
        }

        state = CoinDB.state()
        if (state.blockHash != originalChain!!) {
            Node.announceBlock(state.blockHash, state.cumulativeDifficulty, connection)
            connection.lastBlockTime = connection.lastPacketTime
        }

        if (connection.isClosed())
            logger.info { "Fetched $connectedBlocks blocks from disconnected ${connection.debugName()}" }
        else
            logger.info { "Fetched $connectedBlocks blocks from ${connection.debugName()}" }

        request?.let {
            it.cancel(true)
            request = null
        }
        syncConnection = null
        connectedBlocks = 0
        originalChain = null
        rollbackTo = null
        undoDifficulty = BigInteger.ZERO
    }

    @Suppress("UNUSED_PARAMETER")
    fun consensusFault(connection: Connection, consensusFault: ConsensusFault) {
        if (!connection.requestedBlocks) {
            connection.dos("Unexpected packet ConsensusFault")
            return
        }

        connection.close()

        if (syncConnection != connection)
            return

        request?.completeExceptionally(CancellationException("Dipath longer than the rolling checkpoint"))
    }

    fun blocks(connection: Connection, blocks: Blocks) {
        val requestedDifficulty = connection.requestedDifficulty.also {
            connection.requestedDifficulty = BigInteger.ZERO
        }

        if (requestedDifficulty == BigInteger.ZERO) {
            connection.dos("Unexpected packet Blocks")
            return
        }

        if (request == null || syncConnection != connection) {
            blocksQueue.offer(Deferred(connection, blocks, requestedDifficulty))
            return
        }

        request!!.complete(blocks)
    }

    private fun processBlocks(connection: Connection, answer: Blocks): Boolean {
        if (rollbackTo != null && undoRollback == null) {
            undoDifficulty = CoinDB.state().cumulativeDifficulty
            undoRollback = CoinDB.rollbackToImpl(rollbackTo!!)
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
            CoinDB.pruneImpl()
        connectedBlocks += answer.blocks.size
        if (answer.blocks.size >= 10)
            logger.info { "Connected ${answer.blocks.size} blocks" }
        return true
    }

    // Blocks were received after timeout. During lags, processing these helps to stay in sync.
    private fun processDeferred(connection: Connection, answer: Blocks) {
        if (answer.blocks.isNotEmpty()) {
            logger.info { "Mongering ${answer.blocks.size} deferred blocks from ${connection.debugName()}" }
            Kernel.blockDB().reentrant.writeLock().withLock {
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
                        is NotReachableVertex -> {
                            // 何罵
                            logger.debug { "NotReachableVertex $hash" }
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
        request = CompletableFuture()
        connection.requestedDifficulty = difficulty
        connection.sendPacket(PacketType.GetBlocks, GetBlocks(hash, checkpoint))
    }

    private fun timeout(): Long {
        return if (!PoS.guessInitialSynchronization())
            4 * 1000L
        else
            10 * 1000L
    }
}

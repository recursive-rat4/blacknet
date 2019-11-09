/*
 * Copyright (c) 2018-2019 Pavel Vasin
 * Copyright (c) 2019 Blacknet Team
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.network

import kotlinx.coroutines.*
import kotlinx.coroutines.channels.Channel
import kotlinx.coroutines.channels.ClosedSendChannelException
import mu.KotlinLogging
import ninja.blacknet.Runtime
import ninja.blacknet.core.*
import ninja.blacknet.crypto.BigInt
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PoS
import ninja.blacknet.db.BlockDB
import ninja.blacknet.db.LedgerDB
import ninja.blacknet.packet.Blocks
import ninja.blacknet.packet.GetBlocks
import ninja.blacknet.util.withTimeout

private val logger = KotlinLogging.logger {}

object ChainFetcher {
    private val chains = Channel<Triple<Connection, Hash, BigInt>?>(16)
    private val recvChannel = Channel<Blocks>(Channel.RENDEZVOUS)
    @Volatile
    private var request: Deferred<Blocks>? = null
    private var connectedBlocks = 0
    @Volatile
    private var stakedBlock: Triple<Hash, ByteArray, CompletableDeferred<Pair<Status, Int>>>? = null
    @Volatile
    private var syncConnection: Connection? = null
    private var originalChain: Hash? = null
    private var rollbackTo: Hash? = null
    private var undoDifficulty = BigInt.ZERO
    private var undoRollback: List<Hash>? = null

    init {
        Runtime.launch { fetcher() }
    }

    fun isSynchronizing(): Boolean {
        return syncConnection != null
    }

    fun disconnected(connection: Connection) {
        if (syncConnection == connection)
            request?.cancel(CancellationException("Connection closed"))
    }

    fun offer(connection: Connection, chain: Hash, cumulativeDifficulty: BigInt) {
        if (cumulativeDifficulty <= LedgerDB.cumulativeDifficulty())
            return

        chains.offer(Triple(connection, chain, cumulativeDifficulty))
    }

    suspend fun stakedBlock(hash: Hash, bytes: ByteArray): Pair<Status, Int> {
        val deferred = CompletableDeferred<Pair<Status, Int>>()
        stakedBlock = Triple(hash, bytes, deferred)
        request?.cancel(CancellationException("Staked new block"))
        chains.offer(null)
        return deferred.await()
    }

    private suspend fun fetcher() {
        while (true) {
            val data = chains.receive()

            stakedBlock?.let { (hash, bytes, deferred) ->
                val status = BlockDB.process(hash, bytes)
                val n = when (status) {
                    Accepted -> Node.announceChain(hash, LedgerDB.cumulativeDifficulty())
                    else -> 0
                }

                stakedBlock = null
                deferred.complete(Pair(status, n))
            }

            if (data == null)
                continue

            val (connection, chain, cumulativeDifficulty) = data

            if (cumulativeDifficulty <= LedgerDB.cumulativeDifficulty())
                continue
            if (BlockDB.isRejected(chain)) {
                connection.dos("Rejected chain")
                continue
            }

            logger.info("Fetching $chain")
            syncConnection = connection
            originalChain = LedgerDB.blockHash()

            try {
                requestBlocks(connection, originalChain!!, LedgerDB.rollingCheckpoint())

                requestLoop@ while (true) {
                    val answer = withTimeout(timeout()) {
                        request!!.await()
                    }
                    if (answer.isEmpty()) {
                        break
                    }
                    if (!answer.hashes.isEmpty()) {
                        if (rollbackTo != null) {
                            connection.dos("Unexpected rollback")
                            break
                        }
                        val height = LedgerDB.height()
                        val checkpoint = LedgerDB.rollingCheckpoint()
                        var prev = checkpoint
                        for (hash in answer.hashes) {
                            if (BlockDB.isRejected(hash)) {
                                connection.dos("Rejected chain")
                                break@requestLoop
                            }
                            val chainIndex = LedgerDB.getChainIndex(hash)
                            if (chainIndex == null)
                                break
                            if (chainIndex.height < height - PoS.MATURITY) {
                                connection.dos("Rollback to ${chainIndex.height}")
                                break@requestLoop
                            }
                            prev = hash
                        }
                        rollbackTo = prev
                        requestBlocks(connection, prev, checkpoint)
                        continue
                    }

                    val accepted = processBlocks(connection, answer)
                    if (!accepted)
                        break

                    if (chain == LedgerDB.blockHash()) {
                        break
                    } else {
                        if (cumulativeDifficulty > LedgerDB.cumulativeDifficulty()) {
                            requestBlocks(connection, LedgerDB.blockHash(), LedgerDB.rollingCheckpoint())
                            continue
                        } else {
                            break
                        }
                    }
                }
            } catch (e: ClosedSendChannelException) {
            } catch (e: CancellationException) {
                logger.info("Fetching cancelled: ${e.message}")
            } catch (e: Throwable) {
                logger.error("Exception in processBlocks ${connection.debugName()}", e)
                connection.close()
            }

            fetched()
        }
    }

    private suspend fun fetched() {
        val request = request
        val connection = syncConnection!!
        val undoRollback = undoRollback

        val cumulativeDifficulty = LedgerDB.cumulativeDifficulty()
        if (undoRollback != null) {
            if (undoDifficulty >= cumulativeDifficulty) {
                logger.info("Reconnecting ${undoRollback.size} blocks")
                val toRemove = LedgerDB.undoRollback(rollbackTo!!, undoRollback)
                BlockDB.remove(toRemove)
            } else {
                logger.debug { "Removing ${undoRollback.size} blocks from db" }
                BlockDB.remove(undoRollback)
            }
            this.undoRollback = null
        }

        val newChain = LedgerDB.blockHash()
        if (newChain != originalChain) {
            Node.announceChain(newChain, cumulativeDifficulty, connection)
            connection.lastBlockTime = Runtime.time()
        }

        if (connection.isClosed())
            logger.info("${connection.debugName(true)} disconnected. Fetched $connectedBlocks blocks")
        else
            logger.info("Finished fetching $connectedBlocks blocks from ${connection.debugName()}")

        recvChannel.poll()
        if (request != null) {
            request.cancel()
            this.request = null
        }
        syncConnection = null
        connectedBlocks = 0
        originalChain = null
        rollbackTo = null
        undoDifficulty = BigInt.ZERO
    }

    fun chainFork(connection: Connection) {
        val request = request

        if (request == null || syncConnection != connection) {
            connection.dos("Unexpected chain fork message")
            return
        }

        request.cancel(CancellationException("Fork longer than the rolling checkpoint"))
    }

    suspend fun blocks(connection: Connection, blocks: Blocks) {
        if (request == null || syncConnection != connection) {
            // request may be cancelled
            //logger.info("Unexpected synchronization ${connection.debugName()}")
            //connection.close()
            return
        }
        recvChannel.send(blocks)
    }

    private suspend fun processBlocks(connection: Connection, answer: Blocks): Boolean {
        if (rollbackTo != null && undoRollback == null) {
            undoDifficulty = LedgerDB.cumulativeDifficulty()
            undoRollback = LedgerDB.rollbackTo(rollbackTo!!)
            logger.info("Disconnected ${undoRollback!!.size} blocks")
        }
        for (i in answer.blocks) {
            val hash = Block.Hasher(i.array)
            if (undoRollback?.contains(hash) == true) {
                connection.dos("Rollback contains $hash")
                return false
            }
            val status = BlockDB.process(hash, i.array)
            if (status != Accepted) {
                connection.dos(status.toString())
                return false
            }
        }
        if (undoRollback == null)
            LedgerDB.prune()
        connectedBlocks += answer.blocks.size
        if (answer.blocks.size >= 10)
            logger.info("Connected ${answer.blocks.size} blocks")
        return true
    }

    private fun requestBlocks(connection: Connection, hash: Hash, checkpoint: Hash) {
        request = Runtime.async { recvChannel.receive() }
        connection.sendPacket(GetBlocks(hash, checkpoint))
    }

    private fun timeout(): Int {
        return if (!PoS.guessInitialSynchronization()) 5 else 10
    }
}

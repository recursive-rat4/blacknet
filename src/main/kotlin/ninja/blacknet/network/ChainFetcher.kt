/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.network

import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.TimeoutCancellationException
import kotlinx.coroutines.channels.Channel
import kotlinx.coroutines.launch
import mu.KotlinLogging
import ninja.blacknet.core.Block
import ninja.blacknet.core.DataDB.Status
import ninja.blacknet.crypto.BigInt
import ninja.blacknet.crypto.Hash
import ninja.blacknet.db.BlockDB
import ninja.blacknet.db.LedgerDB
import ninja.blacknet.util.withTimeout
import kotlin.coroutines.CoroutineContext

private val logger = KotlinLogging.logger {}

object ChainFetcher : CoroutineScope {
    override val coroutineContext: CoroutineContext = Dispatchers.Default
    const val TIMEOUT = 5
    private val chains = Channel<ChainData>(16)
    private val recvChannel = Channel<Blocks>(Channel.RENDEZVOUS)
    private var connectedBlocks = 0
    @Volatile
    private var disconnected: ChainData? = null
    @Volatile
    private var syncChain: ChainData? = null
    private var originalChain: Hash? = null
    private var rollbackTo: Hash? = null
    private var undoDifficulty = BigInt.ZERO
    private var undoRollback: ArrayList<Hash>? = null

    init {
        launch { fetcher() }
    }

    fun isSynchronizing(): Boolean {
        return syncChain != null
    }

    fun disconnected(connection: Connection) {
        if (syncChain?.connection == connection) {
            disconnected = syncChain
        }
    }

    fun offer(connection: Connection, chain: Hash, cumulativeDifficulty: BigInt? = null) {
        if (chain == Hash.ZERO)
            return
        if (cumulativeDifficulty != null && cumulativeDifficulty <= LedgerDB.cumulativeDifficulty())
            return

        chains.offer(ChainData(connection, chain, cumulativeDifficulty))
    }

    private suspend fun fetcher() {
        while (true) {
            if (disconnected == syncChain)
                fetched()
            else
                disconnected = null

            val data = chains.receive()

            if (data.connection.isClosed())
                continue
            if (data.cumulativeDifficulty() != BigInt.ZERO && data.cumulativeDifficulty() <= LedgerDB.cumulativeDifficulty())
                continue
            if (!BlockDB.isInteresting(data.chain))
                continue

            logger.info("Fetching ${data.chain} from ${data.connection.remoteAddress}")
            syncChain = data
            originalChain = LedgerDB.blockHash()
            data.connection.sendPacket(GetBlocks(originalChain!!, LedgerDB.getRollingCheckpoint()))

            try {
                requestLoop@ while (true) {
                    val answer = withTimeout(TIMEOUT) { recvChannel.receive() }
                    if (answer.isEmpty()) {
                        fetched()
                        break
                    }
                    if (!answer.hashes.isEmpty()) {
                        if (rollbackTo != null) {
                            logger.info("Unexpected rollback")
                            data.connection.close()
                            break
                        }
                        val checkpoint = LedgerDB.getRollingCheckpoint()
                        var prev = checkpoint
                        for (hash in answer.hashes) {
                            if (BlockDB.isRejected(hash)) {
                                data.connection.dos("invalid chain")
                                fetched()
                                break@requestLoop
                            }
                            if (LedgerDB.getBlockNumber(hash) == null)
                                break
                            prev = hash
                        }
                        rollbackTo = prev
                        data.connection.sendPacket(GetBlocks(prev, checkpoint))
                        continue
                    }
                    if (!processBlocks(data.connection, answer)) {
                        fetched()
                        break
                    }

                    if (data.chain == LedgerDB.blockHash()) {
                        fetched()
                        break
                    } else {
                        if (data.cumulativeDifficulty() == BigInt.ZERO
                                || data.cumulativeDifficulty() > LedgerDB.cumulativeDifficulty()) {
                            requestBlocks(data.connection)
                        } else {
                            fetched()
                            break
                        }
                    }
                }
            } catch (e: TimeoutCancellationException) {
                logger.info("${e.message}")
                data.connection.close()
            } catch (e: Throwable) {
                logger.error("Exception in processBlocks ${data.connection.remoteAddress}", e)
                data.connection.close()
            }
        }
    }

    private suspend fun fetched() {
        val cumulativeDifficulty = LedgerDB.cumulativeDifficulty()
        if (undoRollback != null) {
            if (undoDifficulty >= cumulativeDifficulty) {
                logger.info("Reconnecting ${undoRollback!!.size} blocks")
                val toRemove = LedgerDB.undoRollback(rollbackTo!!, undoRollback!!)
                LedgerDB.commit()
                launch { BlockDB.remove(toRemove) }
            } else {
                logger.info("Removing ${undoRollback!!.size} blocks from db")
                val toRemove = undoRollback!!
                launch { BlockDB.remove(toRemove) }
            }
        }
        if (syncChain != null) {
            val newChain = LedgerDB.blockHash()
            if (newChain != originalChain) {
                Node.announceChain(newChain, cumulativeDifficulty, syncChain!!.connection)
                LedgerDB.prune()
            }

            if (syncChain == disconnected)
                logger.info("Peer disconnected. Fetched $connectedBlocks blocks")
            else
                logger.info("Finished fetching $connectedBlocks blocks")
        }

        syncChain = null
        disconnected = null
        connectedBlocks = 0
        originalChain = null
        rollbackTo = null
        undoRollback = null
        undoDifficulty = BigInt.ZERO
    }

    suspend fun fetched(connection: Connection, answer: Blocks) {
        if (syncChain == null || syncChain!!.connection != connection) {
            logger.info("Unexpected synchronization. Disconnecting ${connection.remoteAddress}")
            connection.close()
            return
        }
        if (syncChain != disconnected)
            recvChannel.send(answer)
    }

    private suspend fun processBlocks(connection: Connection, answer: Blocks): Boolean {
        if (rollbackTo != null && undoRollback == null) {
            undoDifficulty = LedgerDB.cumulativeDifficulty()
            undoRollback = LedgerDB.rollbackTo(rollbackTo!!)
            logger.info("Disconnected ${undoRollback!!.size} blocks")
            LedgerDB.commit()
        }
        for (i in answer.blocks) {
            val hash = Block.Hasher(i.array)
            if (undoRollback?.contains(hash) == true) {
                logger.info("Rollback contains $hash")
                connection.close()
                return false
            }
            val status = BlockDB.process(hash, i.array, null)
            if (status == Status.IN_FUTURE) {
                return false
            } else if (status != Status.ACCEPTED && status != Status.ALREADY_HAVE) {
                logger.info("$status block $hash")
                connection.close()
                return false
            }
        }
        connection.lastBlockTime = Node.time()
        connectedBlocks += answer.blocks.size
        if (answer.blocks.size >= 10)
            logger.info("Connected ${answer.blocks.size} blocks")
        return true
    }

    private fun requestBlocks(connection: Connection) {
        connection.sendPacket(GetBlocks(LedgerDB.blockHash(), LedgerDB.getRollingCheckpoint()))
    }

    private class ChainData(val connection: Connection, val chain: Hash, val cumulativeDifficulty: BigInt?) {
        fun cumulativeDifficulty() = cumulativeDifficulty ?: BigInt.ZERO
    }
}

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
import kotlinx.coroutines.launch
import mu.KotlinLogging
import ninja.blacknet.core.Block
import ninja.blacknet.core.DataDB.Status
import ninja.blacknet.core.DataType
import ninja.blacknet.crypto.BigInt
import ninja.blacknet.crypto.Hash
import ninja.blacknet.db.BlockDB
import ninja.blacknet.db.LedgerDB
import ninja.blacknet.serialization.SerializableByteArray
import ninja.blacknet.util.SynchronizedArrayList
import ninja.blacknet.util.delay
import kotlin.coroutines.CoroutineContext

private val logger = KotlinLogging.logger {}

object ChainFetcher : CoroutineScope {
    override val coroutineContext: CoroutineContext = Dispatchers.Default
    const val TIMEOUT = 5
    private val chains = SynchronizedArrayList<ChainData>()
    private var requestTime = 0L
    @Volatile
    private var disconnected: ChainData? = null
    @Volatile
    private var syncChain: ChainData? = null
    private var rollbackTo: Hash? = null
    private var undoDifficulty = BigInt.ZERO
    private var undoRollback: ArrayList<Hash>? = null

    init {
        launch { fetcher() }
    }

    fun isSynchronizing(): Boolean {
        return syncChain != null
    }

    fun disconnected(connection: Connection) = launch {
        chains.removeIf { it.connection == connection }
        if (syncChain?.connection == connection) {
            disconnected = syncChain
        }
    }

    suspend fun offer(connection: Connection, chain: Hash, cumulativeDifficulty: BigInt? = null) {
        chains.add(ChainData(connection, chain, cumulativeDifficulty))
    }

    private suspend fun fetcher() {
        while (true) {
            if (disconnected == syncChain)
                fetched()
            else
                disconnected = null

            if (syncChain != null) {
                if (Node.time() <= requestTime + Node.NETWORK_TIMEOUT) {
                    delay(TIMEOUT)
                    continue
                }

                logger.info("Disconnecting on timeout ${syncChain!!.connection.remoteAddress}")
                syncChain!!.connection.close()
                fetched()
            }

            val data = selectChain()
            if (data == null) {
                delay(TIMEOUT)
                continue
            }

            if (data.cumulativeDifficulty() != BigInt.ZERO && data.cumulativeDifficulty() <= LedgerDB.cumulativeDifficulty())
                continue
            if (!BlockDB.isInteresting(data.chain))
                continue

            logger.info("Fetching ${data.chain} from ${data.connection.remoteAddress}")
            requestTime = Node.time()
            syncChain = data
            data.connection.sendPacket(GetBlocks(LedgerDB.blockHash(), LedgerDB.getRollingCheckpoint()))
        }
    }

    private suspend fun selectChain(): ChainData? {
        val chain = chains.maxBy { it.cumulativeDifficulty() } ?: return null
        chains.remove(chain)
        return chain
    }

    private suspend fun fetched() {
        if (undoRollback != null) {
            if (undoDifficulty >= LedgerDB.cumulativeDifficulty()) {
                logger.info("Reconnecting ${undoRollback!!.size} blocks")
                LedgerDB.undoRollback(rollbackTo!!, undoRollback!!)
                LedgerDB.commit()
            } else {
                logger.info("Removing ${undoRollback!!.size} blocks from db")
                val toRemove = undoRollback!!
                launch { BlockDB.remove(toRemove) }
                //announce new chain after reorganization
                Node.broadcastInv(arrayListOf(Pair(DataType.Block, LedgerDB.blockHash())), syncChain!!.connection)
            }
        }
        if (syncChain != null) {
            if (syncChain == disconnected)
                logger.info("Peer disconnected")
            else
                logger.info("Finished fetching")
        }

        syncChain = null
        disconnected = null
        rollbackTo = null
        undoRollback = null
        undoDifficulty = BigInt.ZERO
    }

    suspend fun fetched(connection: Connection, hashes: ArrayList<Hash>, blocks: ArrayList<SerializableByteArray>) {
        if (syncChain == null || syncChain!!.connection != connection) {
            logger.info("Unexpected synchronization. Disconnecting ${connection.remoteAddress}")
            connection.close()
            return
        }
        if (!hashes.isEmpty()) {
            if (rollbackTo != null) {
                logger.info("Unexpected rollback")
                connection.close()
                fetched()
                return
            }
            val checkpoint = LedgerDB.getRollingCheckpoint()
            var prev = checkpoint
            for (hash in hashes) {
                if (LedgerDB.getBlockNumber(hash) == null)
                    break
                prev = hash
            }
            requestTime = Node.time()
            rollbackTo = prev
            connection.sendPacket(GetBlocks(prev, checkpoint))
            return
        }
        if (blocks.isEmpty()) {
            logger.info("No blocks. Disconnecting ${connection.remoteAddress}")
            connection.close()
            fetched()
            return
        }
        if (rollbackTo != null && undoRollback == null) {
            undoDifficulty = LedgerDB.cumulativeDifficulty()
            undoRollback = LedgerDB.rollbackTo(rollbackTo!!)
            logger.info("Disconnected ${undoRollback!!.size} blocks")
            LedgerDB.commit()
        }
        for (i in blocks) {
            // prevent slow nodes to Disconnect on timeout while verifying many blocks
            requestTime = Node.time()

            val hash = Block.Hasher(i.array)
            if (undoRollback?.contains(hash) == true) {
                logger.info("Rollback contains $hash")
                connection.close()
                fetched()
                return
            }
            val status = BlockDB.process(hash, i.array, null)
            if (status != Status.ACCEPTED) {
                logger.info("$status block $hash")
                connection.close()
                fetched()
                return
            }
        }
        logger.info("Connected ${blocks.size} blocks")
        if (syncChain!!.chain == LedgerDB.blockHash()) {
            fetched()
        } else {
            if (syncChain!!.cumulativeDifficulty() == BigInt.ZERO
                    || syncChain!!.cumulativeDifficulty() > LedgerDB.cumulativeDifficulty())
                requestBlocks()
            else
                fetched()
        }
    }

    private fun requestBlocks() {
        requestTime = Node.time()
        syncChain!!.connection.sendPacket(GetBlocks(LedgerDB.blockHash(), LedgerDB.getRollingCheckpoint()))
    }

    private class ChainData(val connection: Connection, val chain: Hash, val cumulativeDifficulty: BigInt?) {
        fun cumulativeDifficulty() = cumulativeDifficulty ?: BigInt.ZERO
    }
}

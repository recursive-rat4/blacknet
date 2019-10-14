/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.db

import kotlinx.coroutines.sync.withLock
import mu.KotlinLogging
import ninja.blacknet.Config
import ninja.blacknet.api.APIServer
import ninja.blacknet.core.Block
import ninja.blacknet.core.DataDB
import ninja.blacknet.core.TxPool
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PoS
import ninja.blacknet.network.Connection
import ninja.blacknet.util.emptyByteArray

private val logger = KotlinLogging.logger {}

object BlockDB : DataDB() {
    private const val MIN_DISK_SPACE = LedgerDB.MAX_BLOCK_SIZE * 2L
    private val BLOCK_KEY = "block".toByteArray()
    private var cachedBlockHash = Hash.ZERO
    private var cachedBlockBytes = emptyByteArray()

    suspend fun block(hash: Hash): Pair<Block, Int>? = mutex.withLock {
        return@withLock blockImpl(hash)
    }

    suspend fun blockImpl(hash: Hash): Pair<Block, Int>? {
        val bytes = getImpl(hash) ?: return null
        val block = Block.deserialize(bytes)
        return Pair(block, bytes.size)
    }

    suspend fun remove(list: ArrayList<Hash>) = mutex.withLock {
        val txDb = LevelDB.createWriteBatch()
        list.forEach {
            txDb.delete(BLOCK_KEY, it.bytes)
        }
        txDb.write()
    }

    override suspend fun containsImpl(hash: Hash): Boolean {
        return LedgerDB.chainContains(hash)
    }

    override suspend fun getImpl(hash: Hash): ByteArray? {
        return if (hash == cachedBlockHash)
            cachedBlockBytes
        else
            LevelDB.get(BLOCK_KEY, hash.bytes)
    }

    override suspend fun processImpl(hash: Hash, bytes: ByteArray, connection: Connection?): Status {
        val block = Block.deserialize(bytes)
        if (block.version.toUInt() > Block.VERSION.toUInt()) {
            val percent = 100 * LedgerDB.upgraded() / PoS.MATURITY
            if (percent > 9)
                logger.info("$percent% of blocks upgraded")
            else
                logger.info("unknown version ${block.version.toUInt()}")
        }
        if (PoS.isTooFarInFuture(block.time)) {
            logger.debug { "too far in future ${block.time}" }
            return Status.IN_FUTURE
        }
        if (!block.verifyContentHash(bytes)) {
            logger.info("invalid content hash")
            return Status.INVALID
        }
        if (!block.verifySignature(hash)) {
            logger.info("invalid signature")
            return Status.INVALID
        }
        if (block.previous != LedgerDB.blockHash()) {
            logger.info("block $hash not on current chain prev ${block.previous}")
            return Status.NOT_ON_THIS_CHAIN
        }
        val batch = LevelDB.createWriteBatch()
        val txDb = LedgerDB.Update(batch, block.version, hash, block.previous, block.time, bytes.size, block.generator)
        val txHashes = LedgerDB.processBlockImpl(txDb, hash, block, bytes.size)
        if (txHashes != null) {
            batch.put(BLOCK_KEY, hash.bytes, bytes)
            txDb.commitImpl()
            TxPool.mutex.withLock {
                TxPool.clearRejectsImpl()
                TxPool.removeImpl(txHashes)
            }
            APIServer.blockNotify(block, hash, LedgerDB.height(), bytes.size)
            cachedBlockHash = hash
            cachedBlockBytes = bytes
            return Status.ACCEPTED
        } else {
            batch.close()
            return Status.INVALID
        }
    }

    fun warnings(): List<String> {
        if (Config.dataDir.getUsableSpace() < MIN_DISK_SPACE)
            return listOf("Disk space is low!")

        return emptyList()
    }
}

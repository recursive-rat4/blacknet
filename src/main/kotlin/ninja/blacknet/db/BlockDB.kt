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
import ninja.blacknet.core.*
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PoS
import ninja.blacknet.network.Connection

private val logger = KotlinLogging.logger {}

object BlockDB : DataDB() {
    private const val MIN_DISK_SPACE = LedgerDB.MAX_BLOCK_SIZE * 2L
    private val BLOCK_KEY = "block".toByteArray()
    @Volatile
    internal var cachedBlock: Pair<Hash, ByteArray>? = null

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
        return LevelDB.get(BLOCK_KEY, hash.bytes)
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
            return InFuture
        }
        if (!block.verifyContentHash(bytes)) {
            return Invalid("Invalid content hash")
        }
        if (!block.verifySignature(hash)) {
            return Invalid("Invalid signature")
        }
        if (block.previous != LedgerDB.blockHash()) {
            return NotOnThisChain
        }
        val batch = LevelDB.createWriteBatch()
        val txDb = LedgerDB.Update(batch, block.version, hash, block.previous, block.time, bytes.size, block.generator)
        val status = LedgerDB.processBlockImpl(txDb, hash, block, bytes.size)
        if (status == Accepted) {
            batch.put(BLOCK_KEY, hash.bytes, bytes)
            txDb.commitImpl()
            APIServer.blockNotify(block, hash, LedgerDB.height(), bytes.size)
            cachedBlock = Pair(block.previous, bytes)
        } else {
            batch.close()
        }
        return status
    }

    fun warnings(): List<String> {
        if (Config.dataDir.getUsableSpace() < MIN_DISK_SPACE)
            return listOf("Disk space is low!")

        return emptyList()
    }
}

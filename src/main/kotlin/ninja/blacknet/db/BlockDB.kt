/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.db

import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock
import mu.KotlinLogging
import ninja.blacknet.Config
import ninja.blacknet.api.APIServer
import ninja.blacknet.core.*
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PoS
import ninja.blacknet.db.LedgerDB.forkV2
import java.util.Collections

private val logger = KotlinLogging.logger {}

object BlockDB {
    private const val MIN_DISK_SPACE = PoS.MAX_BLOCK_SIZE * 2L
    internal val mutex = Mutex()
    private val BLOCK_KEY = DBKey(0xC0.toByte(), Hash.SIZE)
    @Volatile
    internal var cachedBlock: Pair<Hash, ByteArray>? = null

    private val rejects = Collections.newSetFromMap(object : LinkedHashMap<Hash, Boolean>() {
        override fun removeEldestEntry(eldest: MutableMap.MutableEntry<Hash, Boolean>): Boolean {
            return size > PoS.MATURITY
        }
    })

    internal fun isRejectedImpl(hash: Hash): Boolean {
        return rejects.contains(hash)
    }

    suspend fun block(hash: Hash): Pair<Block, Int>? = mutex.withLock {
        return@withLock blockImpl(hash)
    }

    internal fun blockImpl(hash: Hash): Pair<Block, Int>? {
        val bytes = getImpl(hash) ?: return null
        val block = Block.deserialize(bytes)
        return Pair(block, bytes.size)
    }

    suspend fun get(hash: Hash): ByteArray? = mutex.withLock {
        return@withLock getImpl(hash)
    }

    internal fun removeImpl(list: List<Hash>) {
        val batch = LevelDB.createWriteBatch()
        list.forEach { hash ->
            batch.delete(BLOCK_KEY, hash.bytes)
        }
        batch.write()
    }

    private fun containsImpl(hash: Hash): Boolean {
        return LedgerDB.chainContains(hash)
    }

    internal fun getImpl(hash: Hash): ByteArray? {
        return LevelDB.get(BLOCK_KEY, hash.bytes)
    }

    private suspend fun processBlockImpl(hash: Hash, bytes: ByteArray): Status {
        val block = Block.deserialize(bytes)
        val state = LedgerDB.state()
        if (block.version.toUInt() > Block.VERSION.toUInt()) {
            val percent = 100 * state.upgraded / PoS.MATURITY
            if (percent > 9)
                logger.info("$percent% upgraded to unknown version")
            else
                logger.info("Unknown version ${block.version.toUInt()}")
        }
        if (forkV2()) {
            if (block.version.toUInt() < 2.toUInt()) {
                return Invalid("Block version ${block.version.toUInt()} is no longer accepted")
            }
        } else {
            val percent = 100 * state.forkV2 / PoS.MATURITY
            if (percent > 9)
                logger.info("$percent% upgraded to fork v2")
        }
        if (PoS.isTooFarInFuture(block.time)) {
            return InFuture(block.time.toString())
        }
        if (!block.verifyContentHash(bytes)) {
            return Invalid("Invalid content hash")
        }
        if (!block.verifySignature(hash)) {
            return Invalid("Invalid signature")
        }
        if (block.previous != state.blockHash) {
            return NotOnThisChain(block.previous.toString())
        }
        val batch = LevelDB.createWriteBatch()
        val txDb = LedgerDB.Update(batch, block.version, hash, block.previous, block.time, bytes.size, block.generator)
        val (status, txHashes) = LedgerDB.processBlockImpl(txDb, hash, block, bytes.size)
        if (status == Accepted) {
            batch.put(BLOCK_KEY, hash.bytes, bytes)
            txDb.commitImpl()
            TxPool.mutex.withLock {
                TxPool.clearRejectsImpl()
                TxPool.removeImpl(txHashes)
            }
            APIServer.blockNotify(block, hash, state.height + 1, bytes.size)
            cachedBlock = Pair(block.previous, bytes)
        } else {
            batch.close()
        }
        return status
    }

    internal suspend fun processImpl(hash: Hash, bytes: ByteArray): Status {
        if (rejects.contains(hash))
            return Invalid("Already rejected block")
        if (containsImpl(hash))
            return AlreadyHave(hash.toString())
        val status = processBlockImpl(hash, bytes)
        if (status is Invalid)
            rejects.add(hash)
        return status
    }

    fun warnings(): List<String> {
        if (Config.dataDir.getUsableSpace() < MIN_DISK_SPACE)
            return listOf("Disk space is low!")

        return emptyList()
    }
}

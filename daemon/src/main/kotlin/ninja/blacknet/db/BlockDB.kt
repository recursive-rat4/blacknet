/*
 * Copyright (c) 2018-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.db

import io.github.oshai.kotlinlogging.KotlinLogging
import java.util.Collections
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock
import ninja.blacknet.core.*
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.HashSerializer
import ninja.blacknet.crypto.PoS
import ninja.blacknet.dataDir
import ninja.blacknet.db.LedgerDB.forkV2
import ninja.blacknet.rpc.RPCServer
import ninja.blacknet.serialization.bbf.binaryFormat

private val logger = KotlinLogging.logger {}

object BlockDB {
    private const val MIN_DISK_SPACE = PoS.MAX_BLOCK_SIZE * 2L
    internal val mutex = Mutex()
    private val BLOCK_KEY = DBKey(0xC0.toByte(), HashSerializer.SIZE_BYTES)
    @Volatile
    internal var cachedBlock: Pair<ByteArray, ByteArray>? = null

    val blocks = DBView(LevelDB, BLOCK_KEY, Block.serializer(), binaryFormat)

    private val rejects = Collections.newSetFromMap(object : LinkedHashMap<Hash, Boolean>() {
        override fun removeEldestEntry(eldest: Map.Entry<Hash, Boolean>): Boolean {
            return size > PoS.ROLLBACK_LIMIT
        }
    })

    internal fun isRejectedImpl(hash: ByteArray): Boolean {
        return rejects.contains(Hash(hash))
    }

    internal fun removeImpl(list: List<ByteArray>) {
        val batch = LevelDB.createWriteBatch()
        list.forEach { hash ->
            batch.delete(BLOCK_KEY, hash)
        }
        batch.write()
    }

    private fun containsImpl(hash: ByteArray): Boolean {
        return LedgerDB.chainIndexes.contains(hash)
    }

    private suspend fun processBlockImpl(hash: ByteArray, bytes: ByteArray): Status {
        val block = binaryFormat.decodeFromByteArray(Block.serializer(), bytes)
        val state = LedgerDB.state()
        if (block.version > Block.VERSION) {
            val percent = 100 * state.upgraded / PoS.UPGRADE_THRESHOLD
            if (percent > 9)
                logger.info { "$percent% upgraded to unknown version" }
            else
                logger.info { "Unknown version ${block.version}" }
        }
        if (forkV2()) {
            if (block.version < 2u) {
                return Invalid("Block version ${block.version} is no longer accepted")
            }
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
        if (!block.previous.contentEquals(state.blockHash)) {
            return NotOnThisChain(HashSerializer.encode(block.previous))
        }
        val batch = LevelDB.createWriteBatch()
        val txDb = LedgerDB.Update(batch, block.version, hash, block.previous, block.time, bytes.size, block.generator)
        val (status, txHashes) = LedgerDB.processBlockImpl(txDb, hash, block, bytes.size)
        if (status == Accepted) {
            batch.put(BLOCK_KEY, hash, bytes)
            txDb.commitImpl()
            TxPool.mutex.withLock {
                TxPool.clearRejectsImpl()
                TxPool.removeImpl(txHashes)
            }
            RPCServer.blockNotify(block, hash, state.height + 1, bytes.size)
            cachedBlock = Pair(block.previous, bytes)
        } else {
            batch.close()
        }
        return status
    }

    internal suspend fun processImpl(hash: ByteArray, bytes: ByteArray): Status {
        if (rejects.contains(Hash(hash)))
            return Invalid("Already rejected block")
        if (containsImpl(hash))
            return AlreadyHave(HashSerializer.encode(hash))
        val status = processBlockImpl(hash, bytes)
        if (status is Invalid)
            rejects.add(Hash(hash))
        return status
    }

    fun warnings(): List<String> {
        return if (dataDir.getUsableSpace() > MIN_DISK_SPACE)
            emptyList()
        else
            listOf("Disk space is low!")
    }
}

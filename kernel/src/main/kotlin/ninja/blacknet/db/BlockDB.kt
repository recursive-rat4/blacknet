/*
 * Copyright (c) 2018-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.db

import io.github.oshai.kotlinlogging.KotlinLogging
import java.nio.file.Files
import java.util.Collections
import java.util.concurrent.locks.ReentrantReadWriteLock
import kotlin.concurrent.withLock
import kotlinx.serialization.Serializable
import ninja.blacknet.core.*
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PoS
import ninja.blacknet.dataDir
import ninja.blacknet.db.LedgerDB.forkV2
import ninja.blacknet.serialization.bbf.binaryFormat
import ninja.blacknet.signal.Signal5

private val logger = KotlinLogging.logger {}
private const val MIN_DISK_SPACE = PoS.MAX_BLOCK_SIZE * 2L

class BlockDB(
    private val store: KeyValueStore,
) {
    internal val reentrant = ReentrantReadWriteLock()
    private val BLOCK_KEY = DBKey(0xC0.toByte(), Hash.SIZE_BYTES)
    @Volatile
    internal var cachedBlock: Pair<Hash, ByteArray>? = null

    val blocks = DBView(store, BLOCK_KEY, Block.serializer(), binaryFormat)

    val blockNotify = Signal5<Block, Hash, Int, Int, List<Hash>>()

    private val rejects = Collections.newSetFromMap(object : LinkedHashMap<Hash, Boolean>() {
        override fun removeEldestEntry(eldest: Map.Entry<Hash, Boolean>): Boolean {
            return size > PoS.ROLLBACK_LIMIT
        }
    })

    private val fs = Files.getFileStore(dataDir)

    internal fun isRejectedImpl(hash: Hash): Boolean {
        return rejects.contains(hash)
    }

    internal fun deleteImpl(list: List<Hash>) {
        val batch = LevelDB.createWriteBatch()
        list.forEach { hash ->
            batch.delete(BLOCK_KEY, hash.bytes)
        }
        batch.write()
    }

    private fun containsImpl(hash: Hash): Boolean {
        return LedgerDB.chainIndexes.contains(hash.bytes)
    }

    private suspend fun processBlockImpl(hash: Hash, bytes: ByteArray): Status {
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
        if (block.previous != state.blockHash) {
            return NotOnThisChain(block.previous.toString())
        }
        val batch = LevelDB.createWriteBatch()
        val txDb = LedgerDB.Update(batch, block.version, hash, block.previous, block.time, bytes.size, block.generator)
        val (status, txHashes) = LedgerDB.processBlockImpl(txDb, hash, block, bytes.size)
        if (status == Accepted) {
            batch.put(BLOCK_KEY, hash.bytes, bytes)
            txDb.commitImpl()
            blockNotify(block, hash, state.height + 1, bytes.size, txHashes)
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
        return if (fs.getUsableSpace() > MIN_DISK_SPACE)
            emptyList()
        else
            listOf("Disk space is low!")
    }

    fun check(): Check = reentrant.readLock().withLock {
        val result = Check(false, LedgerDB.state().height, 0, 0)
        val iterator = LevelDB.iterator()
        if (LevelDB.seek(iterator, LedgerDB.CHAIN_KEY)) {
            while (iterator.hasNext()) {
                val entry = iterator.next()
                if (LedgerDB.CHAIN_KEY - entry != null)
                    result.indexes += 1
            }
        }
        if (LevelDB.seek(iterator, BLOCK_KEY)) {
            while (iterator.hasNext()) {
                val entry = iterator.next()
                if (BLOCK_KEY - entry != null)
                    result.blocks += 1
            }
        }
        iterator.close()
        // genesis is not in blocks, but is in indexes
        if (result.height + 1 == result.indexes && result.height == result.blocks)
            result.result = true
        return@withLock result
    }

    @Serializable
    class Check(
        var result: Boolean,
        val height: Int,
        var indexes: Int,
        var blocks: Int,
    )
}

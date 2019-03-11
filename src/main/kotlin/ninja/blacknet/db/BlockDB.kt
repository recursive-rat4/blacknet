/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.db

import mu.KotlinLogging
import ninja.blacknet.api.APIServer
import ninja.blacknet.core.Block
import ninja.blacknet.core.DataDB
import ninja.blacknet.core.TxPool
import ninja.blacknet.crypto.Hash
import ninja.blacknet.network.Connection
import ninja.blacknet.network.Node
import org.mapdb.DBMaker
import org.mapdb.Serializer

private val logger = KotlinLogging.logger {}

object BlockDB : DataDB() {
    private val db = DBMaker.fileDB("db/blocks").transactionEnable().closeOnJvmShutdown().make()
    private val map = db.hashMap("blocks", HashSerializer, Serializer.BYTE_ARRAY).createOrOpen()

    fun commit() {
        db.commit()
    }

    fun rollback() {
        db.rollback()
    }

    fun size(): Int {
        return map.size
    }

    suspend fun block(hash: Hash): Pair<Block, Int>? {
        val bytes = get(hash) ?: return null
        val block = Block.deserialize(bytes)
        if (block == null) {
            logger.error("$hash deserialization failed")
            return null
        }
        return Pair(block, bytes.size)
    }

    suspend fun remove(list: ArrayList<Hash>) {
        list.forEach {
            remove(it)
        }
        commit()
    }

    override suspend fun contains(hash: Hash): Boolean {
        return map.contains(hash)
    }

    override suspend fun get(hash: Hash): ByteArray? {
        return map[hash]
    }

    override suspend fun remove(hash: Hash): ByteArray? {
        return map.remove(hash)
    }

    override suspend fun processImpl(hash: Hash, bytes: ByteArray, connection: Connection?): Status {
        val block = Block.deserialize(bytes)
        if (block == null) {
            logger.info("deserialization failed")
            return Status.INVALID
        }
        if (block.version != 0 && block.version != 1) {
            logger.info("unknown version ${block.version}")
        }
        if (Node.isTooFarInFuture(block.time)) {
            logger.info("too far in future ${block.time}")
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
            if (connection == null)
                logger.info("block $hash not on current chain")
            return Status.NOT_ON_THIS_CHAIN
        }
        map[hash] = bytes
        val txHashes = ArrayList<Hash>(block.transactions.size)
        if (LedgerDB.processBlock(hash, block, bytes.size, txHashes)) {
            LedgerDB.commit()
            commit()
            if (connection != null) {
                logger.info("Accepted block $hash")
                connection.lastBlockTime = Node.time()
                Node.announceChain(hash, LedgerDB.cumulativeDifficulty(), connection)
            }
            TxPool.remove(txHashes)
            APIServer.blockNotify(hash)
            return Status.ACCEPTED
        } else {
            LedgerDB.rollback()
            rollback()
            return Status.INVALID
        }
    }

    internal fun clear() {
        map.clear()
        commit()
    }
}

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
import ninja.blacknet.core.PoS
import ninja.blacknet.crypto.Hash
import ninja.blacknet.network.Node
import org.mapdb.DBMaker
import org.mapdb.Serializer

private val logger = KotlinLogging.logger {}

object BlockDB : DataDB() {
    private val db = DBMaker.fileDB("block.db").transactionEnable().fileMmapEnableIfSupported().closeOnJvmShutdown().make()
    private val map = db.hashMap("blocks", HashSerializer, Serializer.BYTE_ARRAY).createOrOpen()

    fun commit() {
        db.commit()
    }

    fun size(): Int {
        return map.size
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

    override suspend fun processImpl(hash: Hash, bytes: ByteArray): Boolean {
        if (bytes.size > LedgerDB.maxBlockSize()) {
            logger.info("too large block ${bytes.size} bytes, maximum ${LedgerDB.maxBlockSize()}")
            return false
        }
        val block = Block.deserialize(bytes)
        if (block == null) {
            logger.info("deserialization failed")
            return false
        }
        if (block.version != 0) {
            logger.info("unknown version ${block.version}")
        }
        if (Node.isTooFarInFuture(block.time)) {
            logger.info("too far in future ${block.time}")
            return false
        }
        if (!block.verifySignature(hash)) {
            logger.info("invalid signature")
            return false
        }
        if (!PoS.check(block)) {
            logger.info("invalid proof of stake")
            return false
        }
        if (LedgerDB.processBlock(hash, block, bytes.size)) {
            LedgerDB.commit()
            map[hash] = bytes
            commit()
            logger.info("Accepted block $hash")
            APIServer.blockNotify(hash)
            return true
        } else {
            LedgerDB.rollback()
            return false
        }
    }
}
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
import ninja.blacknet.core.Block
import ninja.blacknet.crypto.Hash
import org.mapdb.DBMaker

private val logger = KotlinLogging.logger {}

object Ledger {
    private val db = DBMaker.fileDB("ledger.db").transactionEnable().fileMmapEnable().closeOnJvmShutdown().make()
    private val height = db.atomicInteger("height").createOrOpen()
    private val blockHash = db.atomicVar("blockHash", HashSerializer, Hash.ZERO).createOrOpen()

    fun commit() {
        db.commit()
    }

    fun height(): Int {
        return height.get()
    }

    fun blockHash(): Hash {
        return blockHash.get()
    }

    fun processBlock(block: Block): Boolean {
        if (block.previous != blockHash()) {
            logger.error("not on current chain")
            return false
        }
        return false //TODO
    }
}
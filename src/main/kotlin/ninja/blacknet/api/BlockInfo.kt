/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.api

import kotlinx.serialization.Serializable
import mu.KotlinLogging
import ninja.blacknet.core.Block
import ninja.blacknet.core.Transaction
import ninja.blacknet.crypto.Hash
import ninja.blacknet.db.BlockDB

private val logger = KotlinLogging.logger {}

@Serializable
class BlockInfo(
        val size: Int,
        val version: Int,
        val previous: String,
        val time: Long,
        val generator: String,
        val contentHash: String,
        val signature: String,
        val transactions: List<String>
) {
    constructor(block: Block, size: Int) : this(
            size,
            block.version,
            block.previous.toString(),
            block.time,
            block.generator.toString(),
            block.contentHash.toString(),
            block.signature.toString(),
            block.transactions.map { Transaction.Hasher(it.array).toString() }
    )

    companion object {
        suspend fun get(hash: Hash): BlockInfo? {
            val bytes = BlockDB.get(hash) ?: return null
            val block = Block.deserialize(bytes)
            if (block == null) {
                logger.error("block deserialization failed")
                return null
            }
            return BlockInfo(block, bytes.size)
        }
    }
}

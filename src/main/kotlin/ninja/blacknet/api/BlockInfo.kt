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
import kotlinx.serialization.json.JSON
import ninja.blacknet.core.Block
import ninja.blacknet.core.Transaction
import ninja.blacknet.crypto.Address
import ninja.blacknet.crypto.Hash
import ninja.blacknet.db.BlockDB

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
    constructor(block: Block, size: Int, txdetail: Boolean) : this(
            size,
            block.version,
            block.previous.toString(),
            block.time,
            Address.encode(block.generator),
            block.contentHash.toString(),
            block.signature.toString(),
            block.transactions.map {
                if (txdetail)
                    TransactionInfo.fromBytes(it.array)?.let { JSON.plain.stringify(TransactionInfo.serializer(), it) } ?: "Deserialization error"
                else
                    Transaction.Hasher(it.array).toString()
            }
    )

    companion object {
        fun get(hash: Hash, txdetail: Boolean): BlockInfo? {
            val block = BlockDB.block(hash) ?: return null
            return BlockInfo(block.first, block.second, txdetail)
        }
    }
}

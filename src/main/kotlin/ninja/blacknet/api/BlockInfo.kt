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
import kotlinx.serialization.json.JsonArray
import kotlinx.serialization.json.JsonElement
import kotlinx.serialization.json.JsonLiteral
import ninja.blacknet.core.Block
import ninja.blacknet.core.Transaction
import ninja.blacknet.crypto.Address
import ninja.blacknet.crypto.Hash
import ninja.blacknet.db.BlockDB

@Serializable
class BlockInfo(
        val hash: String,
        val size: Int,
        val version: Int,
        val previous: String,
        val time: Long,
        val generator: String,
        val contentHash: String,
        val signature: String,
        val transactions: JsonElement
) {
    constructor(block: Block, hash: Hash, size: Int, transactions: JsonElement) : this(
            hash.toString(),
            size,
            block.version,
            block.previous.toString(),
            block.time,
            Address.encode(block.generator),
            block.contentHash.toString(),
            block.signature.toString(),
            transactions
    )

    companion object {
        suspend fun get(hash: Hash, txdetail: Boolean): BlockInfo? {
            val block = BlockDB.block(hash) ?: return null
            val transactions = if (txdetail) {
                JsonArray(block.first.transactions.map {
                    val bytes = it.array
                    val tx = Transaction.deserialize(bytes) ?: throw RuntimeException("Deserialization error")
                    val txHash = Transaction.Hasher(bytes)
                    val info = TransactionInfo.get(tx, txHash, bytes.size)
                    return@map APIServer.json.toJson(TransactionInfo.serializer(), info)
                })
            } else {
                JsonLiteral(block.first.transactions.size)
            }
            return BlockInfo(block.first, hash, block.second, transactions)
        }
    }
}

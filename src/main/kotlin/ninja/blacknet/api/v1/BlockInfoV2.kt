/*
 * Copyright (c) 2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.api.v1

import kotlinx.serialization.Serializable
import kotlinx.serialization.json.JsonArray
import kotlinx.serialization.json.JsonElement
import kotlinx.serialization.json.JsonLiteral
import ninja.blacknet.core.Block
import ninja.blacknet.core.Transaction
import ninja.blacknet.crypto.Address
import ninja.blacknet.crypto.Hash
import ninja.blacknet.serialization.Json

@Serializable
class BlockInfoV2(
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
    constructor(block: Block, hash: Hash, size: Int, txdetail: Boolean) : this(
            hash.toString(),
            size,
            block.version,
            block.previous.toString(),
            block.time,
            Address.encode(block.generator),
            block.contentHash.toString(),
            block.signature.toString(),
            transactions(block, txdetail)
    )

    fun toJson() = Json.toJson(serializer(), this)

    companion object {
        private fun transactions(block: Block, txdetail: Boolean): JsonElement {
            if (txdetail) {
                return JsonArray(block.transactions.map {
                    val bytes = it.array
                    val tx = Transaction.deserialize(bytes)
                    val txHash = Transaction.Hasher(bytes)
                    return@map TransactionInfoV2(tx, txHash, bytes.size).toJson()
                })
            }
            return JsonLiteral(block.transactions.size)
        }
    }
}

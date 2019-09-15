/*
 * Copyright (c) 2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.api

import kotlinx.serialization.Serializable
import kotlinx.serialization.json.JsonElement
import kotlinx.serialization.json.json
import ninja.blacknet.core.Transaction
import ninja.blacknet.crypto.Address
import ninja.blacknet.crypto.Hash
import ninja.blacknet.serialization.Json
import ninja.blacknet.transaction.TxData
import ninja.blacknet.transaction.TxType

@Serializable
class TransactionInfo(
        val hash: String,
        val size: Int,
        val signature: String,
        val from: String,
        val seq: Int,
        val blockHash: String,
        val fee: String,
        val type: Int,
        val data: JsonElement
) {
    constructor(tx: Transaction, hash: Hash, size: Int) : this(
            hash.toString(),
            size,
            tx.signature.toString(),
            Address.encode(tx.from),
            tx.seq,
            tx.blockHash.toString(),
            tx.fee.toString(),
            tx.type.toUByte().toInt(),
            data(tx.type, tx.data.array)
    )

    fun toJson() = Json.toJson(serializer(), this)

    companion object {
        fun data(type: Byte, bytes: ByteArray): JsonElement {
            if (type == TxType.Generated.type) return json {}
            val txData = TxData.deserialize(type, bytes)
            return txData.toJson()
        }
    }
}

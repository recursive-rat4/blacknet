/*
 * Copyright (c) 2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

@file:Suppress("DEPRECATION")

package ninja.blacknet.api.v1

import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.JsonElement
import kotlinx.serialization.json.json
import ninja.blacknet.core.Transaction
import ninja.blacknet.crypto.Address
import ninja.blacknet.crypto.HashSerializer
import ninja.blacknet.crypto.SignatureSerializer
import ninja.blacknet.serialization.BinaryDecoder
import ninja.blacknet.transaction.TxData
import ninja.blacknet.transaction.TxType

@Serializable
class TransactionInfoV2(
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
    constructor(tx: Transaction, hash: ByteArray, size: Int) : this(
            HashSerializer.stringify(hash),
            size,
            SignatureSerializer.stringify(tx.signature),
            Address.encode(tx.from),
            tx.seq,
            HashSerializer.stringify(tx.referenceChain),
            tx.fee.toString(),
            tx.type.toUByte().toInt(),
            data(tx.type, tx.data)
    )

    fun toJson() = Json.toJson(serializer(), this)

    companion object {
        fun data(type: Byte, bytes: ByteArray): JsonElement {
            if (type == TxType.Generated.type) return json {}
            @Suppress("UNCHECKED_CAST")
            val serializer = TxType.getSerializer(type) as KSerializer<TxData>
            val txData = BinaryDecoder(bytes).decode(serializer)
            return Json.toJson(serializer, txData)
        }
    }
}

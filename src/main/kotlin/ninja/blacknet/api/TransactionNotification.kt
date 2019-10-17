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
import ninja.blacknet.core.Transaction
import ninja.blacknet.crypto.Address
import ninja.blacknet.crypto.Hash
import ninja.blacknet.serialization.Json

@Serializable
class TransactionNotification(
        val hash: String,
        val time: Long,
        val size: Int,
        val signature: String,
        val from: String,
        val seq: Int,
        val referenceChain: String,
        val fee: String,
        val type: Int,
        val data: JsonElement
) {
    constructor(tx: Transaction, hash: Hash, time: Long, size: Int) : this(
            hash.toString(),
            time,
            size,
            tx.signature.toString(),
            Address.encode(tx.from),
            tx.seq,
            tx.referenceChain.toString(),
            tx.fee.toString(),
            tx.type.toUByte().toInt(),
            TransactionInfo.data(tx.type, tx.data.array)
    )

    fun toJson() = Json.toJson(serializer(), this)
}

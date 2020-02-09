/*
 * Copyright (c) 2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.transaction

import kotlinx.serialization.Serializable
import ninja.blacknet.core.*
import ninja.blacknet.crypto.Hash
import ninja.blacknet.serialization.BinaryDecoder
import ninja.blacknet.serialization.BinaryEncoder
import ninja.blacknet.serialization.Json
import ninja.blacknet.serialization.SerializableByteArray

@Serializable
class MultiData(
        val multiData: ArrayList<Pair<Byte, SerializableByteArray>>
) : TxData {
    override fun getType() = TxType.MultiData
    override fun serialize() = BinaryEncoder.toBytes(serializer(), this)
    override fun toJson() = Json.toJson(serializer(), this)

    override fun processImpl(tx: Transaction, hash: Hash, dataIndex: Int, ledger: Ledger): Status {
        if (dataIndex != 0) {
            return Invalid("Recursive MultiData is not permitted")
        }
        if (multiData.size < 2 || multiData.size > 20) {
            return Invalid("Invalid MultiData size ${multiData.size}")
        }

        for (index in 0 until multiData.size) {
            val (type, bytes) = multiData[index]
            val data = TxData.deserialize(type, bytes.array)
            val status = data.processImpl(tx, hash, index + 1, ledger)
            if (status != Accepted) {
                return notAccepted("MultiData ${index + 1}", status)
            }
        }

        return Accepted
    }

    companion object {
        fun deserialize(bytes: ByteArray): MultiData = BinaryDecoder(bytes).decode(serializer())
    }
}

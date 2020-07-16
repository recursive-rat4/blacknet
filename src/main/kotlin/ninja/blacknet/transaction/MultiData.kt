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
import ninja.blacknet.serialization.BinaryDecoder
import ninja.blacknet.serialization.BinaryEncoder
import ninja.blacknet.serialization.ByteArraySerializer
import ninja.blacknet.serialization.Json

@Serializable
class MultiData(
        val multiData: ArrayList<TxDataData>
) : TxData {
    @Serializable
    class TxDataData(
            val type: Byte,
            @Serializable(with = ByteArraySerializer::class)
            val data: ByteArray
    ) {
        operator fun component1() = type
        operator fun component2() = data
    }

    override fun processImpl(tx: Transaction, hash: ByteArray, dataIndex: Int, ledger: Ledger): Status {
        if (dataIndex != 0) {
            return Invalid("Recursive MultiData is not permitted")
        }
        if (multiData.size < 2 || multiData.size > 20) {
            return Invalid("Invalid MultiData size ${multiData.size}")
        }

        for (index in 0 until multiData.size) {
            val (type, bytes) = multiData[index]
            val serializer = TxType.getSerializer(type)
            val data = BinaryDecoder(bytes).decode(serializer)
            val status = data.processImpl(tx, hash, index + 1, ledger)
            if (status != Accepted) {
                return notAccepted("MultiData ${index + 1}", status)
            }
        }

        return Accepted
    }
}

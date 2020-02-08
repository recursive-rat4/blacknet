/*
 * Copyright (c) 2018-2019 Pavel Vasin
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

/**
 * 包
 */
@Serializable
class Bundle(
        val ring: VehicleRing,
        val data: SerializableByteArray
) : TxData {
    override fun getType() = TxType.Bundle
    override fun serialize() = BinaryEncoder.toBytes(serializer(), this)
    override fun toJson() = Json.toJson(serializer(), this)

    override fun processImpl(tx: Transaction, hash: Hash, dataIndex: Int, ledger: Ledger): Status {
        return Accepted
    }

    operator fun component1() = ring
    operator fun component2() = data.array

    companion object {
        fun deserialize(bytes: ByteArray): Bundle = BinaryDecoder.fromBytes(bytes).decode(serializer())
    }
}

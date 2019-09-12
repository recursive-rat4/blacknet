/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

import kotlinx.serialization.Serializable
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.serialization.BinaryDecoder
import ninja.blacknet.serialization.BinaryEncoder
import ninja.blacknet.util.sumByLong

@Serializable
class Multisig(
        val n: Byte,
        val deposits: ArrayList<Pair<PublicKey, Long>>
) {
    fun involves(publicKey: PublicKey): Boolean = deposits.find { it.first == publicKey } != null
    fun serialize(): ByteArray = BinaryEncoder.toBytes(serializer(), this)

    fun amount(): Long {
        return deposits.sumByLong { it.second }
    }

    companion object {
        fun deserialize(bytes: ByteArray): Multisig? = BinaryDecoder.fromBytes(bytes).decode(serializer())
    }
}

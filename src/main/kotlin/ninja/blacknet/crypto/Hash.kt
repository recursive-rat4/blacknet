/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

import kotlinx.serialization.Serializable
import ninja.blacknet.serialization.SerializableByteArray32
import ninja.blacknet.util.fromHex
import java.math.BigInteger

@Serializable
class Hash(val bytes: SerializableByteArray32) {
    constructor(bytes: ByteArray) : this(SerializableByteArray32(bytes))

    override fun equals(other: Any?): Boolean = (other is Hash) && bytes == other.bytes
    override fun hashCode(): Int = bytes.hashCode()
    override fun toString(): String = bytes.toString()

    fun toBigInt(): BigInt = BigInt(BigInteger(1, bytes.array))

    companion object {
        const val SIZE = 32
        val ZERO = Hash(SerializableByteArray32())

        fun fromString(hex: String?): Hash? {
            if (hex == null || hex.length != SIZE * 2)
                return null
            val bytes = fromHex(hex) ?: return null
            return Hash(bytes)
        }
    }
}
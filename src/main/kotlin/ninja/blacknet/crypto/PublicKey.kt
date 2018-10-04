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

@Serializable
class PublicKey(val bytes: SerializableByteArray32) {
    constructor(bytes: ByteArray) : this(SerializableByteArray32(bytes))

    override fun equals(other: Any?): Boolean {
        return (other is PublicKey) && bytes == other.bytes
    }

    override fun hashCode(): Int {
        return bytes.hashCode()
    }

    override fun toString(): String {
        return bytes.toString()
    }

    companion object {
        const val SIZE = 32

        fun fromString(hex: String?): PublicKey? {
            if (hex == null || hex.length != SIZE * 2)
                return null
            val bytes = fromHex(hex) ?: return null
            return PublicKey(bytes)
        }
    }
}
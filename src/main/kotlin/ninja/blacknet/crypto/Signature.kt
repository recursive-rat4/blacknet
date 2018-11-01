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
import ninja.blacknet.serialization.SerializableByteArray64
import ninja.blacknet.util.fromHex

@Serializable
class Signature(val bytes: SerializableByteArray64) {
    constructor(bytes: ByteArray) : this(SerializableByteArray64(bytes))

    override fun toString(): String = bytes.toString()

    companion object {
        const val SIZE = 64
        val EMPTY = Signature(SerializableByteArray64())

        fun fromString(hex: String?): Signature? {
            if (hex == null || hex.length != SIZE * 2)
                return null
            val bytes = fromHex(hex) ?: return null
            return Signature(bytes)
        }
    }
}

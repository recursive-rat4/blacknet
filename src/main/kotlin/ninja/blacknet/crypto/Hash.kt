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
import ninja.blacknet.core.toHex
import ninja.blacknet.serialization.SerializableByteArray32

@Serializable
class Hash(val bytes: SerializableByteArray32) {
    constructor(bytes: ByteArray) : this(SerializableByteArray32(bytes))

    override fun toString(): String {
        return bytes.array.toHex()
    }

    companion object {
        const val SIZE = 32
        val ZERO = Hash(SerializableByteArray32())
    }
}
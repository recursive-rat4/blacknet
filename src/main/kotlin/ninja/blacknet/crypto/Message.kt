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
import kotlinx.serialization.toUtf8Bytes
import ninja.blacknet.serialization.SerializableByteArray

@Serializable
class Message(
        val type: Byte,
        val data: SerializableByteArray
) {
    companion object {
        const val PLAIN: Byte = 0
        const val ENCRYPTED: Byte = 1

        fun create(string: String?, type: Byte?): Message {
            if (string == null) return empty()
            if (type == null || type == PLAIN)
                return Message(PLAIN, SerializableByteArray(string.toUtf8Bytes()))
            TODO("ENCRYPTED")
        }

        fun empty() = Message(PLAIN, SerializableByteArray.EMPTY)
    }
}
/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.serialization

import kotlinx.serialization.Decoder
import kotlinx.serialization.Encoder
import kotlinx.serialization.Serializable
import kotlinx.serialization.Serializer
import ninja.blacknet.util.toHex

@Serializable
class SerializableByteArray32(val array: ByteArray) {
    constructor() : this(ByteArray(SIZE))

    override fun equals(other: Any?): Boolean {
        return (other is SerializableByteArray32) && array.contentEquals(other.array)
    }

    override fun hashCode(): Int {
        return array.contentHashCode()
    }

    override fun toString(): String {
        return array.toHex()
    }

    @Serializer(forClass = SerializableByteArray32::class)
    companion object {
        const val SIZE = 32

        override fun deserialize(input: Decoder): SerializableByteArray32 {
            return SerializableByteArray32((input as BlacknetDecoder).decodeByteArrayValue(SIZE))
        }

        override fun serialize(output: Encoder, obj: SerializableByteArray32) {
            (output as BlacknetEncoder).encodeByteArrayValue(obj.array, SIZE)
        }
    }
}
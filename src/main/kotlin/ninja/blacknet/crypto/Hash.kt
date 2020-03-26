/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

import kotlinx.serialization.Decoder
import kotlinx.serialization.Encoder
import kotlinx.serialization.Serializable
import kotlinx.serialization.Serializer
import kotlinx.serialization.json.JsonInput
import kotlinx.serialization.json.JsonOutput
import ninja.blacknet.coding.fromHex
import ninja.blacknet.coding.toHex
import ninja.blacknet.serialization.BinaryDecoder
import ninja.blacknet.serialization.BinaryEncoder

/**
 * Blake2b hash
 */
@Serializable
class Hash(val bytes: ByteArray) {
    override fun equals(other: Any?): Boolean = (other is Hash) && bytes.contentEquals(other.bytes)
    override fun hashCode(): Int = Salt.hashCode { x(bytes) }
    override fun toString(): String = bytes.toHex()

    @Serializer(forClass = Hash::class)
    companion object {
        /**
         * The number of bytes in a binary representation of a [Hash].
         */
        const val SIZE_BYTES = Blake2b.DIGEST_SIZE_BYTES
        val ZERO = Hash(ByteArray(SIZE_BYTES))

        fun fromString(hex: String?): Hash? {
            if (hex == null) return null
            val bytes = fromHex(hex, SIZE_BYTES) ?: return null
            return Hash(bytes)
        }

        override fun deserialize(decoder: Decoder): Hash {
            return when (decoder) {
                is BinaryDecoder -> Hash(decoder.decodeFixedByteArray(SIZE_BYTES))
                is JsonInput -> Hash.fromString(decoder.decodeString())!!
                else -> throw RuntimeException("Unsupported decoder")
            }
        }

        override fun serialize(encoder: Encoder, value: Hash) {
            when (encoder) {
                is BinaryEncoder -> encoder.encodeFixedByteArray(value.bytes)
                is JsonOutput -> encoder.encodeString(value.bytes.toHex())
                else -> throw RuntimeException("Unsupported encoder")
            }
        }
    }
}

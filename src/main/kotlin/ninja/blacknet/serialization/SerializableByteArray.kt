/*
 * Copyright (c) 2018-2019 Pavel Vasin
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
import kotlinx.serialization.json.JsonInput
import kotlinx.serialization.json.JsonOutput
import ninja.blacknet.coding.fromHex
import ninja.blacknet.coding.toHex
import ninja.blacknet.crypto.HashCoder
import ninja.blacknet.crypto.SipHash.hashCode
import ninja.blacknet.crypto.encodeByteArray
import ninja.blacknet.util.emptyByteArray

/**
 * Serializable [ByteArray]
 */
@Serializable
class SerializableByteArray(
        val array: ByteArray
) {
    override fun equals(other: Any?): Boolean {
        return (other is SerializableByteArray) && array.contentEquals(other.array)
    }

    override fun hashCode(): Int {
        return hashCode(serializer(), this)
    }

    override fun toString(): String {
        return array.toHex()
    }

    @Serializer(forClass = SerializableByteArray::class)
    companion object {
        val EMPTY = SerializableByteArray(emptyByteArray())

        fun fromString(hex: String?): SerializableByteArray? {
            if (hex == null) return null
            val bytes = fromHex(hex) ?: return null
            return SerializableByteArray(bytes)
        }

        override fun deserialize(decoder: Decoder): SerializableByteArray {
            return when (decoder) {
                is BinaryDecoder -> SerializableByteArray(decoder.decodeByteArray())
                is JsonInput -> fromString(decoder.decodeString())!!
                else -> throw notSupportedDecoderException(decoder, this)
            }
        }

        override fun serialize(encoder: Encoder, value: SerializableByteArray) {
            when (encoder) {
                is BinaryEncoder -> encoder.encodeByteArray(value.array)
                is HashCoder -> encoder.encodeByteArray(value.array)
                is JsonOutput -> encoder.encodeString(value.array.toHex())
                else -> throw notSupportedEncoderException(encoder, this)
            }
        }
    }
}

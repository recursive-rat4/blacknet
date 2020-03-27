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
import ninja.blacknet.serialization.notSupportedDecoderException
import ninja.blacknet.serialization.notSupportedEncoderException

/**
 * Ed25519 signature
 */
@Serializable
class Signature(val bytes: ByteArray) {
    override fun equals(other: Any?): Boolean = (other is Signature) && bytes.contentEquals(other.bytes)
    override fun hashCode(): Int = Salt.hashCode { x(bytes) }
    override fun toString(): String = bytes.toHex()

    @Serializer(forClass = Signature::class)
    companion object {
        /**
         * The number of bytes in a binary representation of a [Signature].
         */
        const val SIZE_BYTES = 64
        val EMPTY = Signature(ByteArray(SIZE_BYTES))

        fun fromString(hex: String?): Signature? {
            if (hex == null) return null
            val bytes = fromHex(hex, SIZE_BYTES) ?: return null
            return Signature(bytes)
        }

        override fun deserialize(decoder: Decoder): Signature {
            return when (decoder) {
                is BinaryDecoder -> Signature(decoder.decodeFixedByteArray(SIZE_BYTES))
                is JsonInput -> Signature.fromString(decoder.decodeString())!!
                else -> throw notSupportedDecoderException(decoder, this)
            }
        }

        override fun serialize(encoder: Encoder, value: Signature) {
            when (encoder) {
                is BinaryEncoder -> encoder.encodeFixedByteArray(value.bytes)
                is JsonOutput -> encoder.encodeString(value.bytes.toHex())
                else -> throw notSupportedEncoderException(encoder, this)
            }
        }
    }
}

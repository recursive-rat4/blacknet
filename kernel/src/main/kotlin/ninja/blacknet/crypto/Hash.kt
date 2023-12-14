/*
 * Copyright (c) 2018-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

import java.util.Arrays
import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable
import kotlinx.serialization.descriptors.PrimitiveKind
import kotlinx.serialization.descriptors.PrimitiveSerialDescriptor
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import kotlinx.serialization.json.JsonDecoder
import kotlinx.serialization.json.JsonEncoder
import ninja.blacknet.crypto.SipHash.hashCode
import ninja.blacknet.rpc.requests.RequestDecoder
import ninja.blacknet.serialization.bbf.BinaryDecoder
import ninja.blacknet.serialization.bbf.BinaryEncoder
import ninja.blacknet.serialization.config.ConfigDecoder
import ninja.blacknet.serialization.notSupportedFormatError

/**
 * Represents a BLAKE2b-256 hash.
 */
@Serializable(Hash.Companion::class)
class Hash(
    val bytes: ByteArray
) : Comparable<Hash> {
    override fun equals(other: Any?): Boolean {
        return (other is Hash) && bytes.contentEquals(other.bytes)
    }

    override fun hashCode(): Int {
        return hashCode(serializer(), this)
    }

    override fun toString(): String {
        return encodeHex(bytes, Hash.SIZE_BYTES * 2)
    }

    override fun compareTo(other: Hash): Int {
        return Arrays.compareUnsigned(bytes, other.bytes)
    }

    companion object : KSerializer<Hash> {
        /**
         * The number of bytes in a binary representation of the hash.
         */
        const val SIZE_BYTES = 32
        val ZERO = Hash(ByteArray(SIZE_BYTES))

        //JSON override val descriptor: SerialDescriptor = ListSerialDescriptor("ninja.blacknet.crypto.Hash", Byte.serializer().descriptor)
        override val descriptor: SerialDescriptor = PrimitiveSerialDescriptor("ninja.blacknet.crypto.Hash", PrimitiveKind.STRING)

        override fun deserialize(decoder: Decoder) = Hash(
            when (decoder) {
                is BinaryDecoder,
                    -> decoder.decodeFixedByteArray(SIZE_BYTES)
                is ConfigDecoder,
                is JsonDecoder,
                is RequestDecoder,
                    -> fromString(decoder.decodeString())
                else
                    -> throw notSupportedFormatError(decoder, this)
            }
        )

        override fun serialize(encoder: Encoder, value: Hash) {
            when (encoder) {
                is BinaryEncoder,
                    -> encoder.encodeFixedByteArray(value.bytes)
                is HashEncoder,
                    -> encoder.encodeByteArray(value.bytes)
                is JsonEncoder,
                    -> encoder.encodeString(value.toString())
                else
                    -> throw notSupportedFormatError(encoder, this)
            }
        }

        internal fun fromString(string: String) =
            decodeHex(string, SIZE_BYTES * 2)
    }
}

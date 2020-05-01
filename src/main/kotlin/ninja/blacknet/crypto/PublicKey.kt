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
import ninja.blacknet.crypto.SipHash.hashCode
import ninja.blacknet.serialization.BinaryDecoder
import ninja.blacknet.serialization.BinaryEncoder
import ninja.blacknet.serialization.notSupportedDecoderError
import ninja.blacknet.serialization.notSupportedEncoderError

/**
 * Represents an Ed25519 public key.
 */
@Serializable
class PublicKey(val bytes: ByteArray) {
    override fun equals(other: Any?): Boolean = (other is PublicKey) && bytes.contentEquals(other.bytes)
    override fun hashCode(): Int = hashCode(serializer(), this)
    override fun toString(): String = bytes.toHex()

    @Serializer(forClass = PublicKey::class)
    companion object {
        /**
         * The number of bytes in a binary representation of a [PublicKey].
         */
        const val SIZE_BYTES = 32

        fun fromString(hex: String): PublicKey {
            val bytes = fromHex(hex, SIZE_BYTES)
            return PublicKey(bytes)
        }

        override fun deserialize(decoder: Decoder): PublicKey {
            return when (decoder) {
                is BinaryDecoder -> PublicKey(decoder.decodeFixedByteArray(SIZE_BYTES))
                is JsonInput -> Address.decode(decoder.decodeString())
                else -> throw notSupportedDecoderError(decoder, this)
            }
        }

        override fun serialize(encoder: Encoder, value: PublicKey) {
            when (encoder) {
                is BinaryEncoder -> encoder.encodeFixedByteArray(value.bytes)
                is HashCoder -> encoder.encodePublicKey(value)
                is JsonOutput -> encoder.encodeString(Address.encode(value))
                else -> throw notSupportedEncoderError(encoder, this)
            }
        }
    }
}

/**
 * Encodes a public key value.
 *
 * @param value the [PublicKey] containing the data
 */
fun HashCoder.encodePublicKey(value: PublicKey) {
    writer.writeByteArray(value.bytes)
}

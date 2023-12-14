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
 * An Ed25519-BLAKE2b public key.
 */
@Serializable(PublicKey.Companion::class)
class PublicKey(
    val bytes: ByteArray
) : Comparable<PublicKey> {
    override fun equals(other: Any?): Boolean {
        return (other is PublicKey) && bytes.contentEquals(other.bytes)
    }

    override fun hashCode(): Int {
        return hashCode(serializer(), this)
    }

    override fun toString(): String {
        return Address.encode(bytes)
    }

    override fun compareTo(other: PublicKey): Int {
        return Arrays.compareUnsigned(bytes, other.bytes)
    }

    companion object : KSerializer<PublicKey> {
        /**
         * The number of bytes in a binary representation of the public key.
         */
        const val SIZE_BYTES = 32

        //JSON override val descriptor: SerialDescriptor = ListSerialDescriptor("ninja.blacknet.crypto.PublicKey", Byte.serializer().descriptor)
        override val descriptor: SerialDescriptor = PrimitiveSerialDescriptor("ninja.blacknet.crypto.PublicKey", PrimitiveKind.STRING)

        override fun deserialize(decoder: Decoder) = PublicKey(
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

        override fun serialize(encoder: Encoder, value: PublicKey) {
            when (encoder) {
                is BinaryEncoder,
                    -> encoder.encodeFixedByteArray(value.bytes)
                is HashEncoder,
                    -> encoder.encodeByteArray(value.bytes)
                is JsonEncoder,
                    -> encoder.encodeString(Address.encode(value.bytes))
                else
                    -> throw notSupportedFormatError(encoder, this)
            }
        }

        private fun fromString(string: String) =
            try {
                decodeHex(string, SIZE_BYTES * 2)
            } catch (e: IllegalArgumentException) {
                Address.decode(string)
            }
    }
}

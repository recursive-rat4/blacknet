/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.contract

import kotlinx.serialization.Decoder
import kotlinx.serialization.Encoder
import kotlinx.serialization.Serializable
import kotlinx.serialization.Serializer
import kotlinx.serialization.json.JsonInput
import kotlinx.serialization.json.JsonOutput
import ninja.blacknet.crypto.Address
import ninja.blacknet.crypto.HashCoder
import ninja.blacknet.crypto.SipHash.hashCode
import ninja.blacknet.ktor.requests.RequestDecoder
import ninja.blacknet.serialization.BinaryDecoder
import ninja.blacknet.serialization.BinaryEncoder
import ninja.blacknet.serialization.notSupportedDecoderError
import ninja.blacknet.serialization.notSupportedEncoderError

/**
 * Represents an id of a dapp.
 */
@Serializable
class DAppId(val bytes: ByteArray) {
    override fun equals(other: Any?): Boolean = (other is DAppId) && bytes.contentEquals(other.bytes)
    override fun hashCode(): Int = hashCode(serializer(), this)
    override fun toString(): String = Address.encode(Address.DAPP, bytes)

    @Serializer(forClass = DAppId::class)
    companion object {
        /**
         * The number of bytes in a binary representation of a [DAppId].
         */
        const val SIZE_BYTES = 4

        fun parse(string: String): DAppId {
            val bytes = Address.decode(Address.DAPP, string)
            return DAppId(bytes)
        }

        override fun deserialize(decoder: Decoder): DAppId {
            return when (decoder) {
                is BinaryDecoder -> DAppId(decoder.decodeFixedByteArray(SIZE_BYTES))
                is RequestDecoder -> DAppId.parse(decoder.decodeString())
                is JsonInput -> DAppId.parse(decoder.decodeString())
                else -> throw notSupportedDecoderError(decoder, this)
            }
        }

        override fun serialize(encoder: Encoder, value: DAppId) {
            when (encoder) {
                is BinaryEncoder -> encoder.encodeFixedByteArray(value.bytes)
                is HashCoder -> encoder.encodeDAppId(value)
                is JsonOutput -> encoder.encodeString(value.toString())
                else -> throw notSupportedEncoderError(encoder, this)
            }
        }
    }
}

/**
 * Encodes a dapp id value.
 *
 * @param value the [DAppId] containing the data
 */
fun HashCoder.encodeDAppId(value: DAppId) {
    writer.writeByteArray(value.bytes)
}

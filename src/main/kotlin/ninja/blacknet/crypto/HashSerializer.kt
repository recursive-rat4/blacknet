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
import kotlinx.serialization.KSerializer
import kotlinx.serialization.SerialDescriptor
import kotlinx.serialization.StructureKind
import kotlinx.serialization.builtins.ListSerializer
import kotlinx.serialization.json.JsonInput
import kotlinx.serialization.json.JsonOutput
import ninja.blacknet.coding.fromHex
import ninja.blacknet.coding.toHex
import ninja.blacknet.crypto.SipHash.hashCode
import ninja.blacknet.crypto.encodeByteArray
import ninja.blacknet.ktor.requests.RequestDecoder
import ninja.blacknet.serialization.BinaryDecoder
import ninja.blacknet.serialization.BinaryEncoder
import ninja.blacknet.serialization.notSupportedDecoderError
import ninja.blacknet.serialization.notSupportedEncoderError

/**
 * Serializes a BLAKE2b-256 hash.
 */
object HashSerializer : KSerializer<ByteArray> {
    /**
     * The number of bytes in a binary representation of the hash.
     */
    const val SIZE_BYTES = 32
    val ZERO = ByteArray(SIZE_BYTES)

    override val descriptor: SerialDescriptor = SerialDescriptor(
        "ninja.blacknet.crypto.HashSerializer",
        StructureKind.LIST  // PrimitiveKind.STRING
    )

    fun parse(string: String): ByteArray {
        return fromHex(string, SIZE_BYTES)
    }

    fun stringify(hash: ByteArray): String {
        return hash.toHex()
    }

    override fun deserialize(decoder: Decoder): ByteArray {
        return when (decoder) {
            is BinaryDecoder -> decoder.decodeFixedByteArray(SIZE_BYTES)
            is RequestDecoder,
            is JsonInput -> parse(decoder.decodeString())
            else -> throw notSupportedDecoderError(decoder, this)
        }
    }

    override fun serialize(encoder: Encoder, value: ByteArray) {
        when (encoder) {
            is BinaryEncoder -> encoder.encodeFixedByteArray(value)
            is HashCoder -> encoder.encodeByteArray(value)
            is JsonOutput -> encoder.encodeString(stringify(value))
            else -> throw notSupportedEncoderError(encoder, this)
        }
    }
}

/**
 * Serializes a [`List<ByteArray>`][List] with [ListSerializer] and [HashSerializer].
 */
object HashListSerializer : KSerializer<List<ByteArray>> by ListSerializer(HashSerializer)

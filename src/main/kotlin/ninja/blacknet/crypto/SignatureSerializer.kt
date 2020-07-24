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
import kotlinx.serialization.Serializer
import kotlinx.serialization.StructureKind
import kotlinx.serialization.json.JsonInput
import kotlinx.serialization.json.JsonOutput
import ninja.blacknet.coding.fromHex
import ninja.blacknet.coding.toHex
import ninja.blacknet.ktor.requests.RequestDecoder
import ninja.blacknet.serialization.BinaryDecoder
import ninja.blacknet.serialization.BinaryEncoder
import ninja.blacknet.serialization.notSupportedCoderError

val EMPTY_SIGNATURE = ByteArray(SignatureSerializer.SIZE_BYTES)

/**
 * Serializes an Ed25519 signature.
 */
object SignatureSerializer : KSerializer<ByteArray> {
    /**
     * The number of bytes in a binary representation of the signature.
     */
    const val SIZE_BYTES = 64

    override val descriptor: SerialDescriptor = SerialDescriptor(
        "ninja.blacknet.crypto.SignatureSerializer",
        StructureKind.LIST  // PrimitiveKind.STRING
    )

    fun parse(string: String): ByteArray {
        return fromHex(string, SIZE_BYTES)
    }

    fun stringify(signature: ByteArray): String {
        return signature.toHex()
    }

    override fun deserialize(decoder: Decoder): ByteArray {
        return when (decoder) {
            is BinaryDecoder -> decoder.decodeFixedByteArray(SIZE_BYTES)
            is RequestDecoder -> parse(decoder.decodeString())
            is JsonInput -> parse(decoder.decodeString())
            else -> throw notSupportedCoderError(decoder, this)
        }
    }

    override fun serialize(encoder: Encoder, value: ByteArray) {
        when (encoder) {
            is BinaryEncoder -> encoder.encodeFixedByteArray(value)
            is HashCoder -> encoder.encodeByteArray(value)
            is JsonOutput -> encoder.encodeString(value.toHex())
            else -> throw notSupportedCoderError(encoder, this)
        }
    }
}

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
import ninja.blacknet.codec.base.Base16
import ninja.blacknet.rpc.requests.RequestDecoder
import ninja.blacknet.serialization.BinaryDecoder
import ninja.blacknet.serialization.BinaryEncoder
import ninja.blacknet.serialization.notSupportedFormatError

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
        return decodeHex(string, SIZE_BYTES * 2)
    }

    fun stringify(signature: ByteArray): String {
        return encodeHex(signature, SIZE_BYTES * 2)
    }

    override fun deserialize(decoder: Decoder): ByteArray {
        return when (decoder) {
            is BinaryDecoder -> decoder.decodeFixedByteArray(SIZE_BYTES)
            is RequestDecoder,
            is JsonInput -> parse(decoder.decodeString())
            else -> throw notSupportedFormatError(decoder, this)
        }
    }

    override fun serialize(encoder: Encoder, value: ByteArray) {
        when (encoder) {
            is BinaryEncoder -> encoder.encodeFixedByteArray(value)
            is HashCoder -> encoder.encodeByteArray(value)
            is JsonOutput -> encoder.encodeString(stringify(value))
            else -> throw notSupportedFormatError(encoder, this)
        }
    }
}

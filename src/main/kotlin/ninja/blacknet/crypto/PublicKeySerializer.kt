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
import kotlinx.serialization.json.JsonInput
import kotlinx.serialization.json.JsonOutput
import ninja.blacknet.coding.HexFormatException
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
 * The number of bytes in a binary representation of a public key.
 */
const val PUBLIC_KEY_SIZE_BYTES = 32

/**
 * Serializes an Ed25519 public key.
 */
object PublicKeySerializer : KSerializer<ByteArray> {
    override val descriptor: SerialDescriptor = SerialDescriptor(
        "ninja.blacknet.crypto.PublicKeySerializer",
        StructureKind.LIST  // PrimitiveKind.STRING
    )

    fun parse(string: String): ByteArray {
        return try {
            fromHex(string, PUBLIC_KEY_SIZE_BYTES)
        } catch (e: HexFormatException) {
            try {
                Address.decode(string)
            } catch (e: Throwable) {
                throw e
            }
        }
    }

    override fun deserialize(decoder: Decoder): ByteArray {
        return when (decoder) {
            is BinaryDecoder -> decoder.decodeFixedByteArray(PUBLIC_KEY_SIZE_BYTES)
            is RequestDecoder,
            is JsonInput -> parse(decoder.decodeString())
            else -> throw notSupportedDecoderError(decoder, this)
        }
    }

    override fun serialize(encoder: Encoder, value: ByteArray) {
        when (encoder) {
            is BinaryEncoder -> encoder.encodeFixedByteArray(value)
            is HashCoder -> encoder.encodeByteArray(value)
            is JsonOutput -> encoder.encodeString(Address.encode(value))
            else -> throw notSupportedEncoderError(encoder, this)
        }
    }
}

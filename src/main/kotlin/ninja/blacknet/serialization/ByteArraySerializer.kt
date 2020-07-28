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
import kotlinx.serialization.KSerializer
import kotlinx.serialization.SerialDescriptor
import kotlinx.serialization.StructureKind
import kotlinx.serialization.json.JsonInput
import kotlinx.serialization.json.JsonOutput
import ninja.blacknet.coding.fromHex
import ninja.blacknet.coding.toHex
import ninja.blacknet.crypto.HashCoder
import ninja.blacknet.crypto.SipHash.hashCode
import ninja.blacknet.crypto.encodeByteArray
import ninja.blacknet.ktor.requests.RequestDecoder

/**
 * Serializes a [ByteArray] with a transformation to a hex string in some representations.
 */
object ByteArraySerializer : KSerializer<ByteArray> {
    override val descriptor: SerialDescriptor = SerialDescriptor(
        "ninja.blacknet.serialization.ByteArraySerializer",
        StructureKind.LIST  // PrimitiveKind.STRING
    )

    override fun deserialize(decoder: Decoder): ByteArray {
        return when (decoder) {
            is BinaryDecoder -> decoder.decodeByteArray()
            is RequestDecoder -> fromHex(decoder.decodeString())
            is JsonInput -> fromHex(decoder.decodeString())
            else -> throw notSupportedCoderError(decoder, this)
        }
    }

    override fun serialize(encoder: Encoder, value: ByteArray) {
        when (encoder) {
            is BinaryEncoder -> encoder.encodeByteArray(value)
            is HashCoder -> encoder.encodeByteArray(value)
            is JsonOutput -> encoder.encodeString(value.toHex())
            else -> throw notSupportedCoderError(encoder, this)
        }
    }
}

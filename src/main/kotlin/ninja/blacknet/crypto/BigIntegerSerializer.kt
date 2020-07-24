/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

import java.math.BigInteger
import kotlinx.serialization.Decoder
import kotlinx.serialization.Encoder
import kotlinx.serialization.KSerializer
import kotlinx.serialization.SerialDescriptor
import kotlinx.serialization.Serializer
import kotlinx.serialization.StructureKind
import kotlinx.serialization.json.JsonInput
import kotlinx.serialization.json.JsonOutput
import ninja.blacknet.ktor.requests.RequestDecoder
import ninja.blacknet.serialization.BinaryDecoder
import ninja.blacknet.serialization.BinaryEncoder
import ninja.blacknet.serialization.notSupportedCoderError

/**
 * Serializes a [BigInteger] with a transformation to a decimal string in some representations.
 */
object BigIntegerSerializer : KSerializer<BigInteger> {
    override val descriptor: SerialDescriptor = SerialDescriptor(
        "ninja.blacknet.crypto.BigIntegerSerializer",
        StructureKind.LIST  // PrimitiveKind.STRING
    )

    override fun deserialize(decoder: Decoder): BigInteger {
        return when (decoder) {
            is BinaryDecoder -> BigInteger(decoder.decodeByteArray())
            is RequestDecoder -> BigInteger(decoder.decodeString())
            is JsonInput -> BigInteger(decoder.decodeString())
            else -> throw notSupportedCoderError(decoder, this)
        }
    }

    override fun serialize(encoder: Encoder, value: BigInteger) {
        when (encoder) {
            is BinaryEncoder -> encoder.encodeByteArray(value.toByteArray())
            is HashCoder -> encoder.encodeByteArray(value.toByteArray())
            is JsonOutput -> encoder.encodeString(value.toString())
            else -> throw notSupportedCoderError(encoder, this)
        }
    }
}

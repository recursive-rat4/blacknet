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
import kotlinx.serialization.KSerializer
import kotlinx.serialization.builtins.serializer
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import kotlinx.serialization.json.JsonDecoder
import kotlinx.serialization.json.JsonEncoder
import ninja.blacknet.rpc.requests.RequestDecoder
import ninja.blacknet.serialization.bbf.BinaryDecoder
import ninja.blacknet.serialization.bbf.BinaryEncoder
import ninja.blacknet.serialization.notSupportedFormatError
import ninja.blacknet.serialization.descriptor.ListSerialDescriptor

/**
 * Serializes a [BigInteger] with a transformation to a decimal string in some representations.
 */
object BigIntegerSerializer : KSerializer<BigInteger> {
    override val descriptor: SerialDescriptor = ListSerialDescriptor(
            "ninja.blacknet.crypto.BigIntegerSerializer",
            Byte.serializer().descriptor  // PrimitiveKind.STRING
    )

    override fun deserialize(decoder: Decoder): BigInteger {
        return when (decoder) {
            is BinaryDecoder -> BigInteger(decoder.decodeByteArray())
            is RequestDecoder,
            is JsonDecoder -> BigInteger(decoder.decodeString())
            else -> throw notSupportedFormatError(decoder, this)
        }
    }

    override fun serialize(encoder: Encoder, value: BigInteger) {
        when (encoder) {
            is BinaryEncoder -> encoder.encodeByteArray(value.toByteArray())
            is HashCoder -> encoder.encodeByteArray(value.toByteArray())
            is JsonEncoder -> encoder.encodeString(value.toString())
            else -> throw notSupportedFormatError(encoder, this)
        }
    }
}

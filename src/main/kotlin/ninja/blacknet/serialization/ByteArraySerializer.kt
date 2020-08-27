/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.serialization

import kotlinx.serialization.KSerializer
import kotlinx.serialization.builtins.serializer
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import kotlinx.serialization.json.JsonDecoder
import kotlinx.serialization.json.JsonEncoder
import ninja.blacknet.codec.base.Base16
import ninja.blacknet.crypto.HashCoder
import ninja.blacknet.crypto.encodeByteArray
import ninja.blacknet.rpc.requests.RequestDecoder
import ninja.blacknet.serialization.descriptor.ListSerialDescriptor

/**
 * Serializes a [ByteArray] with a transformation to a hex string in some representations.
 */
object ByteArraySerializer : KSerializer<ByteArray> {
    override val descriptor: SerialDescriptor = ListSerialDescriptor(
            "ninja.blacknet.serialization.ByteArraySerializer",
            Byte.serializer().descriptor  // PrimitiveKind.STRING
    )

    override fun deserialize(decoder: Decoder): ByteArray {
        return when (decoder) {
            is BinaryDecoder -> decoder.decodeByteArray()
            is RequestDecoder,
            is JsonDecoder -> Base16.decode(decoder.decodeString())
            else -> throw notSupportedFormatError(decoder, this)
        }
    }

    override fun serialize(encoder: Encoder, value: ByteArray) {
        when (encoder) {
            is BinaryEncoder -> encoder.encodeByteArray(value)
            is HashCoder -> encoder.encodeByteArray(value)
            is JsonEncoder -> encoder.encodeString(Base16.encode(value))
            else -> throw notSupportedFormatError(encoder, this)
        }
    }
}

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
import kotlinx.serialization.SerialFormat
import kotlinx.serialization.SerializationException
import kotlinx.serialization.descriptors.PrimitiveKind
import kotlinx.serialization.descriptors.PrimitiveSerialDescriptor
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonDecoder
import kotlinx.serialization.json.JsonEncoder
import kotlin.experimental.and
import kotlin.experimental.or
import ninja.blacknet.crypto.HashCoder
import ninja.blacknet.rpc.requests.RequestDecoder
import ninja.blacknet.serialization.notSupportedFormatError
import ninja.blacknet.serialization.bbf.*

/**
 * Serializes a [Long] with variable number of bytes in a binary representation.
 */
object VarLongSerializer : KSerializer<Long> {
    fun descriptor(format: SerialFormat): SerialDescriptor = PrimitiveSerialDescriptor(
        "ninja.blacknet.serialization.VarLongSerializer",
        if (format !is Json)
            PrimitiveKind.LONG
        else
            PrimitiveKind.STRING
    )

    override val descriptor: SerialDescriptor = PrimitiveSerialDescriptor(
        "ninja.blacknet.serialization.VarLongSerializer",
        PrimitiveKind.LONG  // PrimitiveKind.STRING
    )

    override fun deserialize(decoder: Decoder): Long {
        return when (decoder) {
            is BinaryDecoder -> decoder.decodeVarLong()
            is JsonDecoder -> decoder.decodeString().toLong()
            is RequestDecoder -> decoder.decodeLong()
            else -> throw notSupportedFormatError(decoder, this)
        }
    }

    override fun serialize(encoder: Encoder, value: Long) {
        when (encoder) {
            is BinaryEncoder -> encoder.encodeVarLong(value)
            is HashCoder -> encoder.encodeLong(value)
            is JsonEncoder -> encoder.encodeString(value.toString())
            else -> throw notSupportedFormatError(encoder, this)
        }
    }
}

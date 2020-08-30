/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 *
 * decodeVarLong, encodeVarLong originally come from MapDB http://www.mapdb.org/
 * licensed under the Apache License, Version 2.0
 */

package ninja.blacknet.serialization

import kotlinx.serialization.Decoder
import kotlinx.serialization.Encoder
import kotlinx.serialization.KSerializer
import kotlinx.serialization.PrimitiveDescriptor
import kotlinx.serialization.PrimitiveKind
import kotlinx.serialization.SerialDescriptor
import kotlinx.serialization.SerialFormat
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonInput
import kotlinx.serialization.json.JsonOutput
import kotlin.experimental.and
import kotlin.experimental.or
import ninja.blacknet.crypto.HashCoder
import ninja.blacknet.rpc.requests.RequestDecoder

/**
 * Serializes a [Long] with variable number of bytes in a binary representation.
 */
object VarLongSerializer : KSerializer<Long> {
    /**
     * The maximum number of bytes in a binary representation of the [Long].
     */
    const val MAX_SIZE_BYTES = 10

    fun descriptor(format: SerialFormat): SerialDescriptor = PrimitiveDescriptor(
        "ninja.blacknet.serialization.VarLongSerializer",
        if (format !is Json)
            PrimitiveKind.LONG
        else
            PrimitiveKind.STRING
    )

    override val descriptor: SerialDescriptor = PrimitiveDescriptor(
        "ninja.blacknet.serialization.VarLongSerializer",
        PrimitiveKind.LONG  // PrimitiveKind.STRING
    )

    override fun deserialize(decoder: Decoder): Long {
        return when (decoder) {
            is BinaryDecoder -> decoder.decodeVarLong()
            is JsonInput -> decoder.decodeString().toLong()
            is RequestDecoder -> decoder.decodeLong()
            else -> throw notSupportedCoderError(decoder, this)
        }
    }

    override fun serialize(encoder: Encoder, value: Long) {
        when (encoder) {
            is BinaryEncoder -> encoder.encodeVarLong(value)
            is HashCoder -> encoder.encodeLong(value)
            is JsonOutput -> encoder.encodeString(value.toString())
            else -> throw notSupportedCoderError(encoder, this)
        }
    }
}

/**
 * Decodes a variable length long value.
 *
 * @return the [Long] value containing the data
 */
fun Decoder.decodeVarLong(): Long {
    var c = VarLongSerializer.MAX_SIZE_BYTES + 1
    var result = 0L
    var v: Byte
    do {
        if (--c != 0) {
            v = decodeByte()
            result = result shl 7 or (v and 0x7F.toByte()).toLong()
        } else {
            throw SerializationException("Too long VarLong")
        }
    } while (v and 0x80.toByte() == 0.toByte())
    return result
}

/**
 * Encodes a variable length long value.
 *
 * @param value the [Long] containing the data
 */
fun Encoder.encodeVarLong(value: Long) {
    var shift = 63 - java.lang.Long.numberOfLeadingZeros(value)
    shift -= shift % 7 // round down to nearest multiple of 7
    while (shift != 0) {
        encodeByte(value.ushr(shift).toByte() and 0x7F)
        shift -= 7
    }
    encodeByte(value.toByte() and 0x7F or 0x80.toByte())
}

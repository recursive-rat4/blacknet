/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 *
 * decodeVarInt, encodeVarInt originally come from MapDB http://www.mapdb.org/
 * licensed under the Apache License, Version 2.0
 */

package ninja.blacknet.serialization

import kotlinx.serialization.Decoder
import kotlinx.serialization.Encoder
import kotlinx.serialization.KSerializer
import kotlinx.serialization.PrimitiveDescriptor
import kotlinx.serialization.PrimitiveKind
import kotlinx.serialization.SerialDescriptor
import kotlinx.serialization.json.JsonInput
import kotlinx.serialization.json.JsonOutput
import kotlin.experimental.and
import kotlin.experimental.or
import ninja.blacknet.crypto.HashCoder
import ninja.blacknet.ktor.requests.RequestDecoder

/**
 * Serializes an [Int] with variable number of bytes in a binary representation.
 */
object VarIntSerializer : KSerializer<Int> {
    /**
     * The maximum number of bytes in a binary representation of a [Int].
     */
    const val MAX_SIZE_BYTES = 5

    override val descriptor: SerialDescriptor = PrimitiveDescriptor(
        "ninja.blacknet.serialization.VarIntSerializer",
        PrimitiveKind.INT
    )

    override fun deserialize(decoder: Decoder): Int {
        return when (decoder) {
            is BinaryDecoder -> decoder.decodeVarInt()
            is JsonInput, is RequestDecoder -> decoder.decodeInt()
            else -> throw notSupportedDecoderError(decoder, this)
        }
    }

    override fun serialize(encoder: Encoder, value: Int) {
        when (encoder) {
            is BinaryEncoder -> encoder.encodeVarInt(value)
            is HashCoder, is JsonOutput -> encoder.encodeInt(value)
            else -> throw notSupportedEncoderError(encoder, this)
        }
    }
}

/**
 * Decodes a variable length int value.
 *
 * @return the [Int] value containing the data
 */
fun Decoder.decodeVarInt(): Int {
    var c = VarIntSerializer.MAX_SIZE_BYTES + 1
    var result = 0
    var v: Byte
    do {
        if (--c != 0) {
            v = decodeByte()
            result = result shl 7 or (v and 0x7F.toByte()).toInt()
        } else {
            throw DecoderException("Too long VarInt")
        }
    } while (v and 0x80.toByte() == 0.toByte())
    return result
}

/**
 * Encodes a variable length int value.
 *
 * @param value the [Int] containing the data
 */
fun Encoder.encodeVarInt(value: Int) {
    var shift = 31 - Integer.numberOfLeadingZeros(value)
    shift -= shift % 7 // round down to nearest multiple of 7
    while (shift != 0) {
        encodeByte(value.ushr(shift).toByte() and 0x7F)
        shift -= 7
    }
    encodeByte(value.toByte() and 0x7F or 0x80.toByte())
}

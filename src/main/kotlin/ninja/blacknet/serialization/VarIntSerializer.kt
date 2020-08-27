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

import kotlinx.serialization.KSerializer
import kotlinx.serialization.SerializationException
import kotlinx.serialization.descriptors.PrimitiveKind
import kotlinx.serialization.descriptors.PrimitiveSerialDescriptor
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import kotlinx.serialization.json.JsonDecoder
import kotlinx.serialization.json.JsonEncoder
import kotlin.experimental.and
import kotlin.experimental.or
import ninja.blacknet.crypto.HashCoder
import ninja.blacknet.rpc.requests.RequestDecoder

/**
 * Serializes an [Int] with variable number of bytes in a binary representation.
 */
object VarIntSerializer : KSerializer<Int> {
    /**
     * The maximum number of bytes in a binary representation of the [Int].
     */
    const val MAX_SIZE_BYTES = 5

    override val descriptor: SerialDescriptor = PrimitiveSerialDescriptor(
        "ninja.blacknet.serialization.VarIntSerializer",
        PrimitiveKind.INT
    )

    override fun deserialize(decoder: Decoder): Int {
        return when (decoder) {
            is BinaryDecoder -> decoder.decodeVarInt()
            is JsonDecoder, is RequestDecoder -> decoder.decodeInt()
            else -> throw notSupportedFormatError(decoder, this)
        }
    }

    override fun serialize(encoder: Encoder, value: Int) {
        when (encoder) {
            is BinaryEncoder -> encoder.encodeVarInt(value)
            is HashCoder, is JsonEncoder -> encoder.encodeInt(value)
            else -> throw notSupportedFormatError(encoder, this)
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
            throw SerializationException("Too long VarInt")
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

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
import kotlinx.serialization.Serializable
import kotlinx.serialization.Serializer
import kotlinx.serialization.json.JsonInput
import kotlinx.serialization.json.JsonOutput
import kotlin.experimental.and
import kotlin.experimental.or
import ninja.blacknet.crypto.HashCoder
import ninja.blacknet.crypto.SipHash.hashCode
import ninja.blacknet.ktor.requests.RequestDecoder

/**
 * Represents an int with variable number of bytes in a binary representation.
 */
@Serializable
class VarInt(val int: Int) {
    override fun equals(other: Any?): Boolean = (other is VarInt) && int == other.int
    override fun hashCode(): Int = hashCode(serializer(), this)
    override fun toString(): String = int.toString()

    @Serializer(forClass = VarInt::class)
    companion object {
        /**
         * The maximum number of bytes in a binary representation of a [VarInt].
         */
        const val MAX_SIZE_BYTES = 5
        val ZERO = VarInt(0)

        override fun deserialize(decoder: Decoder): VarInt {
            return when (decoder) {
                is BinaryDecoder -> VarInt(decoder.decodeVarInt())
                is JsonInput, is RequestDecoder -> VarInt(decoder.decodeInt())
                else -> throw notSupportedDecoderError(decoder, this)
            }
        }

        override fun serialize(encoder: Encoder, value: VarInt) {
            when (encoder) {
                is BinaryEncoder -> encoder.encodeVarInt(value.int)
                is HashCoder, is JsonOutput -> encoder.encodeInt(value.int)
                else -> throw notSupportedEncoderError(encoder, this)
            }
        }
    }
}

/**
 * Decodes a variable length int value.
 *
 * @return the [Int] value containing the data
 */
fun Decoder.decodeVarInt(): Int {
    var c = VarInt.MAX_SIZE_BYTES + 1
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

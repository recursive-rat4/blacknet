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
import kotlinx.serialization.Serializable
import kotlinx.serialization.Serializer
import kotlinx.serialization.json.JsonInput
import kotlinx.serialization.json.JsonOutput
import kotlin.experimental.and
import kotlin.experimental.or
import ninja.blacknet.debugMessage
import ninja.blacknet.crypto.HashCoder
import ninja.blacknet.crypto.SipHash.hashCode
import ninja.blacknet.ktor.requests.RequestDecoder

/**
 * Represents a long with variable number of bytes in a binary representation.
 */
@Serializable
class VarLong(val long: Long) {
    override fun equals(other: Any?): Boolean = (other is VarLong) && long == other.long
    override fun hashCode(): Int = hashCode(serializer(), this)
    override fun toString(): String = long.toString()

    @Serializer(forClass = VarLong::class)
    companion object {
        /**
         * The maximum number of bytes in a binary representation of a [VarLong].
         */
        const val MAX_SIZE_BYTES = 10
        val ZERO = VarLong(0L)

        fun parse(string: String): VarLong = try {
                                    VarLong(string.toLong())
                                } catch(e: NumberFormatException) {
                                    throw ParserException(e.debugMessage(), e)
                                }

        override fun deserialize(decoder: Decoder): VarLong {
            return when (decoder) {
                is BinaryDecoder -> VarLong(decoder.decodeVarLong())
                is JsonInput -> VarLong.parse(decoder.decodeString())
                is RequestDecoder -> VarLong(decoder.decodeLong())
                else -> throw notSupportedDecoderError(decoder, this)
            }
        }

        override fun serialize(encoder: Encoder, value: VarLong) {
            when (encoder) {
                is BinaryEncoder -> encoder.encodeVarLong(value.long)
                is HashCoder -> encoder.encodeLong(value.long)
                is JsonOutput -> encoder.encodeString(value.toString())
                else -> throw notSupportedEncoderError(encoder, this)
            }
        }
    }
}

/**
 * Decodes a variable length long value.
 *
 * @return the [Long] value containing the data
 */
fun Decoder.decodeVarLong(): Long {
    var c = VarLong.MAX_SIZE_BYTES + 1
    var result = 0L
    var v: Byte
    do {
        if (--c != 0) {
            v = decodeByte()
            result = result shl 7 or (v and 0x7F.toByte()).toLong()
        } else {
            throw DecoderException("Too long VarLong")
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

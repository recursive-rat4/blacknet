/*
 * Copyright (c) 2018-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 *
 * decodeVarLong, encodeVarLong originally come from MapDB http://www.mapdb.org/
 * licensed under the Apache License, Version 2.0
 */

package ninja.blacknet.serialization.bbf

import kotlinx.serialization.SerializationException
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import kotlin.experimental.and
import kotlin.experimental.or

/**
 * The maximum number of bytes in the Blacknet Binary Format representation of the [Long].
 */
private const val MAX_SIZE_BYTES = 10

/**
 * Decodes a variable length long value.
 *
 * @return the [Long] value containing the data
 */
public fun Decoder.decodeVarLong(): Long {
    var c = MAX_SIZE_BYTES + 1
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
public fun Encoder.encodeVarLong(value: Long) {
    var shift = 63 - java.lang.Long.numberOfLeadingZeros(value)
    shift -= shift % 7 // round down to nearest multiple of 7
    while (shift != 0) {
        encodeByte(value.ushr(shift).toByte() and 0x7F)
        shift -= 7
    }
    encodeByte(value.toByte() and 0x7F or 0x80.toByte())
}

/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 *
 * decodeVarInt, decodeVarLong originally come from MapDB http://www.mapdb.org/
 * licensed under the Apache License, Version 2.0
 */

package ninja.blacknet.serialization

import io.ktor.utils.io.core.*
import kotlinx.serialization.DeserializationStrategy
import kotlinx.serialization.ElementValueDecoder
import kotlinx.serialization.SerialDescriptor
import kotlinx.serialization.internal.EnumDescriptor
import kotlin.experimental.and

/**
 * Decoder from the Blacknet Binary Format
 */
class BinaryDecoder(
        private val input: ByteReadPacket
) : ElementValueDecoder() {
    constructor(bytes: ByteArray) : this(ByteReadPacket(bytes))

    fun <T : Any?> decode(loader: DeserializationStrategy<T>): T {
        val value = loader.deserialize(this)
        val remaining = input.remaining
        return if (remaining == 0L) {
            value
        } else {
            input.release()
            throw RuntimeException("$remaining trailing bytes")
        }
    }

    override fun decodeByte(): Byte = input.readByte()
    override fun decodeShort(): Short = input.readShort()
    override fun decodeInt(): Int = input.readInt()
    override fun decodeLong(): Long = input.readLong()

    override fun decodeFloat(): Float = input.readFloat()
    override fun decodeDouble(): Double = input.readDouble()

    override fun decodeNotNullMark(): Boolean {
        return when (val byte = input.readByte()) {
            0.toByte() -> false
            1.toByte() -> true
            else -> throw RuntimeException("Unexpected value for NotNullMark $byte")
        }
    }
    override fun decodeBoolean(): Boolean {
        return when (val byte = input.readByte()) {
            0.toByte() -> false
            1.toByte() -> true
            else -> throw RuntimeException("Unexpected value for Boolean $byte")
        }
    }

    override fun decodeString(): String {
        val size = decodeVarInt()
        return String(input.readBytes(size))
    }

    override fun decodeCollectionSize(desc: SerialDescriptor): Int = decodeVarInt()

    fun decodeByteArray(): ByteArray {
        val size = decodeVarInt()
        return input.readBytes(size)
    }

    fun decodeFixedByteArray(size: Int): ByteArray {
        return input.readBytes(size)
    }

    fun decodeVarInt(): Int {
        var c = VARINT_MAX_SIZE + 1
        var result = 0
        var v: Byte
        do {
            if (--c != 0) {
                v = input.readByte()
                result = result shl 7 or (v and 0x7F.toByte()).toInt()
            } else {
                throw RuntimeException("Too long VarInt")
            }
        } while (v and 0x80.toByte() == 0.toByte())
        return result
    }

    fun decodeVarLong(): Long {
        var c = VARLONG_MAX_SIZE + 1
        var result = 0L
        var v: Byte
        do {
            if (--c != 0) {
                v = input.readByte()
                result = result shl 7 or (v and 0x7F.toByte()).toLong()
            } else {
                throw RuntimeException("Too long VarLong")
            }
        } while (v and 0x80.toByte() == 0.toByte())
        return result
    }

    companion object {
        const val VARINT_MAX_SIZE = 5
        const val VARLONG_MAX_SIZE = 10
    }
}

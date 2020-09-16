/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.serialization

import io.ktor.utils.io.core.*
import kotlinx.serialization.DeserializationStrategy
import kotlinx.serialization.SerializationException
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.modules.EmptySerializersModule
import kotlinx.serialization.modules.SerializersModule

/**
 * Decoder from the Blacknet Binary Format
 */
class BinaryDecoder(
        private val input: ByteReadPacket
) : AdaptorDecoder() {
    constructor(bytes: ByteArray) : this(ByteReadPacket(bytes))

    override val serializersModule: SerializersModule = EmptySerializersModule

    fun <T : Any?> decode(strategy: DeserializationStrategy<T>): T {
        val value = strategy.deserialize(this)
        val remaining = input.remaining
        return if (remaining == 0L) {
            value
        } else {
            input.release()
            throw SerializationException("$remaining trailing bytes")
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
            else -> throw SerializationException("Unexpected value for NotNullMark $byte")
        }
    }
    override fun decodeBoolean(): Boolean {
        return when (val byte = input.readByte()) {
            0.toByte() -> false
            1.toByte() -> true
            else -> throw SerializationException("Unexpected value for Boolean $byte")
        }
    }

    override fun decodeString(): String {
        val size = decodeVarInt()
        return String(input.readBytes(size))
    }

    override fun decodeCollectionSize(descriptor: SerialDescriptor): Int = decodeVarInt()

    fun decodeByteArray(): ByteArray {
        val size = decodeVarInt()
        return input.readBytes(size)
    }

    fun decodeFixedByteArray(size: Int): ByteArray {
        return input.readBytes(size)
    }
}

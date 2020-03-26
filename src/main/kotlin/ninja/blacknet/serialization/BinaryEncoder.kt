/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 *
 * encodeVarInt, encodeVarLong originally come from MapDB http://www.mapdb.org/
 * licensed under the Apache License, Version 2.0
 */

package ninja.blacknet.serialization

import io.ktor.utils.io.core.*
import kotlinx.serialization.*
import kotlinx.serialization.builtins.AbstractEncoder
import kotlin.experimental.and
import kotlin.experimental.or

/**
 * Encoder to the Blacknet Binary Format
 */
class BinaryEncoder : AbstractEncoder() {
    private val out = BytePacketBuilder()

    fun toPacket(): ByteReadPacket {
        return out.build()
    }

    fun toBytes(): ByteArray {
        return toPacket().readBytes()
    }

    override fun encodeByte(value: Byte) = out.writeByte(value)
    override fun encodeShort(value: Short) = out.writeShort(value)
    override fun encodeInt(value: Int) = out.writeInt(value)
    override fun encodeLong(value: Long) = out.writeLong(value)

    override fun encodeFloat(value: Float) = out.writeFloat(value)
    override fun encodeDouble(value: Double) = out.writeDouble(value)

    override fun encodeNull() = out.writeByte(0)
    override fun encodeNotNullMark() = out.writeByte(1)
    override fun encodeBoolean(value: Boolean) = out.writeByte(if (value) 1 else 0)

    override fun encodeString(value: String) {
        val bytes = value.toByteArray()
        encodeVarInt(bytes.size)
        out.writeFully(bytes, 0, bytes.size)
    }

    override fun beginCollection(descriptor: SerialDescriptor, collectionSize: Int, vararg typeSerializers: KSerializer<*>): CompositeEncoder {
        return super.beginCollection(descriptor, collectionSize, *typeSerializers).also {
            encodeVarInt(collectionSize)
        }
    }

    override fun endStructure(descriptor: SerialDescriptor) = Unit

    fun encodeByteArray(value: ByteArray) {
        encodeVarInt(value.size)
        out.writeFully(value, 0, value.size)
    }

    fun encodeFixedByteArray(value: ByteArray) {
        out.writeFully(value, 0, value.size)
    }

    fun encodeVarInt(value: Int) {
        var shift = 31 - Integer.numberOfLeadingZeros(value)
        shift -= shift % 7 // round down to nearest multiple of 7
        while (shift != 0) {
            out.writeByte(value.ushr(shift).toByte() and 0x7F)
            shift -= 7
        }
        out.writeByte(value.toByte() and 0x7F or 0x80.toByte())
    }

    fun encodeVarLong(value: Long) {
        var shift = 63 - java.lang.Long.numberOfLeadingZeros(value)
        shift -= shift % 7 // round down to nearest multiple of 7
        while (shift != 0) {
            out.writeByte(value.ushr(shift).toByte() and 0x7F)
            shift -= 7
        }
        out.writeByte(value.toByte() and 0x7F or 0x80.toByte())
    }

    companion object {
        fun <T : Any?> toBytes(strategy: SerializationStrategy<T>, value: T): ByteArray {
            val encoder = BinaryEncoder()
            strategy.serialize(encoder, value)
            return encoder.toBytes()
        }

        fun <T : Any?> toPacket(strategy: SerializationStrategy<T>, value: T): ByteReadPacket {
            val encoder = BinaryEncoder()
            strategy.serialize(encoder, value)
            return encoder.toPacket()
        }
    }
}

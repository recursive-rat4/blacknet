/*
 * Copyright (c) 2018-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.serialization.bbf

import io.ktor.utils.io.core.*
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.CompositeEncoder
import kotlinx.serialization.modules.EmptySerializersModule
import kotlinx.serialization.modules.SerializersModule
import ninja.blacknet.serialization.SequentialEncoder

/**
 * Encoder to the Blacknet Binary Format
 */
public class BinaryEncoder(
        internal val output: BytePacketBuilder = BytePacketBuilder(),
        override val serializersModule: SerializersModule = EmptySerializersModule
) : SequentialEncoder() {
    override fun encodeByte(value: Byte): Unit = output.writeByte(value)
    override fun encodeShort(value: Short): Unit = output.writeShort(value)
    override fun encodeInt(value: Int): Unit = output.writeInt(value)
    override fun encodeLong(value: Long): Unit = output.writeLong(value)

    override fun encodeFloat(value: Float): Unit = output.writeFloat(value)
    override fun encodeDouble(value: Double): Unit = output.writeDouble(value)

    override fun encodeNull(): Unit = output.writeByte(0)
    override fun encodeNotNullMark(): Unit = output.writeByte(1)
    override fun encodeBoolean(value: Boolean): Unit = output.writeByte(if (value) 1 else 0)

    override fun encodeString(value: String) {
        val bytes = value.toByteArray()
        encodeVarInt(bytes.size)
        output.writeFully(bytes, 0, bytes.size)
    }

    override fun beginCollection(descriptor: SerialDescriptor, collectionSize: Int): CompositeEncoder {
        return super.beginCollection(descriptor, collectionSize).also {
            encodeVarInt(collectionSize)
        }
    }

    public fun encodeByteArray(value: ByteArray) {
        encodeVarInt(value.size)
        output.writeFully(value, 0, value.size)
    }

    public fun encodeFixedByteArray(value: ByteArray) {
        output.writeFully(value, 0, value.size)
    }
}

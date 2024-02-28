/*
 * Copyright (c) 2018-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.serialization.bbf

import io.ktor.utils.io.core.BytePacketBuilder
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.CompositeEncoder
import kotlinx.serialization.modules.SerializersModule
import ninja.blacknet.serialization.SequentialEncoder

/**
 * Encoder to the Blacknet Binary Format
 */
public class BinaryEncoder(
    private val writer: BinaryWriter,
    override val serializersModule: SerializersModule,
) : SequentialEncoder() {
    public constructor(builder: BytePacketBuilder, serializersModule: SerializersModule) : this(PacketWriter(builder), serializersModule)

    override fun encodeByte(value: Byte): Unit = writer.writeByte(value)
    override fun encodeShort(value: Short): Unit = writer.writeShort(value)
    override fun encodeInt(value: Int): Unit = writer.writeInt(value)
    override fun encodeLong(value: Long): Unit = writer.writeLong(value)

    override fun encodeFloat(value: Float): Unit = writer.writeFloat(value)
    override fun encodeDouble(value: Double): Unit = writer.writeDouble(value)

    override fun encodeNull(): Unit = writer.writeByte(0)
    override fun encodeNotNullMark(): Unit = writer.writeByte(1)
    override fun encodeBoolean(value: Boolean): Unit = writer.writeByte(if (value) 1 else 0)

    override fun encodeString(value: String) {
        val bytes = value.toByteArray()
        encodeVarInt(bytes.size)
        writer.writeBytes(bytes)
    }

    override fun beginCollection(descriptor: SerialDescriptor, collectionSize: Int): CompositeEncoder {
        return super.beginCollection(descriptor, collectionSize).also {
            encodeVarInt(collectionSize)
        }
    }

    public fun encodeByteArray(value: ByteArray) {
        encodeVarInt(value.size)
        writer.writeBytes(value)
    }

    public fun encodeFixedByteArray(value: ByteArray) {
        writer.writeBytes(value)
    }
}

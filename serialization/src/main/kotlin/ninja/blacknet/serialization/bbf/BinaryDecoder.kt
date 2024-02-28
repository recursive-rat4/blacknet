/*
 * Copyright (c) 2018-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.serialization.bbf

import io.ktor.utils.io.core.ByteReadPacket
import kotlinx.serialization.SerializationException
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.modules.SerializersModule
import ninja.blacknet.serialization.SequentialDecoder

/**
 * Decoder from the Blacknet Binary Format
 */
public class BinaryDecoder(
    private val reader: BinaryReader,
    override val serializersModule: SerializersModule,
) : SequentialDecoder() {
    public constructor(bytes: ByteArray, serializersModule: SerializersModule) : this(ByteReadPacket(bytes), serializersModule)
    public constructor(packet: ByteReadPacket, serializersModule: SerializersModule) : this(PacketReader(packet), serializersModule)

    override fun decodeByte(): Byte = reader.readByte()
    override fun decodeShort(): Short = reader.readShort()
    override fun decodeInt(): Int = reader.readInt()
    override fun decodeLong(): Long = reader.readLong()

    override fun decodeFloat(): Float = reader.readFloat()
    override fun decodeDouble(): Double = reader.readDouble()

    override fun decodeNotNullMark(): Boolean {
        return when (val byte = reader.readByte()) {
            0.toByte() -> false
            1.toByte() -> true
            else -> throw SerializationException("Unexpected value for NotNullMark $byte")
        }
    }
    override fun decodeBoolean(): Boolean {
        return when (val byte = reader.readByte()) {
            0.toByte() -> false
            1.toByte() -> true
            else -> throw SerializationException("Unexpected value for Boolean $byte")
        }
    }

    override fun decodeString(): String {
        val size = decodeVarInt()
        return String(reader.readBytes(size))
    }

    override fun decodeCollectionSize(descriptor: SerialDescriptor): Int = decodeVarInt()

    public fun decodeByteArray(): ByteArray {
        val size = decodeVarInt()
        return reader.readBytes(size)
    }

    public fun decodeFixedByteArray(size: Int): ByteArray {
        return reader.readBytes(size)
    }
}

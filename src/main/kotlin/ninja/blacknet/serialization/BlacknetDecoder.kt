/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.serialization

import kotlinx.io.core.ByteReadPacket
import kotlinx.io.core.IoBuffer
import kotlinx.io.core.readBytes
import kotlinx.serialization.DeserializationStrategy
import kotlinx.serialization.ElementValueDecoder
import kotlinx.serialization.SerialDescriptor
import kotlinx.serialization.internal.EnumDescriptor
import java.nio.ByteBuffer

class BlacknetDecoder(private val input: ByteReadPacket) : ElementValueDecoder() {
    fun <T : Any?> decode(loader: DeserializationStrategy<T>): T? {
        val v = loader.deserialize(this)
        if (input.remaining > 0) {
            input.release()
            return null
        }
        return v
    }

    override fun decodeByte(): Byte = input.readByte()
    override fun decodeInt(): Int = input.readInt()
    override fun decodeLong(): Long = input.readLong()

    override fun decodeString(): String {
        val size = input.unpackInt()
        return String(input.readBytes(size))
    }

    override fun decodeEnum(enumDescription: EnumDescriptor): Int = input.unpackInt()
    override fun decodeCollectionSize(desc: SerialDescriptor): Int = input.unpackInt()

    fun decodeSerializableByteArrayValue(): SerializableByteArray {
        val size = input.unpackInt()
        return SerializableByteArray(input.readBytes(size))
    }

    fun decodeByteArrayValue(size: Int): ByteArray {
        return input.readBytes(size)
    }

    companion object {
        fun fromBytes(bytes: ByteArray): BlacknetDecoder {
            val buf = IoBuffer(ByteBuffer.wrap(bytes))
            buf.resetForRead()
            val input = ByteReadPacket(buf, IoBuffer.NoPool)
            return BlacknetDecoder(input)
        }
    }
}

private fun ByteReadPacket.unpackInt(): Int {
    var ret = 0
    var v: Byte
    do {
        v = readByte()
        ret = ret shl 7 or (v.toInt() and 0x7F)
    } while (v.toInt() and 0x80 == 0)
    return ret
}
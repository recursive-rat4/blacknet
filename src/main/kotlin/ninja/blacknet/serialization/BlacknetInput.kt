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
import kotlinx.serialization.ElementValueInput
import kotlinx.serialization.KSerialLoader
import java.nio.ByteBuffer
import kotlin.reflect.KClass

class BlacknetInput(private val input: ByteReadPacket) : ElementValueInput() {
    fun <T : Any?> deserialize(loader: KSerialLoader<T>): T? {
        val v = loader.load(this)
        if (input.remaining > 0) {
            input.release()
            return null
        }
        return v
    }

    override fun readByteValue(): Byte = input.readByte()
    override fun readIntValue(): Int = input.readInt()
    override fun readLongValue(): Long = input.readLong()

    override fun readStringValue(): String {
        val size = input.unpackInt()
        return String(input.readBytes(size))
    }

    @Suppress("UNCHECKED_CAST")
    override fun <T : Enum<T>> readEnumValue(enumClass: KClass<T>): T = enumClass.java.enumConstants[input.unpackInt()]

    fun readSerializableByteArrayValue(): SerializableByteArray {
        val size = input.unpackInt()
        return SerializableByteArray(input.readBytes(size))
    }

    fun readByteArrayValue(size: Int): ByteArray {
        return input.readBytes(size)
    }

    companion object {
        fun fromBytes(bytes: ByteArray): BlacknetInput {
            val buf = IoBuffer(ByteBuffer.wrap(bytes))
            buf.resetForRead()
            val input = ByteReadPacket(buf, IoBuffer.NoPool)
            return BlacknetInput(input)
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
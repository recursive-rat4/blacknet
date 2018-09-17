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
import kotlinx.io.core.readBytes
import kotlinx.serialization.ElementValueInput
import kotlin.reflect.KClass

class BlacknetInput(private val bytes: ByteReadPacket) : ElementValueInput() {
    override fun readByteValue(): Byte = bytes.readByte()
    override fun readIntValue(): Int = bytes.readInt()
    override fun readLongValue(): Long = bytes.readLong()

    override fun readStringValue(): String {
        val size = bytes.unpackInt()
        return String(bytes.readBytes(size))
    }

    @Suppress("UNCHECKED_CAST")
    override fun <T : Enum<T>> readEnumValue(enumClass: KClass<T>): T = enumClass.java.enumConstants[bytes.unpackInt()]

    fun readSerializableByteArrayValue(): SerializableByteArray {
        val size = bytes.unpackInt()
        return SerializableByteArray(bytes.readBytes(size))
    }

    fun readByteArrayValue(size: Int): ByteArray {
        return bytes.readBytes(size)
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
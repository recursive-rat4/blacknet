/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.serialization

import kotlinx.io.core.BytePacketBuilder
import kotlinx.io.core.ByteReadPacket
import kotlinx.serialization.CompositeEncoder
import kotlinx.serialization.ElementValueEncoder
import kotlinx.serialization.KSerializer
import kotlinx.serialization.SerialDescriptor
import kotlinx.serialization.internal.EnumDescriptor

class BlacknetEncoder : ElementValueEncoder() {
    private val out = BytePacketBuilder()

    fun build(): ByteReadPacket {
        return out.build()
    }

    override fun encodeByte(value: Byte) = out.writeByte(value)
    override fun encodeInt(value: Int) = out.writeInt(value)
    override fun encodeLong(value: Long) = out.writeLong(value)

    override fun encodeString(value: String) {
        val bytes = value.toByteArray()
        out.packInt(bytes.size)
        out.writeFully(bytes, 0, bytes.size)
    }

    override fun encodeEnum(enumDescription: EnumDescriptor, ordinal: Int) = out.packInt(ordinal)

    override fun beginCollection(desc: SerialDescriptor, collectionSize: Int, vararg typeParams: KSerializer<*>): CompositeEncoder {
        return super.beginCollection(desc, collectionSize, *typeParams).also {
            out.packInt(collectionSize)
        }
    }

    fun encodeSerializableByteArrayValue(value: SerializableByteArray) {
        out.packInt(value.size())
        out.writeFully(value.array, 0, value.size())
    }

    fun encodeByteArrayValue(value: ByteArray, size: Int) {
        out.writeFully(value, 0, size)
    }
}

private fun BytePacketBuilder.packInt(value: Int) {
    var shift = 31 - Integer.numberOfLeadingZeros(value)
    shift -= shift % 7 // round down to nearest multiple of 7
    while (shift != 0) {
        writeByte((value.ushr(shift) and 0x7F).toByte())
        shift -= 7
    }
    writeByte((value and 0x7F or 0x80).toByte())
}
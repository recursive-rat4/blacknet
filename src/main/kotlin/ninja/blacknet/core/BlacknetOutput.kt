/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

import kotlinx.io.core.BytePacketBuilder
import kotlinx.io.core.ByteReadPacket
import kotlinx.serialization.ElementValueOutput

class BlacknetOutput : ElementValueOutput() {
    private val out = BytePacketBuilder()

    fun build(): ByteReadPacket {
        return out.build()
    }

    override fun writeIntValue(value: Int) = out.writeInt(value)
    override fun writeLongValue(value: Long) = out.writeLong(value)

    override fun writeStringValue(value: String) {
        val bytes = value.toByteArray()
        out.packInt(bytes.size)
        out.writeFully(bytes, 0, bytes.size)
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
/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

import kotlinx.io.core.ByteReadPacket
import kotlinx.io.core.readBytes
import kotlinx.serialization.ElementValueInput

class BlacknetInput(private val bytes: ByteReadPacket) : ElementValueInput() {
    override fun readIntValue(): Int = bytes.readInt()
    override fun readLongValue(): Long = bytes.readLong()

    override fun readStringValue(): String {
        val size = bytes.unpackInt()
        return String(bytes.readBytes(size))
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
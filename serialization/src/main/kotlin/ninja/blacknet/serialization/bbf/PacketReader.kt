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
import io.ktor.utils.io.core.readBytes
import io.ktor.utils.io.core.readDouble
import io.ktor.utils.io.core.readFloat
import io.ktor.utils.io.core.readInt
import io.ktor.utils.io.core.readLong
import io.ktor.utils.io.core.readShort

/**
 * Reader from [ByteReadPacket] for the Blacknet Binary Format
 */
public class PacketReader(
    private val input: ByteReadPacket,
) : BinaryReader {
    override fun readByte(): Byte = input.readByte()
    override fun readShort(): Short = input.readShort()
    override fun readInt(): Int = input.readInt()
    override fun readLong(): Long = input.readLong()

    override fun readFloat(): Float = input.readFloat()
    override fun readDouble(): Double = input.readDouble()

    override fun readBytes(n: Int): ByteArray = input.readBytes(n)
}

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
import io.ktor.utils.io.core.writeDouble
import io.ktor.utils.io.core.writeFloat
import io.ktor.utils.io.core.writeFully
import io.ktor.utils.io.core.writeInt
import io.ktor.utils.io.core.writeLong
import io.ktor.utils.io.core.writeShort

/**
 * Writer to [BytePacketBuilder] for the Blacknet Binary Format
 */
public class PacketWriter(
    private val output: BytePacketBuilder,
) : BinaryWriter {
    override fun writeByte(value: Byte): Unit = output.writeByte(value)
    override fun writeShort(value: Short): Unit = output.writeShort(value)
    override fun writeInt(value: Int): Unit = output.writeInt(value)
    override fun writeLong(value: Long): Unit = output.writeLong(value)

    override fun writeFloat(value: Float): Unit = output.writeFloat(value)
    override fun writeDouble(value: Double): Unit = output.writeDouble(value)

    override fun writeBytes(value: ByteArray): Unit = output.writeFully(value)
}

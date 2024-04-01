/*
 * Copyright (c) 2018-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.serialization.bbf

import java.io.DataOutputStream
import ninja.blacknet.io.writeByte
import ninja.blacknet.io.writeShort

/**
 * Writer to [DataOutputStream] for the Blacknet Binary Format
 */
public class StreamWriter(
    private val output: DataOutputStream,
) : BinaryWriter {
    override fun writeByte(value: Byte): Unit = output.writeByte(value)
    override fun writeShort(value: Short): Unit = output.writeShort(value)
    override fun writeInt(value: Int): Unit = output.writeInt(value)
    override fun writeLong(value: Long): Unit = output.writeLong(value)

    override fun writeFloat(value: Float): Unit = output.writeFloat(value)
    override fun writeDouble(value: Double): Unit = output.writeDouble(value)

    override fun writeBytes(value: ByteArray): Unit = output.write(value)
}

/*
 * Copyright (c) 2018-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.serialization.bbf

/**
 * Writer to [Int] for the Blacknet Binary Format
 */
public class SizeWriter(
    private var output: Int = 0,
) : BinaryWriter {
    override fun writeByte(value: Byte): Unit { output += 1 }
    override fun writeShort(value: Short): Unit { output += 2 }
    override fun writeInt(value: Int): Unit { output += 4 }
    override fun writeLong(value: Long): Unit { output += 8 }

    override fun writeFloat(value: Float): Unit { output += 4 }
    override fun writeDouble(value: Double): Unit { output += 8 }

    override fun writeBytes(value: ByteArray): Unit { output += value.size }

    public fun output(): Int = output
}

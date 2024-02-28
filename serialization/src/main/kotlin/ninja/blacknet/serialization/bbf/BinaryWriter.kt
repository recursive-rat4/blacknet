/*
 * Copyright (c) 2018-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

@file:Suppress("RedundantUnitReturnType")

package ninja.blacknet.serialization.bbf

/**
 * A writer for the Blacknet Binary Format
 */
public interface BinaryWriter {
    public fun writeByte(value: Byte): Unit
    public fun writeShort(value: Short): Unit
    public fun writeInt(value: Int): Unit
    public fun writeLong(value: Long): Unit

    public fun writeFloat(value: Float): Unit
    public fun writeDouble(value: Double): Unit

    public fun writeBytes(value: ByteArray): Unit
}

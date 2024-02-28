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
 * A reader for the Blacknet Binary Format
 */
public interface BinaryReader {
    public fun readByte(): Byte
    public fun readShort(): Short
    public fun readInt(): Int
    public fun readLong(): Long

    public fun readFloat(): Float
    public fun readDouble(): Double

    public fun readBytes(n: Int): ByteArray
}

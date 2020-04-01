/*
 * Copyright (c) 2018-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

interface HashWriter {
    /**
     * Writes a byte value.
     *
     * @param value the [Byte] containing the data
     */
    fun writeByte(value: Byte)

    /**
     * Writes a byte array value.
     *
     * @param value the [ByteArray] containing the data
     */
    fun writeByteArray(value: ByteArray)

    /**
     * Writes the specified bytes of a byte array value.
     *
     * @param value the [ByteArray] containing the data
     * @param offset the offset of the data
     * @param length the length of the data
     */
    fun writeByteArray(value: ByteArray, offset: Int, length: Int)

    /**
     * Resets the writer for re-use.
     */
    fun reset()

    /**
     * Returns the computed value. The writer is reset after this operation.
     */
    fun finish(): ByteArray
}

/**
 * Encodes a byte array value.
 *
 * @param value the [ByteArray] containing the data
 */
fun HashCoder.encodeByteArray(value: ByteArray) {
    writer.writeByteArray(value)
}

/**
 * Encodes the specified bytes of a byte array value.
 *
 * @param value the [ByteArray] containing the data
 * @param offset the offset of the data
 * @param length the length of the data
 */
fun HashCoder.encodeByteArray(value: ByteArray, offset: Int, length: Int) {
    writer.writeByteArray(value, offset, length)
}

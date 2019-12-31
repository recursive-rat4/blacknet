/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.util

private val EMPTY_BYTE_ARRAY = ByteArray(0)

/**
 * Returns an empty [ByteArray]
 */
fun emptyByteArray() = EMPTY_BYTE_ARRAY

/**
 * Returns a [ByteArray] containing the specified bytes represented as [Int]s
 */
fun byteArrayOfInts(vararg ints: Int) = ByteArray(ints.size) { index -> ints[index].toByte() }

/**
 * Returns `true` if this [ByteArray] starts with the specified bytes
 */
fun ByteArray.startsWith(bytes: ByteArray): Boolean {
    for (i in bytes.indices)
        if (this[i] != bytes[i])
            return false
    return true
}

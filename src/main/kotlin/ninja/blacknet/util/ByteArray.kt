/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.util

private val HEX_CHARS = charArrayOf('0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F')
private val HEX_CHARS_LOWER = charArrayOf('0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f')
private val HEX_TABLE = byteArrayOf(-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, -1, -1, -1, -1, -1, -1, -1, 10, 11, 12, 13, 14, 15, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 10, 11, 12, 13, 14, 15)

private fun hexTable(element: Char): Int {
    val i = element.toInt()
    if (i > 0 && i < HEX_TABLE.size)
        return HEX_TABLE[i].toInt()
    return -1
}

fun ByteArray.toHex(): String {
    val result = StringBuilder(size * 2)

    for (i in 0 until size) {
        val octet = this[i].toInt()
        val firstIndex = (octet and 0xF0).ushr(4)
        val secondIndex = octet and 0x0F
        result.append(HEX_CHARS[firstIndex])
        result.append(HEX_CHARS[secondIndex])
    }

    return result.toString()
}

fun fromHex(string: String, size: Int = 0): ByteArray? {
    if (size == 0) {
        if (string.length % 2 == 1)
            return null
    } else {
        if (string.length != size * 2)
            return null
    }

    val result = ByteArray(string.length / 2)

    for (i in 0 until string.length step 2) {
        val firstIndex = hexTable(string[i])
        val secondIndex = hexTable(string[i + 1])

        if (firstIndex == -1 || secondIndex == -1)
            return null

        val octet = firstIndex.shl(4).or(secondIndex)
        result.set(i.shr(1), octet.toByte())
    }

    return result
}

fun byteArrayOfInts(vararg ints: Int) = ByteArray(ints.size) { pos -> ints[pos].toByte() }

fun ByteArray.startsWith(bytes: ByteArray): Boolean {
    for (i in bytes.indices)
        if (this[i] != bytes[i])
            return false
    return true
}

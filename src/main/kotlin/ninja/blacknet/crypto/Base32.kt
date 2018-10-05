/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

object Base32 {
    private const val CHARSET = "abcdefghijklmnopqrstuvwxyz234567"

    fun encode(bytes: ByteArray): String {
        val converted = convertBits(bytes, 8, 5, true)!!

        val builder = StringBuilder(((bytes.size + 4) / 5) * 8)
        converted.forEach {
            builder.append(CHARSET[it.toInt()])
        }
        while (builder.length % 8 != 0)
            builder.append('=')
        return builder.toString()
    }

    fun decode(string: String): ByteArray? {
        if (string.length % 8 != 0) return null

        val data = ByteArray(string.length)
        for (i in string.indices) {
            val x = DECODE[string[i].toInt()]
            if (x == (-1).toByte()) return null
            data[i] = x
        }

        //TODO padding

        return convertBits(data, 5, 8, false)
    }

    fun convertBits(data: ByteArray, fromBits: Int, toBits: Int, pad: Boolean): ByteArray? {
        var acc = 0
        var bits = 0
        val maxv = (1 shl toBits) - 1
        val ret = ArrayList<Byte>(32)

        for (value in data) {
            val b = value.toInt() and 0xff

            if (b < 0) {
                return null
            } else if (b shr fromBits > 0) {
                return null
            }

            acc = acc shl fromBits or b
            bits += fromBits
            while (bits >= toBits) {
                bits -= toBits
                ret.add((acc shr bits and maxv).toByte())
            }
        }

        if (pad && bits > 0) {
            ret.add((acc shl toBits - bits and maxv).toByte())
        } else if (bits >= fromBits || (acc shl toBits - bits and maxv).toByte().toInt() != 0) {
            return null
        }

        return ret.toByteArray()
    }

    private val DECODE = byteArrayOf(
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 26, 27, 28, 29, 30, 31, -1, -1, -1, -1,
            -1, -1, -1, -1, -1,  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14,
            15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, -1, -1, -1, -1, -1, -1,  0,  1,  2,
             3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
            23, 24, 25, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1
    )
}